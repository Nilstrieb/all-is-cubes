// Copyright 2020-2022 Kevin Reid under the terms of the MIT License as detailed
// in the accompanying file README.md or <https://opensource.org/licenses/MIT>.

use std::collections::{btree_map, BTreeMap};
use std::fs;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use all_is_cubes::util::{CustomFormat as _, StatusText};
use async_fn_traits::{AsyncFn0, AsyncFn1, AsyncFn2};
use futures_core::future::BoxFuture;
use tokio::task::JoinHandle;

use crate::{
    results_json_path, write_report_file, ComparisonRecord, HeadlessRenderer, Overlays,
    RendererFactory, RendererId, Scene, TestCaseOutput, TestCombo, TestId,
};

type BoxedTest = Box<dyn Fn(RenderTestContext) -> BoxFuture<'static, ()> + Send + Sync>;

/// Information passed to each run of each test.
///
/// Note: This type must not have a lifetime or the test functions will be very awkward.
#[derive(Debug)]
pub struct RenderTestContext {
    test_id: TestId,
    renderer_factory: Box<dyn RendererFactory>,
    comparison_log: Arc<Mutex<Vec<ComparisonRecord>>>,
}

impl RenderTestContext {
    pub fn id(&self) -> TestId {
        self.test_id.clone()
    }

    pub fn renderer(&self, scene: impl Scene) -> Box<dyn HeadlessRenderer + Send> {
        self.renderer_factory
            .renderer_from_cameras(scene.into_cameras())
    }

    pub async fn render_comparison_test(&self, scene: impl Scene, overlays: Overlays<'_>) {
        let combo = TestCombo {
            test_id: self.id(),
            renderer: self.renderer_factory.id(),
        };

        let image = self.renderer(scene).render(overlays).await;
        let outcome = crate::compare_rendered_image(combo, image).await;

        self.comparison_log.lock().unwrap().push(outcome.clone());

        outcome.panic_if_unsuccessful(); // TODO: have a better failure result?
    }
}

/// Given a function which generates the tests, run all tests or the subset requested.
/// Returns success if all of the tests that were run passed.
///
/// `factory_factory` is a function to be called once per each test which returns a
/// [`RendererFactory`] for the type of renderer under test, which should be as isolated
/// as is reasonable for testing.
pub async fn harness_main<Factory, Ff>(
    renderer_id: RendererId,
    test_suite: fn(&mut TestCaseCollector<'_>),
    factory_factory: Ff, // TODO: better name
) -> Result<(), ()>
where
    Factory: RendererFactory + 'static,
    Ff: AsyncFn0<Output = Factory> + Send + Sync + 'static,
    Ff::OutputFuture: Send,
{
    // Gather tests (don't run them yet).
    let mut test_table: BTreeMap<String, BoxedTest> = BTreeMap::new();
    test_suite(&mut TestCaseCollector(&mut test_table));

    // Start the tests, in parallel.
    // TODO: When we have more tests we might benefit from concurrency limits.
    let suite_start_time = Instant::now();
    let test_handles: BTreeMap<String, RunningTest> = test_table
        .into_iter()
        .map(|(name, test_function)| {
            let comparison_log: Arc<Mutex<Vec<ComparisonRecord>>> = Default::default();

            let handle = {
                let test_id = name.clone();
                let comparison_log = comparison_log.clone();
                let factory_future = factory_factory();
                tokio::spawn(async move {
                    let context = RenderTestContext {
                        test_id,
                        renderer_factory: Box::new(factory_future.await),
                        comparison_log: comparison_log.clone(),
                    };

                    let case_start_time = Instant::now();
                    test_function(context).await;
                    case_start_time.elapsed()
                })
            };

            (
                name,
                RunningTest {
                    comparison_log,
                    handle,
                },
            )
        })
        .collect();

    let mut logging = io::stderr();
    let mut per_test_output = BTreeMap::new();
    let mut count_passed = 0;
    let mut count_failed = 0;
    let mut cumulative_time = Duration::ZERO;

    // Collect results.
    writeln!(logging).unwrap();
    for (name, running_test) in test_handles {
        write!(logging, "test {name:20} ...").unwrap();
        logging.flush().unwrap();

        let passed = match running_test.handle.await {
            Ok(case_time) => {
                writeln!(logging, " ok in {}", case_time.custom_format(StatusText)).unwrap();
                count_passed += 1;
                cumulative_time += case_time;
                true
            }
            Err(e) => {
                if e.is_panic() {
                    writeln!(logging, " panicked").unwrap();
                } else {
                    writeln!(logging, " unknown outcome {e:?}").unwrap();
                }
                count_failed += 1;
                false
            }
        };

        per_test_output.insert(
            name.clone(),
            TestCaseOutput {
                passed,
                test_id: name,
                comparisons: Arc::try_unwrap(running_test.comparison_log)
                    .expect("somebody hung onto the log")
                    .into_inner()
                    .unwrap(),
            },
        );
    }

    // format is imitating the standard test harness
    writeln!(
        logging,
        "\ntest result: {count_passed} passed; {count_failed} failed; \
            finished in {wall_time:.2} s ({cumulative_time:.2} s summed)",
        wall_time = suite_start_time.elapsed().as_secs_f64(),
        cumulative_time = cumulative_time.as_secs_f64()
    )
    .unwrap();

    // Write data from this run to a JSON file.
    fs::write(
        results_json_path(renderer_id),
        serde_json::to_string(&per_test_output).unwrap().as_bytes(),
    )
    .unwrap();

    // Compile this run *and* others into a report file.
    let report_path = write_report_file();
    writeln!(logging, "report written to {p}", p = report_path.display()).unwrap();

    if count_failed == 0 {
        Ok(())
    } else {
        Err(())
    }
}

#[allow(missing_debug_implementations)]
pub struct TestCaseCollector<'a>(&'a mut BTreeMap<String, BoxedTest>);

impl<'a> TestCaseCollector<'a> {
    #[track_caller]
    pub fn insert<F>(&mut self, name: &str, test_function: F)
    where
        F: AsyncFn1<RenderTestContext, Output = ()> + Send + Sync + Clone + 'static,
        F::OutputFuture: Send,
    {
        match self.0.entry(name.to_owned()) {
            btree_map::Entry::Vacant(e) => {
                let boxed_test_function = Box::new(move |context| {
                    let test_function = test_function.clone(); // usually a fn pointer
                    let boxed_future: BoxFuture<'static, ()> = Box::pin(async move {
                        test_function(context).await;
                    });
                    boxed_future
                });
                e.insert(boxed_test_function);
            }
            btree_map::Entry::Occupied(_) => {
                panic!("Duplicate test name {name:?}")
            }
        }
    }

    /// Generate an independent test case for each item of `values`.
    /// The items must serialize to strings.
    pub fn insert_variants<I, F>(&mut self, name: &str, test_function: F, values: I)
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: serde::Serialize + Clone + Send + Sync + 'static,
        F: AsyncFn2<RenderTestContext, <I as IntoIterator>::Item, Output = ()>
            + Send
            + Sync
            + Clone
            + 'static,
        F::OutputFuture: Send,
    {
        for variant_value in values {
            let test_function = test_function.clone();
            let variant_serialized = serde_json::to_value(&variant_value).unwrap();
            let variant_string = variant_serialized
                .as_str()
                .expect("Variant didn't serialize to a string");
            self.insert(&format!("{name}-{variant_string}"), move |context| {
                let variant_value = variant_value.clone();
                test_function(context, variant_value)
            });
        }
    }
}

/// Handle to a test that might be still running.
struct RunningTest {
    handle: JoinHandle<Duration>,
    comparison_log: Arc<Mutex<Vec<ComparisonRecord>>>,
}
