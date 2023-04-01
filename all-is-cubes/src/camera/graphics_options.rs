

/// Options for controlling rendering (not affecting gameplay except informationally).
///
/// Some options may be ignored by some renderers, such as when they request a particular
/// implementation approach or debug visualization. Renderers should make an effort to
/// report such failings via [`Flaws`](crate::camera::Flaws).
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[non_exhaustive]
pub(crate) struct GraphicsOptions {}
impl Default for GraphicsOptions {
    /// Default graphics options broadly have “everything reasonable” turned on
    /// (they may disable things that are not well-implemented yet).
    ///
    /// TODO: Explain exactly what the default is.
    fn default() -> Self {
        loop {}
    }
}
