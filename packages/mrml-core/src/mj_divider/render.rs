use super::{MjDivider, NAME};
use crate::helper::size::{Pixel, Size};
use crate::prelude::render::*;

impl<'element, 'header> Renderer<'element, 'header, MjDivider, ()> {
    fn set_style_p_without_width<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.add_style(
            "border-top",
            format!(
                "{} {} {}",
                self.attribute("border-style").unwrap(),
                self.attribute("border-width").unwrap(),
                self.attribute("border-color").unwrap()
            ),
        )
        .add_style("font-size", "1px")
        .add_style("margin", "0px auto")
    }
    fn set_style_p<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        self.set_style_p_without_width(tag)
            .maybe_add_style("width", self.attribute("width"))
    }

    fn set_style_outlook<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        self.set_style_p_without_width(tag)
            .add_style("width", self.get_outlook_width().to_string())
    }

    fn get_outlook_width(&self) -> Pixel {
        let container_width = self.container_width.as_ref().unwrap();
        let padding_horizontal = self.get_padding_horizontal();
        let width = self
            .attribute_as_size("width")
            .unwrap_or_else(|| Size::percent(100.0));
        match width {
            Size::Percent(value) => {
                let effective = container_width.value() - padding_horizontal.value();
                let multiplier = value.value() / 100.0;
                Pixel::new(effective * multiplier)
            }
            Size::Pixel(value) => value,
            _ => Pixel::new(container_width.value() - padding_horizontal.value()),
        }
    }

    fn render_after(&self, buf: &mut RenderBuffer) {
        let table = self
            .set_style_outlook(Tag::table_presentation())
            .add_attribute("align", "center")
            .add_attribute("width", self.get_outlook_width().to_string());
        let tr = Tag::tr();
        let td = Tag::td()
            .add_style("height", "0")
            .add_style("line-height", "0");

        buf.start_conditional_tag();
        table.render_open(buf);
        tr.render_open(buf);
        td.render_text(buf, "&nbsp;");
        tr.render_close(buf);
        table.render_close(buf);
        buf.end_conditional_tag();
    }
}

impl<'element, 'header> Render<'element, 'header> for Renderer<'element, 'header, MjDivider, ()> {
    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "align" => Some("center"),
            "border-color" => Some("#000000"),
            "border-style" => Some("solid"),
            "border-width" => Some("4px"),
            "padding" => Some("10px 25px"),
            "width" => Some("100%"),
            _ => None,
        }
    }

    fn raw_attribute(&self, key: &str) -> Option<&'element str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn context(&self) -> &'header RenderContext<'header> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let p = self.set_style_p(Tag::new("p"));
        p.render_text(&mut cursor.buffer, "");

        self.render_after(&mut cursor.buffer);
        Ok(())
    }
}

impl<'r, 'element: 'r, 'header: 'r> Renderable<'r, 'element, 'header> for MjDivider {
    fn renderer(
        &'element self,
        context: &'header RenderContext<'header>,
    ) -> Box<dyn Render<'element, 'header> + 'r> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-divider");
    crate::should_render!(class, "mj-divider-class");
    crate::should_render!(
        container_background_color,
        "mj-divider-container-background-color"
    );
    crate::should_render!(padding, "mj-divider-padding");
    crate::should_render!(width, "mj-divider-width");
}
