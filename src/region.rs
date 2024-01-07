use kurbo::Rect;

/// Region type cribbed from Druid.
#[derive(Clone, Debug)]
pub struct Region {
    rects: Vec<Rect>,
}

impl Region {
    /// The empty region.
    pub const EMPTY: Region = Region { rects: Vec::new() };

    /// Returns the collection of rectangles making up this region.
    #[inline]
    pub fn rects(&self) -> &[Rect] {
        &self.rects
    }

    /// Adds a rectangle to this region.
    pub fn add_rect(&mut self, rect: Rect) {
        if !rect.is_empty() {
            self.rects.push(rect);
        }
    }

    /// Replaces this region with a single rectangle.
    pub fn set_rect(&mut self, rect: Rect) {
        self.clear();
        self.add_rect(rect);
    }

    /// Sets this region to the empty region.
    pub fn clear(&mut self) {
        self.rects.clear();
    }

    /// Returns a rectangle containing this region.
    pub fn bounding_box(&self) -> Rect {
        if self.rects.is_empty() {
            Rect::default()
        } else {
            self.rects[1..]
                .iter()
                .fold(self.rects[0], |r, s| r.union(s.clone()))
        }
    }

    /// Returns `true` if this region has a non-empty intersection with the given rectangle.
    pub fn intersects(&self, rect: Rect) -> bool {
        self.rects.iter().any(|other|
            rect.x0 < other.x1
                && rect.x1 > other.x0
                && rect.y0 < other.y1
                && rect.y1 > other.y0
            )
    }

    /// Returns `true` if this region is empty.
    pub fn is_empty(&self) -> bool {
        // Note that we only ever add non-empty rects to self.rects.
        self.rects.is_empty()
    }

    /// Modifies this region by including everything in the other region.
    pub fn union_with(&mut self, other: &Region) {
        self.rects.extend_from_slice(&other.rects);
    }

    // /// Modifies this region by intersecting it with the given rectangle.
    // pub fn intersect_with(&mut self, rect: WorldRect) {
    //     // TODO: this would be a good use of the nightly drain_filter function, if it stabilizes
    //     for r in &mut self.rects {
    //         *r = r.intersect(rect);
    //     }
    //     self.rects.retain(|r| r.area() > 0.0)
    // }
}

// impl<Space> std::ops::AddAssign<Vector2D<f32, Space>> for Region<Space> {
//     fn add_assign(&mut self, rhs: Vector2D<f32, Space>) {
//         for r in &mut self.rects {
//             *r = r.translate(rhs)
//         }
//     }
// }

// impl<Space> std::ops::SubAssign<Vector2D<f32, Space>> for Region<Space> {
//     fn sub_assign(&mut self, rhs: Vector2D<f32, Space>) {
//         for r in &mut self.rects {
//             *r = r.translate(-rhs)
//         }
//     }
// }

impl From<Rect> for Region {
    fn from(rect: Rect) -> Region {
        Region { rects: vec![rect] }
    }
}
