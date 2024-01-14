use crate::*;
use std::any::{Any, TypeId};

pub struct LayoutArgs<'a> {
    pub sz: LocalSize,
    pub cx: &'a mut Context<dyn renderers::Renderer>,
    pub text_bounds: &'a mut dyn FnMut(&str, u32, Option<f32>) -> LocalRect,
}

impl<'a> LayoutArgs<'a> {
    pub fn size(&mut self, sz: LocalSize) -> LayoutArgs {
        LayoutArgs {
            sz,
            cx: self.cx,
            text_bounds: self.text_bounds,
        }
    }
}

/// Trait for the unit of UI composition.
pub trait View: private::Sealed + 'static {
    /// Builds an AccessKit tree. The node ID for the subtree is returned. All generated nodes are accumulated.
    fn access(
        &self,
        _path: &mut IdPath,
        _cx: &mut Context<dyn renderers::Renderer>,
        _nodes: &mut Vec<(accesskit::NodeId, accesskit::Node)>,
    ) -> Option<accesskit::NodeId> {
        None
    }

    /// Accumulates information about menu bar commands.
    fn commands(&self, _path: &mut IdPath, _cx: &mut Context<dyn renderers::Renderer>, _cmds: &mut Vec<CommandInfo>) {}

    /// Determines dirty regions which need repainting.
    fn dirty(&self, _path: &mut IdPath, _xform: LocalToWorld, _cx: &mut Context<dyn renderers::Renderer>) {}

    /// Draws the view using vger.
    fn draw(&self, path: &mut IdPath, args: &mut Context<dyn renderers::Renderer>);

    /// Gets IDs for views currently in use.
    ///
    /// Push onto map if the view stores layout or state info.
    fn gc(&self, _path: &mut IdPath, _cx: &mut Context<dyn renderers::Renderer>, _map: &mut Vec<ViewId>) {}

    /// Returns the topmost view which the point intersects.
    fn hittest(&self, _path: &mut IdPath, _pt: LocalPoint, _cx: &mut Context<dyn renderers::Renderer>) -> Option<ViewId> {
        None
    }

    /// For detecting flexible sized things in stacks.
    fn is_flexible(&self) -> bool {
        false
    }

    /// Lays out subviews and return the size of the view.
    ///
    /// `sz` is the available size for the view
    /// `vger` can be used to get text sizing
    ///
    /// Note that we should probably have a separate text
    /// sizing interface so we don't need a GPU and graphics
    /// context set up to test layout.
    fn layout(&self, path: &mut IdPath, args: &mut LayoutArgs) -> LocalSize;

    /// Processes an event.
    fn process(
        &self,
        _event: &Event,
        _path: &mut IdPath,
        _cx: &mut Context<dyn renderers::Renderer>,
        _actions: &mut Vec<Box<dyn Any>>,
    ) {
    }

    /// Returns the type ID of the underlying view.
    fn tid(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}
