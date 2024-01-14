use crate::*;
use std::any::Any;

pub struct Clip<V> {
    child: V,
}

impl<V> Clip<V>
where
    V: View,
{
    fn geom(&self, path: &IdPath, cx: &mut Context<dyn renderers::Renderer>) -> LocalRect {
        cx.get_layout(path).rect
    }

    pub fn new(child: V) -> Self {
        Self { child }
    }
}

impl<V> View for Clip<V>
where
    V: View,
{
    fn process(
        &self,
        event: &Event,
        path: &mut IdPath,
        cx: &mut Context<dyn renderers::Renderer>,
        actions: &mut Vec<Box<dyn Any>>,
    ) {
        path.push(0);
        self.child.process(event, path, cx, actions);
        path.pop();
    }

    fn draw(&self, path: &mut IdPath, args: &mut Context<dyn renderers::Renderer>) {
        let rect = self.geom(path, args);

        args.vger.save();
        args.vger.scissor(rect);
        path.push(0);
        self.child.draw(path, args);
        path.pop();
        args.vger.restore();
    }

    fn layout(&self, path: &mut IdPath, args: &mut LayoutArgs) -> LocalSize {
        path.push(0);
        self.child.layout(path, args);
        path.pop();
        args.update_layout(
            path,
            LayoutBox {
                rect: LocalRect::new(LocalPoint::zero(), args.sz),
                offset: LocalOffset::zero(),
            },
        );
        // XXX: should this expand to the available space?
        args.sz
    }

    fn hittest(&self, path: &mut IdPath, pt: LocalPoint, cx: &mut Context<dyn renderers::Renderer>) -> Option<ViewId> {
        let rect = self.geom(path, cx);

        if rect.contains(pt) {
            // Test against children.
            path.push(0);
            let vid = self.child.hittest(path, pt, cx);
            path.pop();
            vid
        } else {
            None
        }
    }

    fn commands(&self, path: &mut IdPath, cx: &mut Context<dyn renderers::Renderer>, cmds: &mut Vec<CommandInfo>) {
        path.push(0);
        self.child.commands(path, cx, cmds);
        path.pop();
    }

    fn gc(&self, path: &mut IdPath, cx: &mut Context<dyn renderers::Renderer>, map: &mut Vec<ViewId>) {
        map.push(cx.view_id(path));
        path.push(0);
        self.child.gc(path, cx, map);
        path.pop();
    }

    fn access(
        &self,
        path: &mut IdPath,
        cx: &mut Context<dyn renderers::Renderer>,
        nodes: &mut Vec<(accesskit::NodeId, accesskit::Node)>,
    ) -> Option<accesskit::NodeId> {
        path.push(0);
        let node_id = self.child.access(path, cx, nodes);
        path.pop();
        node_id
    }
}

impl<V> private::Sealed for Clip<V> {}
