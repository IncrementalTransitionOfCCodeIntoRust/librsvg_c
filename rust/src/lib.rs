#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate downcast_rs;

pub use aspect_ratio::{
    FitMode,
    AlignMode,
    Align,
    AspectRatio,
    ParseAspectRatioError,
    rsvg_aspect_ratio_parse,
    rsvg_aspect_ratio_compute
};

pub use length::{
    LengthUnit,
    LengthDir,
    RsvgLength,
    rsvg_length_parse,
    rsvg_length_normalize,
    rsvg_length_hand_normalize,
};

pub use marker::{
    rsvg_node_marker_new,
};

pub use node::{
    rsvg_node_get_type,
    rsvg_node_get_parent,
    rsvg_node_ref,
    rsvg_node_unref,
    rsvg_node_is_same,
    rsvg_node_get_state,
    rsvg_node_add_child,
    rsvg_node_set_atts,
    rsvg_node_draw,
    rsvg_node_foreach_child,
};

pub use path_builder::{
    rsvg_path_builder_new,
    rsvg_path_builder_destroy,
    rsvg_path_builder_move_to,
    rsvg_path_builder_line_to,
    rsvg_path_builder_curve_to,
    rsvg_path_builder_close_path,
    rsvg_path_builder_arc,
    rsvg_path_builder_add_to_cairo_context
};

pub use viewbox::{
    RsvgViewBox
};


mod aspect_ratio;
mod drawing_ctx;
mod handle;
mod length;
mod marker;
mod node;
mod parsers;
mod path_builder;
mod property_bag;
mod state;
mod strtod;
mod util;
mod viewbox;