use cosmic_text::{TextLayout, AttrsList, Attrs};
use peniko::Color;
use crate::*;

pub trait TextModifiers: View + Sized {
    fn layout(self, attrs: AttrsList) -> Text;
}

/// Struct for `text`.
#[derive(Clone)]
pub struct Text {
    text: String,
    layout: TextLayout,
}

impl Text {
    pub const DEFAULT_SIZE: u32 = 18;
}

impl View for Text {
    fn draw(&self, path: &mut IdPath, args: &mut DrawArgs) {
        // let vger = &mut args.cx.vger;
        // let origin = vger.text_bounds(self.text.as_str(), self.size, None).origin;

        // vger.save();
        // vger.translate([-origin.x, -origin.y]);
        // vger.text(self.text.as_str(), self.size, self.color, None);
        // vger.restore();
        let rect = args.cx.get_layout(path).rect;
        let origin = rect.origin;
        args.rd.translate([-origin.x, -origin.y].into(), true);
        args.rd.draw_text(&self.layout, rect.origin);
        args.rd.restore();
    }
    fn layout(&self, path: &mut IdPath, args: &mut LayoutArgs) -> LocalSize {
        args.cx.get_layout(path).rect.size
        // (args.cx.text_bounds)(self.text.as_str(), self.size, None).size
    }
    fn hittest(&self, _path: &mut IdPath, _pt: LocalPoint, _cx: &mut Context) -> Option<ViewId> {
        None
    }

    fn access(
        &self,
        path: &mut IdPath,
        cx: &mut Context,
        nodes: &mut Vec<(accesskit::NodeId, accesskit::Node)>,
    ) -> Option<accesskit::NodeId> {
        let aid = cx.view_id(path).access_id();
        let mut builder = accesskit::NodeBuilder::new(accesskit::Role::LabelText);
        builder.set_name(self.text.clone());
        nodes.push((aid, builder.build(&mut cx.access_node_classes)));
        Some(aid)
    }
}

impl TextModifiers for Text {
    fn layout(self, attrs: AttrsList) -> Text {
        let mut layout = TextLayout::new();
        layout.set_text(&self.text, attrs);
        Text {
            text: self.text,
            layout
        }
    }
}

impl private::Sealed for Text {}

/// Shows a string as a label (not editable).
pub fn text(name: &str) -> Text {

        // size: Text::DEFAULT_SIZE,
    // color: TEXT_COLOR
    let mut layout = TextLayout::new();
    let attrs = Attrs::new().color(Color::GRAY);

    let attrs_list = AttrsList::new(attrs);
    layout.set_text(name, attrs_list);

    Text {
        text: String::from(name),
        layout
    }
}

impl<V> View for V
where
    V: std::fmt::Display + std::fmt::Debug + 'static,
{
    fn draw(&self, path: &mut IdPath, args: &mut DrawArgs) {
        let txt = &format!("{}", self);
        // let vger = &mut args.cx.vger;
        // let origin = vger.text_bounds(txt, Text::DEFAULT_SIZE, None).origin;

        // vger.save();
        // vger.translate([-origin.x, -origin.y]);
        // vger.text(txt, Text::DEFAULT_SIZE, TEXT_COLOR, None);
        // vger.restore();
        let rect = args.cx.get_layout(path).rect;
        let origin = rect.origin;
        args.rd.translate([-origin.x, -origin.y].into(), true);
        let mut layout = TextLayout::new();
        let attrs = Attrs::new().color(Color::GRAY);

        let attrs_list = AttrsList::new(attrs);
        layout.set_text(txt, attrs_list);
        args.rd.draw_text(&layout, rect.origin);
        args.rd.restore();
    }
    fn layout(&self, path: &mut IdPath, args: &mut LayoutArgs) -> LocalSize {
        args.cx.get_layout(path).rect.size
    }

    fn access(
        &self,
        path: &mut IdPath,
        cx: &mut Context,
        nodes: &mut Vec<(accesskit::NodeId, accesskit::Node)>,
    ) -> Option<accesskit::NodeId> {
        let aid = cx.view_id(path).access_id();
        let mut builder = accesskit::NodeBuilder::new(accesskit::Role::LabelText);
        builder.set_name(format!("{}", self));
        nodes.push((aid, builder.build(&mut cx.access_node_classes)));
        Some(aid)
    }
}

impl<V> TextModifiers for V
where
    V: std::fmt::Display + std::fmt::Debug + 'static,
{

    fn layout(self, attrs: AttrsList) -> Text {
        let mut layout = TextLayout::new();
        let text = format!("{}", self);
        layout.set_text(&text, attrs);
        Text {
            text,
            layout
        }
    }
}

impl<V> private::Sealed for V where V: std::fmt::Display {}
