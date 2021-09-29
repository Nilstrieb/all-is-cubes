// Copyright 2020-2021 Kevin Reid under the terms of the MIT License as detailed
// in the accompanying file README.md or <https://opensource.org/licenses/MIT>.

//! Text-based raytracing output.

use std::borrow::Cow;

use cgmath::{Matrix4, Vector2, Vector3};

use crate::camera::{eye_for_look_at, Camera, GraphicsOptions, Viewport};
use crate::math::{FreeCoordinate, Rgba};
use crate::raytracer::{PixelBuf, RaytraceInfo, SpaceRaytracer};
use crate::space::{Space, SpaceBlockData};

/// Implements [`PixelBuf`] for text output: captures the first characters of block names
/// rather than colors.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CharacterBuf {
    /// Text to draw, if determined yet.
    hit_text: Option<String>,
}

impl PixelBuf for CharacterBuf {
    type Pixel = String;
    type BlockData = Cow<'static, str>;

    fn compute_block_data(s: &SpaceBlockData) -> Self::BlockData {
        // TODO: For more Unicode correctness, index by grapheme cluster...
        // ...and do something clever about double-width characters.
        s.evaluated()
            .attributes
            .display_name
            .chars()
            .next()
            .map(|c| Cow::Owned(c.to_string()))
            .unwrap_or(Cow::Borrowed(" "))
    }

    fn error_block_data() -> Self::BlockData {
        Cow::Borrowed("X")
    }

    fn sky_block_data() -> Self::BlockData {
        Cow::Borrowed(" ")
    }

    #[inline]
    fn opaque(&self) -> bool {
        self.hit_text.is_some()
    }

    #[inline]
    fn result(self) -> String {
        self.hit_text.unwrap_or_else(|| ".".to_owned())
    }

    #[inline]
    fn add(&mut self, _surface_color: Rgba, text: &Self::BlockData) {
        if self.hit_text.is_none() {
            self.hit_text = Some(text.to_owned().to_string());
        }
    }

    fn hit_nothing(&mut self) {
        self.hit_text = Some(".".to_owned());
    }
}

/// Print an image of the given space as “ASCII art”.
///
/// Intended for use in tests, to visualize the results in case of failure.
/// Accordingly, it always writes to the same destination as [`print!`] (which is
/// redirected when tests are run).
///
/// `direction` specifies the direction from which the camera will be looking towards
/// the center of the space. The text output will be 80 columns wide.
pub fn print_space(space: &Space, direction: impl Into<Vector3<FreeCoordinate>>) {
    print_space_impl(space, direction, |s| {
        print!("{}", s);
    });
}

/// Version of `print_space` that takes a destination, for testing.
fn print_space_impl<F: FnMut(&str)>(
    space: &Space,
    direction: impl Into<Vector3<FreeCoordinate>>,
    mut write: F,
) -> RaytraceInfo {
    // TODO: optimize height (and thus aspect ratio) for the shape of the space
    let mut camera = Camera::new(
        GraphicsOptions::default(),
        Viewport {
            nominal_size: Vector2::new(40., 40.),
            framebuffer_size: Vector2::new(80, 40),
        },
    );
    camera.set_view_matrix(Matrix4::look_at_rh(
        eye_for_look_at(space.grid(), direction.into()),
        space.grid().center(),
        Vector3::new(0., 1., 0.),
    ));

    SpaceRaytracer::<CharacterBuf>::new(space, GraphicsOptions::default())
        .trace_scene_to_text(&camera, "\n", move |s| {
            write(s);
            let r: Result<(), ()> = Ok(());
            r
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{Block, Resolution};
    use crate::content::make_some_blocks;
    use crate::universe::Universe;

    #[test]
    fn print_space_test() {
        let mut space = Space::empty_positive(3, 1, 1);
        let [b0, b1, b2] = make_some_blocks();
        space.set((0, 0, 0), &b0).unwrap();
        space.set((1, 0, 0), &b1).unwrap();
        space.set((2, 0, 0), &b2).unwrap();

        let mut output = String::new();
        print_space_impl(&space, (1., 1., 1.), |s| output += s);
        print!("{}", output);
        assert_eq!(
            output,
            "\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ...........................0000000000...........................................\n\
            .......................0000000000000001111......................................\n\
            ........................000000001111111111111111................................\n\
            ........................00000011111111111111111112222...........................\n\
            .........................0000011111111111111122222222222222.....................\n\
            .........................000001111111111112222222222222222222222................\n\
            ...........................000011111111122222222222222222222222222..............\n\
            .............................001111111112222222222222222222222222...............\n\
            ...............................111111111222222222222222222222222................\n\
            ..................................11111122222222222222222222222.................\n\
            ....................................11112222222222222222222222..................\n\
            .......................................1222222222222222222222...................\n\
            .........................................2222222222222222222....................\n\
            ............................................222222222222222.....................\n\
            ..............................................22222222222.......................\n\
            ................................................22222222........................\n\
            ...................................................2222.........................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
            ................................................................................\n\
        "
        );
    }

    /// Check that blocks with small spaces are handled without out-of-bounds errors
    #[test]
    fn partial_voxels() {
        let resolution = 4;
        let mut universe = Universe::new();
        let mut block_space = Space::empty_positive(4, 2, 4);
        block_space
            .fill_uniform(block_space.grid(), Block::from(Rgba::WHITE))
            .unwrap();
        let space_ref = universe.insert_anonymous(block_space);
        let partial_block = Block::builder()
            .voxels_ref(resolution as Resolution, space_ref.clone())
            .display_name("P")
            .build();

        let mut space = Space::empty_positive(2, 1, 1);
        let [b0] = make_some_blocks();
        space.set([0, 0, 0], &b0).unwrap();
        space.set([1, 0, 0], &partial_block).unwrap();

        let mut output = String::new();
        print_space_impl(&space, (1., 1., 1.), |s| output += s);
        print!("{}", output);
        assert_eq!(
            output,
            "\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ...............................000000...........................................\n\
                ......................0000000000000000000000....................................\n\
                ...................0000000000000000000000000000    .............................\n\
                ....................000000000000000000000000000           ......................\n\
                ....................000000000000000000000000000                  ...............\n\
                .....................0000000000000000000000000                       ...........\n\
                ......................00000000000000000000000PP                      ...........\n\
                ......................00000000000000000000PPPPPPPPPP                ............\n\
                .......................000000000000000PPPPPPPPPPPPPPPPPP           .............\n\
                .......................000000000000PPPPPPPPPPPPPPPPPPPPPPPPPP     ..............\n\
                .........................0000000PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP...............\n\
                ............................0000PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP................\n\
                ..............................00PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP.................\n\
                ................................0PPPPPPPPPPPPPPPPPPPPPPPPPPPPP..................\n\
                ..................................PPPPPPPPPPPPPPPPPPPPPPPPPPP...................\n\
                ....................................PPPPPPPPPPPPPPPPPPPPPPP.....................\n\
                ......................................PPPPPPPPPPPPPPPPPPPP......................\n\
                ........................................PPPPPPPPPPPPPPPPP.......................\n\
                ..........................................PPPPPPPPPPPPP.........................\n\
                ............................................PPPPPPPPPP..........................\n\
                ..............................................PPPPPP............................\n\
                ................................................PPP.............................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
                ................................................................................\n\
            "
        );
    }
}
