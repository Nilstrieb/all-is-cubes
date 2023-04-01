use ordered_float::NotNan;
use crate::math::{FreeCoordinate, Rgb, Rgba};
/// Options for controlling rendering (not affecting gameplay except informationally).
///
/// Some options may be ignored by some renderers, such as when they request a particular
/// implementation approach or debug visualization. Renderers should make an effort to
/// report such failings via [`Flaws`](crate::camera::Flaws).
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[non_exhaustive]
pub(crate) struct GraphicsOptions {
    /// Whether and how to draw fog obscuring the view distance limit.
    ///
    /// TODO: Implement fog in raytracer.
    pub(crate) fog: FogOption,
    /// Field of view, in degrees from top to bottom edge of the viewport.
    pub(crate) fov_y: NotNan<FreeCoordinate>,
    /// Method to use to remap colors to fit within the displayable range.
    pub(crate) tone_mapping: ToneMappingOperator,
    /// “Camera exposure” value: a scaling factor from scene luminance to displayed
    /// luminance. Note that the exact interpretation of this depends on the chosen
    /// [`tone_mapping`](ToneMappingOperator).
    pub(crate) exposure: ExposureOption,
    /// Proportion of bloom (blurred image) to mix into the original image.
    /// 0.0 is no bloom and 1.0 is no original image.
    pub(crate) bloom_intensity: NotNan<f32>,
    /// Distance, in unit cubes, from the camera to the farthest visible point.
    ///
    /// TODO: Implement view distance limit (and fog) in raytracer.
    pub(crate) view_distance: NotNan<FreeCoordinate>,
    /// Style in which to draw the lighting of [`Space`](crate::space::Space)s.
    /// This does not affect the *computation* of lighting.
    pub(crate) lighting_display: LightingOption,
    /// Method/fidelity to use for transparency.
    pub(crate) transparency: TransparencyOption,
    /// Whether to show the HUD or other UI elements.
    ///
    /// This does not affect UI state or clickability; it purely controls display.
    /// It is intended for the purpose of asking a [renderer] to produce an image
    /// of the scene without any UI.
    ///
    /// The cursor is not currently considered part of the UI. This may be revisited
    /// later. The “info text” is controlled separately by
    /// [`debug_info_text`](Self::debug_info_text).
    ///
    /// [renderer]: crate::camera::HeadlessRenderer
    pub(crate) show_ui: bool,
    /// Whether to apply antialiasing techniques.
    pub(crate) antialiasing: AntialiasingOption,
    /// Whether to use frustum culling for drawing only in-view chunks and objects.
    ///
    /// This option is for debugging and performance testing and should not have any
    /// visible effects.
    pub(crate) use_frustum_culling: bool,
    /// Draw text overlay showing debug information.
    pub(crate) debug_info_text: bool,
    /// Draw boxes around [`Behavior`]s attached to parts of [`Space`]s.
    /// This may also eventually include further in-world diagnostic information.
    ///
    /// [`Behavior`]: crate::behavior::Behavior
    /// [`Space`]: crate::space::Space
    pub(crate) debug_behaviors: bool,
    /// Draw boxes around chunk borders and some debug info.
    pub(crate) debug_chunk_boxes: bool,
    /// Draw collision boxes for some objects.
    pub(crate) debug_collision_boxes: bool,
    /// Draw the light rays that contribute to the selected block.
    pub(crate) debug_light_rays_at_cursor: bool,
}
impl Default for GraphicsOptions {
    /// Default graphics options broadly have “everything reasonable” turned on
    /// (they may disable things that are not well-implemented yet).
    ///
    /// TODO: Explain exactly what the default is.
    fn default() -> Self {
        loop {}
    }
}
/// Choices for [`GraphicsOptions::fog`].
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub(crate) enum FogOption {
    /// No fog: objects will maintain their color and disappear raggedly.
    None,
    /// Fog starts just before the view distance ends.
    Abrupt,
    /// Compromise between `Abrupt` and `Physical` options.
    Compromise,
    /// Almost physically realistic fog of constant density.
    Physical,
}
/// Choices for [`GraphicsOptions::tone_mapping`].
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub(crate) enum ToneMappingOperator {
    /// Limit values above the maximum (or below zero) to lie within that range.
    ///
    /// This is the trivial tone mapping operation, and most “correct” for
    /// colors which do lie within the range, but will cause overly bright colors
    /// to change hue (as the RGB components are clamped independently).
    Clamp,
    /// TODO: As currently implemented this is an inadequate placeholder which is
    /// overly dark.
    Reinhard,
}
/// “Camera exposure” control: selection of algorithm to control the scaling factor from
/// scene luminance to displayed luminance. Part of a [`GraphicsOptions`].
///
/// Note that the exact interpretation of this value also depends on on the chosen
/// [`ToneMappingOperator`].
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub(crate) enum ExposureOption {
    /// Constant exposure; light values in the scene are multiplied by this value
    /// before the tone mapping operator is applied.
    Fixed(NotNan<f32>),
    /// Exposure adjusts to compensate for the actual brightness of the scene.
    Automatic,
}
/// How to display light in a [`Space`]; part of a [`GraphicsOptions`].
///
/// [`Space`]: crate::space::Space
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub(crate) enum LightingOption {
    /// No lighting: objects will be displayed with their original surface color.
    None,
    /// Light is taken from the volume immediately above a cube face.
    /// Edges between cubes are visible.
    Flat,
    /// Light varies across surfaces.
    Smooth,
}
/// How to render transparent objects; part of a [`GraphicsOptions`].
///
/// Note: There is not yet a consistent interpretation of alpha between the `Surface`
/// and `Volumetric` options; this will probably be changed in the future in favor
/// of the volumetric interpretation.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub(crate) enum TransparencyOption {
    /// Conventional transparent surfaces.
    Surface,
    /// Accounts for the thickness of material passed through; colors' alpha values are
    /// interpreted as the opacity of a unit thickness of the material.
    Volumetric,
    /// Alpha above or below the given threshold value will be rounded to fully opaque
    /// or fully transparent, respectively.
    Threshold(NotNan<f32>),
}
/// Choices for [`GraphicsOptions::antialiasing`].
#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub(crate) enum AntialiasingOption {
    /// Do not apply antialiasing. Every pixel of the rendered image will be the exact
    /// color of some part of the world, rather than a combination of adjacent parts.
    #[default]
    None,
    /// If [multisample anti-aliasing](https://en.wikipedia.org/wiki/Multisample_anti-aliasing)
    /// or similar functionality is available, allowing relatively cheap antialiasing,
    /// then enable it.
    IfCheap,
    /// Always perform antialiasing, even if it is expensive.
    Always,
}
