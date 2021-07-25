// Copyright 2020-2021 Kevin Reid under the terms of the MIT License as detailed
// in the accompanying file README.md or <https://opensource.org/licenses/MIT>.

//! Tests for [`crate::space`].
//!
//! Note that some sub-modules have their own test modules.

use super::*;
use crate::block::AIR;
use crate::content::make_some_blocks;
use crate::listen::Sink;
use crate::math::GridPoint;
use crate::universe::{RefError, Universe, UniverseIndex as _};
use cgmath::EuclideanSpace as _;
use std::sync::Arc;

// TODO: test consistency between the index and get_* methods
// TODO: test fill() equivalence and error handling

#[test]
fn initial_state_consistency() {
    Space::empty_positive(0, 0, 0).consistency_check();
    Space::empty_positive(1, 0, 0).consistency_check();
    Space::empty_positive(1, 1, 1).consistency_check();
    Space::empty_positive(10, 20, 30).consistency_check();
    Space::empty(Grid::new([1, 2, 3], [10, 20, 30])).consistency_check();
}

/// set() returns Ok when the cube was changed or already equal.
#[test]
fn set_success() {
    let [first, second] = make_some_blocks();
    let mut space = Space::empty_positive(1, 1, 1);
    let pt = GridPoint::origin();
    assert_eq!(Ok(true), space.set(pt, &first));
    assert_eq!(&space[pt], &first);
    assert_eq!(Ok(false), space.set(pt, &first));
    assert_eq!(&space[pt], &first);
    assert_eq!(Ok(true), space.set(pt, &second));
    assert_eq!(&space[pt], &second);

    space.consistency_check(); // bonus testing
}

#[test]
fn set_failure_out_of_bounds() {
    let [block] = make_some_blocks();
    let pt = GridPoint::new(1, 0, 0);
    let ptg = Grid::single_cube(pt);
    let mut space = Space::empty_positive(1, 1, 1);
    assert_eq!(Err(SetCubeError::OutOfBounds(ptg)), space.set(pt, &block));
    assert_eq!(Err(SetCubeError::OutOfBounds(ptg)), space.set(pt, &AIR));

    space.consistency_check(); // bonus testing
}

/// This test case should also cover `RefError::Gone`.
#[test]
fn set_failure_borrow() {
    let mut u = Universe::new();
    let inner_space_ref = u
        .insert("bs".into(), Space::empty_positive(1, 1, 1))
        .unwrap();
    let block = Block::builder()
        .voxels_ref(1, inner_space_ref.clone())
        .build();
    let mut outer_space = Space::empty_positive(1, 1, 1);

    let borrow = inner_space_ref.borrow_mut();
    assert_eq!(
        Err(SetCubeError::EvalBlock(
            RefError::InUse(Arc::new("bs".into())).into()
        )),
        outer_space.set((0, 0, 0), &block)
    );
    drop(borrow);

    outer_space.consistency_check(); // bonus testing
}

#[test]
fn set_failure_too_many() {
    const N: u16 = 300_u16;
    let blocks = make_some_blocks::<{ N as usize }>();
    let mut space = Space::empty_positive(N.into(), 1, 1);
    for i in 0..N {
        match space.set([i.into(), 0, 0], &blocks[usize::from(i)]) {
            Ok(true) => {}
            Err(SetCubeError::TooManyBlocks()) => break,
            unexpected => panic!("unexpected result: {:?}", unexpected),
        }
    }
    space.consistency_check(); // bonus testing
}

#[test]
fn set_error_format() {
    assert_eq!(
        SetCubeError::OutOfBounds(Grid::single_cube(GridPoint::new(1, 2, 3))).to_string(),
        // TODO: simplify the single cube case
        "Grid(1..2, 2..3, 3..4) is out of bounds"
    );
    assert_eq!(
        SetCubeError::EvalBlock(EvalBlockError::DataRefIs(RefError::Gone(Arc::new(
            "foo".into()
        ))))
        .to_string(),
        // TODO: This message is a bit "revealing our exact data structure"...
        "block evaluation failed: block data inaccessible: object was deleted: 'foo'"
    );
    assert_eq!(
        SetCubeError::TooManyBlocks().to_string(),
        "more than 65536 block types is not yet supported"
    );
}

/// EvaluatedBlock data is updated when a new block index is allocated.
#[test]
fn set_updates_evaluated_on_added_block() {
    let [block] = make_some_blocks();
    let mut space = Space::empty_positive(2, 1, 1);
    space.set((0, 0, 0), &block).unwrap();
    // Confirm the expected indices
    assert_eq!(Some(1), space.get_block_index((0, 0, 0)));
    assert_eq!(Some(0), space.get_block_index((1, 0, 0)));
    // Confirm the data is correct
    assert_eq!(space.get_evaluated((0, 0, 0)), &block.evaluate().unwrap());
    space.consistency_check(); // bonus testing
}

/// EvaluatedBlock data is updated when a block index is reused.
#[test]
fn set_updates_evaluated_on_replaced_block() {
    let [block] = make_some_blocks();
    let mut space = Space::empty_positive(1, 1, 1);
    space.set((0, 0, 0), &block).unwrap();
    // Confirm the expected indices
    assert_eq!(Some(0), space.get_block_index((0, 0, 0)));
    // Confirm the data is correct
    assert_eq!(space.get_evaluated((0, 0, 0)), &block.evaluate().unwrap());
    space.consistency_check(); // bonus testing
}

#[test]
fn removed_blocks_are_forgotten() {
    let [block_0, block_1, block_2] = make_some_blocks();
    let mut space = Space::empty_positive(2, 1, 1);
    let pt1 = GridPoint::new(0, 0, 0);
    let pt2 = GridPoint::new(1, 0, 0);
    // TODO: This test depends on block allocation order. distinct_blocks() ought to be stable or explicitly return a HashSet or something.
    assert_eq!(space.distinct_blocks(), vec![AIR.clone()], "step 1");
    space.set(pt1, &block_0).unwrap();
    space.consistency_check();
    assert_eq!(
        space.distinct_blocks(),
        vec![AIR.clone(), block_0.clone()],
        "step 2"
    );
    space.set(pt2, &block_1).unwrap();
    space.consistency_check();
    assert_eq!(
        space.distinct_blocks(),
        vec![block_1.clone(), block_0.clone()],
        "step 3"
    );
    space.set(pt1, &block_2).unwrap();
    space.consistency_check();
    assert_eq!(
        space.distinct_blocks(),
        vec![block_1.clone(), block_2.clone()],
        "step 4"
    );

    // Make sure that reinserting an old block correctly allocates an index rather than using the old one.
    space.set(pt2, &block_0).unwrap();
    space.consistency_check();
    assert_eq!(
        space.distinct_blocks(),
        vec![block_0.clone(), block_2.clone()],
        "step 4"
    );
}

#[test]
fn change_listener() {
    let [block] = make_some_blocks();
    let mut space = Space::empty_positive(2, 1, 1);
    let mut sink = Sink::new();
    space.listen(sink.listener());

    assert_eq!(Ok(true), space.set((0, 0, 0), &block));
    //panic!("{:?}", sink.collect::<Vec<_>>());
    // Note: Sink currently reports things in reverse of insertion order.
    assert_eq!(
        Some(SpaceChange::Block(GridPoint::new(0, 0, 0))),
        sink.next()
    );
    assert_eq!(
        Some(SpaceChange::Lighting(GridPoint::new(0, 0, 0))),
        sink.next()
    );
    assert_eq!(Some(SpaceChange::Number(1)), sink.next());
    assert_eq!(None, sink.next());

    // No change, no notification
    assert_eq!(Ok(false), space.set((0, 0, 0), &block));
    assert_eq!(None, sink.next());
}

#[test]
fn extract_out_of_bounds() {
    let [block_0, block_1] = make_some_blocks();
    let mut space = Space::empty_positive(2, 1, 1);
    space.set((0, 0, 0), &block_0).unwrap();
    space.set((1, 0, 0), &block_1).unwrap();

    let extract_grid = Grid::new((1, 0, 0), (1, 2, 1));
    let extracted = space.extract(extract_grid, |_index, block_data, _lighting| {
        // TODO: arrange to sanity check index and lighting
        let block = block_data.block().clone();
        assert_eq!(block.evaluate().unwrap(), block_data.evaluated);
        block
    });

    assert_eq!(extracted.grid(), extract_grid);
    assert_eq!(&extracted[(1, 0, 0)], &block_1);
    assert_eq!(&extracted[(1, 1, 0)], &AIR);
}

#[test]
fn fill_out_of_bounds() {
    let mut space = Space::empty_positive(2, 1, 1);
    let fill_grid = Grid::new((1, 0, 0), (1, 2, 1));
    let result = space.fill(fill_grid, |_| None::<Block>);
    assert_eq!(result, Err(SetCubeError::OutOfBounds(fill_grid)));
}

/// Test filling an entire space with one block using [`Space::fill`].
#[test]
fn fill_entire_space() {
    let [block] = make_some_blocks();
    let grid = Grid::new((0, 3, 0), (25 * 16, 16, 2));
    let mut space = Space::empty(grid);
    space.fill(grid, |_| Some(&block)).unwrap();
    space.consistency_check();
    for cube in grid.interior_iter() {
        assert_eq!(&space[cube], &block);
    }
}

/// Test filling an entire space with one block using [`Space::fill_uniform`].
#[test]
fn fill_uniform_entire_space() {
    let [block] = make_some_blocks();
    let grid = Grid::new((0, 3, 0), (25 * 16, 16, 2));
    let mut space = Space::empty(grid);
    let mut sink = Sink::new();
    space.listen(sink.listener());

    space.fill_uniform(grid, &block).unwrap();

    assert_eq!(sink.next(), Some(SpaceChange::EveryBlock));
    assert_eq!(sink.next(), None);
    space.consistency_check();
    for cube in grid.interior_iter() {
        assert_eq!(&space[cube], &block);
    }
}

/// There was a bug triggered when the last instance of a block was replaced with
/// a block already in the space. This specifically runs a consistency check in that
/// case.
#[test]
fn replace_last_block_regression() {
    let [block] = make_some_blocks();
    let grid = Grid::new([0, 0, 0], [3, 1, 1]);
    let mut space = Space::empty(grid);
    for i in 0..3 {
        space.set([i, 0, 0], &block).unwrap();
        space.consistency_check();
    }
}

#[test]
fn listens_to_block_changes() {
    // Set up indirect block
    let mut universe = Universe::new();
    let block_def_ref = universe.insert_anonymous(BlockDef::new(Block::from(Rgba::WHITE)));
    let indirect = Block::Indirect(block_def_ref.clone());

    // Set up space and listener
    let mut space = Space::empty_positive(1, 1, 1);
    space.set((0, 0, 0), indirect).unwrap();
    let mut sink = Sink::new();
    space.listen(sink.listener());
    assert_eq!(None, sink.next());

    // Now mutate the block def .
    let new_block = Block::from(Rgba::BLACK);
    let new_evaluated = new_block.evaluate().unwrap();
    *(block_def_ref.borrow_mut().modify()) = new_block;
    // This does not result in an outgoing notification, because we don't want
    // computations like reevaluation to happen during the notification process.
    assert_eq!(sink.next(), None);
    // Instead, it only happens the next time the space is stepped.
    let (_, _) = space.step(None, Tick::arbitrary());
    // Now we should see a notification and the evaluated block data having changed.
    assert_eq!(sink.next(), Some(SpaceChange::BlockValue(0)));
    assert_eq!(space.get_evaluated((0, 0, 0)), &new_evaluated);
}

#[test]
fn space_debug() {
    let mut space = Space::empty_positive(1, 1, 1);
    space.set_physics(SpacePhysics {
        light: LightPhysics::None,
        ..SpacePhysics::default()
    });
    println!("{:#?}", space);
    assert_eq!(
        format!("{:#?}", space),
        "Space {\n\
        \x20   grid: Grid(\n\
        \x20       0..1,\n\
        \x20       0..1,\n\
        \x20       0..1,\n\
        \x20   ),\n\
        \x20   block_data: [\n\
        \x20       SpaceBlockData {\n\
        \x20           count: 1,\n\
        \x20           block: Atom(\n\
        \x20               BlockAttributes {\n\
        \x20                   display_name: \"<air>\",\n\
        \x20                   selectable: false,\n\
        \x20                   collision: None,\n\
        \x20               },\n\
        \x20               Rgba(0.0, 0.0, 0.0, 0.0),\n\
        \x20           ),\n\
        \x20           ..\n\
        \x20       },\n\
        \x20   ],\n\
        \x20   physics: SpacePhysics {\n\
        \x20       gravity: (+0.000, -20.000, +0.000),\n\
        \x20       sky_color: Rgb(0.79, 0.79, 1.0),\n\
        \x20       light: None,\n\
        \x20   },\n\
        \x20   behaviors: BehaviorSet([]),\n\
        \x20   ..\n\
        }"
    );
}

#[test]
fn set_physics_light_none() {
    let mut space = Space::empty_positive(1, 1, 1);
    space.set([0, 0, 0], Rgba::new(1.0, 1.0, 1.0, 0.5)).unwrap();
    assert_eq!(space.light_update_queue.len(), 1);
    // Check that a no-op update doesn't clear
    space.set_physics(SpacePhysics::default());
    assert_eq!(space.light_update_queue.len(), 1);

    space.set_physics(SpacePhysics {
        light: LightPhysics::None,
        ..SpacePhysics::default()
    });

    // No light data and no queue
    assert_eq!(space.light_update_queue.len(), 0);
    assert_eq!(space.lighting.len(), 0);
    // TODO: test what change notifications are sent
}

#[test]
fn set_physics_light_rays() {
    let mut space = Space::empty_positive(2, 1, 1);
    space.set([0, 0, 0], Rgba::new(1.0, 1.0, 1.0, 0.5)).unwrap();
    space.set([1, 0, 0], Rgba::new(1.0, 1.0, 1.0, 1.0)).unwrap();
    space.set_physics(SpacePhysics {
        light: LightPhysics::None,
        ..SpacePhysics::default()
    });
    assert_eq!(space.light_update_queue.len(), 0);

    // This is the set_physics we're actually testing
    space.set_physics(SpacePhysics {
        light: LightPhysics::Rays {
            maximum_distance: 10,
        },
        ..SpacePhysics::default()
    });

    assert_eq!(space.lighting.len(), 2);
    assert_eq!(space.get_lighting([0, 0, 0]), space.packed_sky_color);
    assert_eq!(space.get_lighting([1, 0, 0]), PackedLight::OPAQUE);
    assert_eq!(space.light_update_queue.len(), 1);
    // TODO: test what change notifications are sent
}