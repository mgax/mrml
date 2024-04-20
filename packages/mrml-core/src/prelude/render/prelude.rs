use std::cell::{Ref, RefCell};
use std::convert::TryFrom;
use std::rc::Rc;

use crate::helper::size::{Pixel, Size};
use crate::helper::spacing::Spacing;
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;

use super::{Error, Header, RenderOptions};

pub trait Render<'header> {
    fn header(&self) -> Ref<Header<'header>>;
    fn tag(&self) -> Option<&str> {
        None
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        None
    }

    fn extra_attributes(&self) -> Option<&Map<String, String>> {
        None
    }

    fn attribute_as_pixel(&self, name: &str) -> Option<Pixel> {
        self.attribute(name)
            .and_then(|value| Pixel::try_from(value.as_str()).ok())
    }

    fn attribute_as_size(&self, name: &str) -> Option<Size> {
        self.attribute(name)
            .and_then(|value| Size::try_from(value.as_str()).ok())
    }

    fn attribute_as_spacing(&self, name: &str) -> Option<Spacing> {
        self.attribute(name)
            .and_then(|value| Spacing::try_from(value.as_str()).ok())
    }

    fn attribute_equals(&self, key: &str, value: &str) -> bool {
        self.attribute(key).map(|res| res == value).unwrap_or(false)
    }

    fn attribute_exists(&self, key: &str) -> bool {
        self.attribute(key).is_some()
    }

    fn get_border_left(&self) -> Option<Pixel> {
        self.attribute_as_pixel("border-left").or_else(|| {
            self.attribute("border")
                .and_then(|value| Pixel::from_border(&value))
        })
    }

    fn get_border_right(&self) -> Option<Pixel> {
        self.attribute_as_pixel("border-right").or_else(|| {
            self.attribute("border")
                .and_then(|value| Pixel::from_border(&value))
        })
    }

    fn get_border_horizontal(&self) -> Pixel {
        let left = self.get_border_left().map(|v| v.value()).unwrap_or(0.0);
        let right = self.get_border_right().map(|v| v.value()).unwrap_or(0.0);
        Pixel::new(left + right)
    }

    fn get_inner_border_left(&self) -> Option<Pixel> {
        self.attribute_as_pixel("inner-border-left").or_else(|| {
            self.attribute_as_spacing("inner-border")
                .and_then(|s| s.left().as_pixel().cloned())
        })
    }

    fn get_inner_border_right(&self) -> Option<Pixel> {
        self.attribute_as_pixel("inner-border-right").or_else(|| {
            self.attribute_as_spacing("inner-border")
                .and_then(|s| s.right().as_pixel().cloned())
        })
    }

    fn get_padding_top(&self) -> Option<Pixel> {
        self.attribute_as_pixel("padding-top").or_else(|| {
            self.attribute_as_spacing("padding")
                .and_then(|s| s.top().as_pixel().cloned())
        })
    }

    fn get_padding_bottom(&self) -> Option<Pixel> {
        self.attribute_as_pixel("padding-bottom").or_else(|| {
            self.attribute_as_spacing("padding")
                .and_then(|s| s.bottom().as_pixel().cloned())
        })
    }

    fn get_padding_left(&self) -> Option<Pixel> {
        self.attribute_as_pixel("padding-left").or_else(|| {
            self.attribute_as_spacing("padding")
                .and_then(|s| s.left().as_pixel().cloned())
        })
    }

    fn get_padding_right(&self) -> Option<Pixel> {
        self.attribute_as_pixel("padding-right").or_else(|| {
            self.attribute_as_spacing("padding")
                .and_then(|s| s.right().as_pixel().cloned())
        })
    }

    fn get_padding_horizontal(&self) -> Pixel {
        let left = self.get_padding_left().map(|v| v.value()).unwrap_or(0.0);
        let right = self.get_padding_right().map(|v| v.value()).unwrap_or(0.0);
        Pixel::new(left + right)
    }

    fn get_padding_vertical(&self) -> Pixel {
        let top = self.get_padding_top().map(|v| v.value()).unwrap_or(0.0);
        let bottom = self.get_padding_bottom().map(|v| v.value()).unwrap_or(0.0);
        Pixel::new(top + bottom)
    }

    fn get_width(&self) -> Option<Size> {
        self.attribute_as_size("width")
    }

    fn default_attribute(&self, _key: &str) -> Option<&str> {
        None
    }

    fn attribute(&self, key: &str) -> Option<String> {
        if let Some(value) = self.attributes().and_then(|attrs| attrs.get(key)) {
            return Some(value.clone());
        }
        if let Some(value) = self.extra_attributes().and_then(|attrs| attrs.get(key)) {
            return Some(value.clone());
        }
        let header = self.header();
        if let Some(value) = self
            .attributes()
            .and_then(|attrs| attrs.get("mj-class"))
            .and_then(|mj_classes| {
                mj_classes
                    .split(' ')
                    .map(|mj_class| mj_class.trim())
                    .filter_map(|mj_class| header.attribute_class(mj_class, key))
                    .next()
            })
        {
            return Some(value.to_string());
        }
        if let Some(tag) = self.tag() {
            if let Some(value) = header.attribute_element(tag, key) {
                return Some(value.to_string());
            }
        }
        if let Some(value) = header.attribute_all(key) {
            return Some(value.to_string());
        }
        self.default_attribute(key).map(|item| item.to_string())
    }

    fn attribute_size(&self, key: &str) -> Option<Size> {
        self.attribute(key)
            .and_then(|value| Size::try_from(value.as_str()).ok())
    }

    fn attribute_pixel(&self, key: &str) -> Option<Pixel> {
        self.attribute(key)
            .and_then(|value| Pixel::try_from(value.as_str()).ok())
    }

    fn set_style(&self, _name: &str, tag: Tag) -> Tag {
        tag
    }

    fn set_container_width(&mut self, _width: Option<Pixel>) {}
    fn set_index(&mut self, _index: usize) {}
    fn set_siblings(&mut self, _count: usize) {}
    fn set_raw_siblings(&mut self, _count: usize) {}

    fn add_extra_attribute(&mut self, _key: &str, _value: &str) {}
    fn maybe_add_extra_attribute(&mut self, key: &str, value: Option<String>) {
        if let Some(ref value) = value {
            self.add_extra_attribute(key, value);
        }
    }

    fn render_fragment(&self, name: &str, opts: &RenderOptions) -> Result<String, Error> {
        match name {
            "main" => self.render(opts),
            _ => Err(Error::UnknownFragment(name.to_string())),
        }
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error>;
}

pub trait Renderable<'render, 'element: 'render, 'header: 'render> {
    fn is_raw(&'element self) -> bool {
        false
    }

    fn renderer(
        &'element self,
        header: Rc<RefCell<Header<'header>>>,
    ) -> Box<dyn Render<'header> + 'render>;
}
