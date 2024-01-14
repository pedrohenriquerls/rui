use crate::*;

/// Struct for `canvas`
#[derive(Clone)]
pub struct Canvas<F> {
    func: F,
}

impl<F> View for Canvas<F>
where
    F: Fn(&mut Context<dyn renderers::Renderer>, LocalRect, &mut Vger) + 'static,
{
    fn draw(&self, path: &mut IdPath, args: &mut Context<dyn renderers::Renderer>) {
        let rect = args.get_layout(path).rect;

        args.vger.save();
        (self.func)(args, rect, args.vger);
        args.vger.restore();
    }

    fn layout(&self, path: &mut IdPath, args: &mut LayoutArgs) -> LocalSize {
        args.update_layout(
            path,
            LayoutBox {
                rect: LocalRect::new(LocalPoint::zero(), args.sz),
                offset: LocalOffset::zero(),
            },
        );
        args.sz
    }

    fn hittest(&self, path: &mut IdPath, pt: LocalPoint, cx: &mut Context<dyn renderers::Renderer>) -> Option<ViewId> {
        let rect = cx.get_layout(path).rect;

        if rect.contains(pt) {
            Some(cx.view_id(path))
        } else {
            None
        }
    }

    fn gc(&self, path: &mut IdPath, cx: &mut Context<dyn renderers::Renderer>, map: &mut Vec<ViewId>) {
        map.push(cx.view_id(path));
    }
}

/// Canvas for GPU drawing with Vger. See https://github.com/audulus/vger-rs.
pub fn canvas<F: Fn(&mut Context<dyn renderers::Renderer>, LocalRect) + 'static>(f: F) -> impl View {
    Canvas { func: f }
}

impl<F> private::Sealed for Canvas<F> {}
