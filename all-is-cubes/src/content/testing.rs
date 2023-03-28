use rand::{Rng as _, SeedableRng as _};
use rand_xoshiro::Xoshiro256Plus;
use crate::block::{Block, AIR};
use crate::character::Spawn;
use crate::content::free_editing_starter_inventory;
use crate::linking::InGenError;
use crate::math::{Face6, FaceMap, GridAab, Rgb};
use crate::space::{LightPhysics, Space, SpacePhysics};
use crate::universe::Universe;
/// Test space for the `lighting_bench` benchmark.
///
/// TODO: Once we have the ability to write save files, give the benchmark code an option
/// to do that instead, so this can just live in the benchmark instead of indirect.
#[doc(hidden)]
pub fn lighting_bench_space(_universe: &mut Universe) -> Result<Space, InGenError> {
    loop {}
}
