use all_is_cubes::content::make_some_voxel_blocks;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use all_is_cubes::block::{Block, Resolution::R16, AIR};
use all_is_cubes::camera::GraphicsOptions;
use all_is_cubes::math::{GridAab, Rgba};
use all_is_cubes::mesh::{
    block_meshes_for_space, BlockMesh, BlockMeshes, BlockVertex, MeshOptions, SpaceMesh,
    TestTextureAllocator, TestTextureTile, TtPoint,
};
use all_is_cubes::rgba_const;
use all_is_cubes::space::Space;
use all_is_cubes::universe::Universe;

criterion_group!(
    benches,
    block_mesh_benches,
    space_mesh_benches,
    slow_mesh_benches
);
criterion_main!(benches);

fn block_mesh_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group("block");
    let options = &MeshOptions::new(&GraphicsOptions::default());

    g.bench_function("checker-new", |b| {
        let mut universe = Universe::new();
        let block = checkerboard_block(&mut universe, [AIR, Block::from(Rgba::WHITE)]);
        let ev = block.evaluate().unwrap();

        b.iter_batched_ref(
            || (),
            |()| {
                BlockMesh::<BlockVertex<TtPoint>, _>::new(
                    &ev,
                    &TestTextureAllocator::new(),
                    options,
                )
            },
            BatchSize::SmallInput,
        );
    });

    g.bench_function("checker-reused", |b| {
        let mut universe = Universe::new();
        let block = checkerboard_block(&mut universe, [AIR, Block::from(Rgba::WHITE)]);
        let ev = block.evaluate().unwrap();

        let mut shared_mesh = BlockMesh::<BlockVertex<TtPoint>, _>::default();

        b.iter_batched_ref(
            || (),
            |()| shared_mesh.compute(&ev, &TestTextureAllocator::new(), options),
            BatchSize::SmallInput,
        );
    });

    g.bench_function("opaque", |b| {
        let mut universe = Universe::new();
        let block = checkerboard_block(
            &mut universe,
            [Block::from(Rgba::BLACK), Block::from(Rgba::WHITE)],
        );
        let ev = block.evaluate().unwrap();

        b.iter_batched_ref(
            || (),
            |()| {
                BlockMesh::<BlockVertex<TtPoint>, _>::new(
                    &ev,
                    &TestTextureAllocator::new(),
                    options,
                )
            },
            BatchSize::SmallInput,
        );
    });

    g.bench_function("msvb", |b| {
        let mut universe = Universe::new();
        let [block] = make_some_voxel_blocks(&mut universe);
        let ev = block.evaluate().unwrap();

        b.iter_batched_ref(
            || (),
            |()| {
                BlockMesh::<BlockVertex<TtPoint>, _>::new(
                    &ev,
                    &TestTextureAllocator::new(),
                    options,
                )
            },
            BatchSize::SmallInput,
        );
    });

    // TODO: Add meshing a block that has a complex but not worst-case shape.
}

fn space_mesh_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group("space");
    let options = &MeshOptions::new(&GraphicsOptions::default());

    g.bench_function("checker-new", |b| {
        // The Space and block meshes are not mutated, so we can construct them once.
        // This also ensures we're not timing dropping them.
        let (space, block_meshes) = checkerboard_space_bench_setup(options, false);
        // This benchmark actually has no use for iter_batched_ref -- it could be
        // iter_with_large_drop -- but I want to make sure any quirks of the measurement
        // are shared between all cases.
        b.iter_batched_ref(
            || (),
            |()| SpaceMesh::new(&space, space.bounds(), options, &*block_meshes),
            BatchSize::SmallInput,
        );
    });

    g.bench_function("checker-reused", |b| {
        let (space, block_meshes) = checkerboard_space_bench_setup(options, false);
        b.iter_batched_ref(
            || {
                let mut buffer = SpaceMesh::default();
                buffer.compute(&space, space.bounds(), options, &*block_meshes);
                // Sanity check that we're actually rendering as much as we expect.
                assert_eq!(buffer.vertices().len(), 6 * 4 * (16 * 16 * 16) / 2);
                buffer
            },
            |buffer: &mut SpaceMesh<BlockVertex<TtPoint>, TestTextureTile>| {
                // As of this writing, this benchmark is really just "what if we don't allocate
                // a new Vec". Later, the buffers will hopefully become cleverer and we'll be
                // able to reuse some work (or at least send only part of the buffer to the GPU),
                // and so this will become a meaningful benchmark of how much CPU time we're
                // spending or saving on that.
                buffer.compute(&space, space.bounds(), options, &*block_meshes)
            },
            BatchSize::SmallInput,
        );
    });
}

fn slow_mesh_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group("slow");
    g.sample_size(10);
    let options = &MeshOptions::new(&GraphicsOptions::default());

    g.bench_function("transparent", |b| {
        let (space, block_meshes) = checkerboard_space_bench_setup(options, true);
        b.iter_batched_ref(
            || (),
            |()| SpaceMesh::new(&space, space.bounds(), options, &*block_meshes),
            BatchSize::SmallInput,
        );
    });
}

fn checkerboard_space_bench_setup(
    options: &MeshOptions,
    transparent: bool,
) -> (Space, BlockMeshes<BlockVertex<TtPoint>, TestTextureTile>) {
    let space = checkerboard_space([
        AIR,
        Block::from(if transparent {
            rgba_const!(0.5, 0.5, 0.5, 0.5)
        } else {
            Rgba::WHITE
        }),
    ]);

    let block_meshes = block_meshes_for_space(&space, &TestTextureAllocator::new(), options);

    (space, block_meshes)
}

fn checkerboard_block(universe: &mut Universe, voxels: [Block; 2]) -> Block {
    Block::builder()
        .voxels_ref(R16, universe.insert_anonymous(checkerboard_space(voxels)))
        .build()
}

fn checkerboard_space(blocks: [Block; 2]) -> Space {
    let bounds = GridAab::from_lower_size([0, 0, 0], [16, 16, 16]);
    let mut space = Space::empty(bounds);
    space
        .fill(bounds, |p| {
            Some(&blocks[((p.x + p.y + p.z) as usize).rem_euclid(blocks.len())])
        })
        .unwrap();
    space
}
