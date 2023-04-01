
bitflags::bitflags! {
    #[doc = " Deficiencies of a rendering."] #[doc = ""] #[doc =
    " This type describes the ways in which a rendered image could fail to accurately"]
    #[doc =
    " represent the scene, or fail to comply with requested [`GraphicsOptions`]."] #[doc
    = ""] #[doc =
    " It is a [`bitflags`] generated bit-flag type. *Note: We make no guarantees that"]
    #[doc =
    " the numeric value of flags will stay the same across versions*; please treat this"]
    #[doc = " as a set of named values only."] #[doc = ""] #[doc =
    " The [empty](Self::empty) set means no flaws are present."] #[doc = ""] #[doc =
    " [`GraphicsOptions`]: super::GraphicsOptions"] #[derive(Clone, Copy, Debug, Hash,
    Eq, Ord, PartialEq, PartialOrd)] pub struct Flaws : u16 { #[doc =
    " The rendering is incomplete due to the renderer not having had enough"] #[doc =
    " time to finish initialization or catch up to changes."] const UNFINISHED = 1 << 0;
    #[doc = " Antialiasing has not been used,"] #[doc =
    " despite being requested by the graphics options."] const NO_ANTIALIASING = 1 << 2;
    #[doc = " Bloom has not been rendered,"] #[doc =
    " despite being requested by the graphics options."] const NO_BLOOM = 1 << 3; #[doc =
    " A cursor has not been rendered, despite one being given."] const NO_CURSOR = 1 <<
    4; #[doc =
    " View-distance fog has not been rendered, despite being requested by the"] #[doc =
    " graphics options."] #[doc = ""] #[doc =
    " This does not refer to explicitly semitransparent objects within the scene."] const
    NO_FOG = 1 << 5; #[doc =
    " Surfaces that should have textures rather than a solid color don't."] const
    MISSING_TEXTURES = 1 << 6; }
}
