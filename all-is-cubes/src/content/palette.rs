//! Colors to use in the UI and default content.
//!
//! This module exists to be a place where we can review the different colors in use
//! and tweak them to go well together, and avoid introducing extra slightly different
//! hardcoded colors if possible.
//!
//! TODO: Split "system UI" colors and "demo content" colors.
palette! {
    #[doc = " Default sky color for new [`Space`](crate::space::Space)s."] DAY_SKY_COLOR
    = srgb[243 243 255]; #[doc =
    " Used on the surface of a mesh where there should be a texture, but something went"]
    #[doc = " wrong."] #[doc = ""] #[doc =
    " TODO: This is no longer actually used, as the mesh generation now explicitly"]
    #[doc =
    " reports flaws while using an approximate value. But perhaps we should use this"]
    #[doc = " for another error condition."] MISSING_TEXTURE_FALLBACK = srgb[0xFF 0x00
    0xBB 0xFF]; #[doc =
    " Used when a recursive block definition should have provided a voxel color but did not."]
    MISSING_VOXEL_FALLBACK = srgb[0xBB 0x00 0xFF 0xFF]; #[doc =
    " Used in unallocated texture atlas space."] #[doc = ""] #[doc =
    " TODO: Not currently used."] UNPAINTED_TEXTURE_FALLBACK = srgb[0x00 0xC5 0xC5 0xFF];
    #[doc =
    " Fill color to draw when a renderer does not have any [`Space`](crate::space::Space)"]
    #[doc = " to define a sky color."] NO_WORLD_TO_SHOW = srgb[0xBC 0xBC 0xBC 0xFF];
}
palette! {
    #[doc =
    " A realistic value for typical black materials, which do not absorb all light."]
    ALMOST_BLACK = srgb[0x3d 0x3d 0x3d]; #[doc = ""] GRASS = srgb[0x61 0xAA 0x31]; #[doc
    = ""] DIRT = srgb[0x6C 0x50 0x44]; #[doc =
    " Generic unspecified some-kind-of-stone..."] STONE = srgb[0xD9 0xD7 0xD5]; #[doc =
    " TODO: Not actually exercised in demo content yet"] TREE_BARK = srgb[0x93 0x5C
    0x32]; #[doc = " TODO: Not actually exercised in demo content yet"] TREE_LEAVES =
    srgb[0x61 0xAA 0x31]; #[doc = " Some kind of metallic structure."] #[doc = ""] #[doc
    = " TODO: not taken from real references"] STEEL = srgb[0xAA 0xAA 0xAA]; #[doc =
    " Some kind of prepared wood."] #[doc = ""] #[doc =
    " TODO: not taken from real references"] PLANK = srgb[0xE8 0xCC 0x95];
}
palette! {
    LOGO_FILL = srgb[0xC7 0x33 0x78]; LOGO_STROKE = srgb[0x33 0x33 0x33];
}
palette! {
    CURSOR_OUTLINE = srgb[0x00 0x00 0x00 0xFF]; #[doc =
    " Illumination color in the HUD."] HUD_SKY = srgb[0xFF 0xFF 0xFF]; HUD_TEXT_FILL =
    srgb[0x00 0x00 0x00 0xFF]; HUD_TEXT_STROKE = srgb[0xFF 0xFF 0xFF 0xFF];
    HUD_TOOLBAR_BACK = srgb[0x7E 0x7E 0x7E 0xFF]; HUD_TOOLBAR_FRAME = srgb[0xDD 0xDD 0xDD
    0xFF]; MENU_BACK = srgb[0xBC 0xBC 0xBC 0xFF]; MENU_FRAME = srgb[0xFA 0xFA 0xFA 0xFF];
    BUTTON_FRAME = srgb[0x3d 0x3d 0x3d 0xFF]; BUTTON_BACK = srgb[0xBC 0xBC 0xBC 0xFF];
    BUTTON_LABEL = srgb[0x3d 0x3d 0x3d 0xFF]; BUTTON_ACTIVATED_BACK = srgb[0xE1 0xE1 0xE1
    0xFF]; BUTTON_ACTIVATED_LABEL = srgb[0x63 0x63 0x63 0xFF];
}
palette! {
    DEBUG_BEHAVIOR_BOUNDS = srgb[0x00 0x70 0x00 0xFF]; DEBUG_COLLISION_BOX = srgb[0x00
    0x00 0xFF 0xFF]; DEBUG_COLLISION_CUBES = srgb[0xFF 0x00 0x00 0xFF]; DEBUG_CHUNK_MAJOR
    = srgb[0x00 0x00 0xE8 0xFF]; DEBUG_CHUNK_MINOR = srgb[0x00 0xE8 0xE8 0xFF];
}
palette! {
    UNIFORM_LUMINANCE_RED = srgb[0x9E 0x00 0x00]; UNIFORM_LUMINANCE_GREEN = srgb[0x00
    0x59 0x00]; UNIFORM_LUMINANCE_BLUE = srgb[0x00 0x00 0xFF];
}
