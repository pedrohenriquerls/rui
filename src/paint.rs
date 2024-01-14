use crate::*;

/// Specifies how a region should be filled.
#[derive(Clone, Copy)]
pub enum Paint {
    /// Fill a region with a solid color.
    Color(Color),

    /// Fill a region with a linear gradient between two colors.
    Gradient {
        start: LocalPoint,
        end: LocalPoint,
        inner_color: Color,
        outer_color: Color,
    },
}
