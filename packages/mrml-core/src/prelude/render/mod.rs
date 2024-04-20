use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::atomic::{AtomicU16, Ordering};

use super::hash::Set;
use crate::helper::size::{Pixel, Size};
use crate::mj_head::MjHead;
use crate::prelude::hash::Map;

mod error;
mod prelude;

pub use error::Error;
pub use prelude::*;

#[deprecated = "use mrml::prelude::render::RenderOptions instead"]
pub type Options = RenderOptions;

#[derive(Debug)]
pub struct RenderOptions {
    pub disable_comments: bool,
    pub social_icon_origin: Option<Cow<'static, str>>,
    pub fonts: HashMap<String, Cow<'static, str>>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            disable_comments: false,
            social_icon_origin: None,
            fonts: HashMap::from([
                (
                    "Open Sans".into(),
                    "https://fonts.googleapis.com/css?family=Open+Sans:300,400,500,700".into(),
                ),
                (
                    "Droid Sans".into(),
                    "https://fonts.googleapis.com/css?family=Droid+Sans:300,400,500,700".into(),
                ),
                (
                    "Lato".into(),
                    "https://fonts.googleapis.com/css?family=Lato:300,400,500,700".into(),
                ),
                (
                    "Roboto".into(),
                    "https://fonts.googleapis.com/css?family=Roboto:300,400,500,700".into(),
                ),
                (
                    "Ubuntu".into(),
                    "https://fonts.googleapis.com/css?family=Ubuntu:300,400,500,700".into(),
                ),
            ]),
        }
    }
}

pub struct Header<'h> {
    head: &'h Option<MjHead>,
    attributes_all: Map<&'h str, &'h str>,
    attributes_class: Map<&'h str, Map<&'h str, &'h str>>,
    attributes_element: Map<&'h str, Map<&'h str, &'h str>>,
    breakpoint: Pixel,
    font_families: Map<&'h str, &'h str>,
    used_font_families: Set<String>,
    media_queries: Map<String, Size>,
    styles: Set<String>,
    lang: Option<String>,
    generator: AtomicU16,
}

impl<'h> Header<'h> {
    pub fn new(head: &'h Option<MjHead>) -> Self {
        Self {
            head,
            attributes_all: head
                .as_ref()
                .map(|h| h.build_attributes_all())
                .unwrap_or_default(),
            attributes_class: head
                .as_ref()
                .map(|h| h.build_attributes_class())
                .unwrap_or_default(),
            attributes_element: head
                .as_ref()
                .map(|h| h.build_attributes_element())
                .unwrap_or_default(),
            breakpoint: head
                .as_ref()
                .and_then(|h| h.breakpoint())
                .and_then(|s| Pixel::try_from(s.value()).ok())
                .unwrap_or_else(|| Pixel::new(480.0)),
            font_families: head
                .as_ref()
                .map(|h| h.build_font_families())
                .unwrap_or_default(),
            used_font_families: Set::new(),
            media_queries: Map::new(),
            styles: Set::new(),
            lang: Default::default(),
            generator: AtomicU16::new(0),
        }
    }

    pub fn attribute_all(&self, key: &str) -> Option<&str> {
        self.attributes_all.get(key).copied()
    }

    pub fn attribute_class(&self, name: &str, key: &str) -> Option<&str> {
        self.attributes_class
            .get(name)
            .and_then(|class_map| class_map.get(key))
            .copied()
    }

    pub fn attribute_element(&self, name: &str, key: &str) -> Option<&str> {
        self.attributes_element
            .get(name)
            .and_then(|elt| elt.get(key))
            .copied()
    }

    pub fn head(&self) -> &Option<MjHead> {
        self.head
    }

    pub fn breakpoint(&self) -> &Pixel {
        &self.breakpoint
    }

    pub fn add_used_font_family(&mut self, value: &str) {
        self.used_font_families.insert(value.to_string());
    }

    pub fn add_font_families<T: AsRef<str>>(&mut self, value: T) {
        for name in value
            .as_ref()
            .split(',')
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
        {
            self.add_used_font_family(name);
        }
    }

    pub fn maybe_add_font_families<T: AsRef<str>>(&mut self, value: Option<T>) {
        if let Some(value) = value {
            self.add_font_families(value);
        }
    }

    pub fn used_font_families(&self) -> &Set<String> {
        &self.used_font_families
    }

    pub fn font_families(&self) -> &Map<&str, &str> {
        &self.font_families
    }

    pub fn media_queries(&self) -> &Map<String, Size> {
        &self.media_queries
    }

    pub fn add_media_query(&mut self, classname: String, size: Size) {
        self.media_queries.insert(classname, size);
    }

    pub fn styles(&self) -> &Set<String> {
        &self.styles
    }

    pub fn add_style(&mut self, value: String) {
        self.styles.insert(value);
    }

    pub fn maybe_add_style(&mut self, value: Option<String>) {
        if let Some(value) = value {
            self.add_style(value);
        }
    }

    pub fn lang(&self) -> Option<&str> {
        self.lang.as_deref()
    }

    pub fn maybe_set_lang(&mut self, value: Option<String>) {
        self.lang = value;
    }

    pub fn next_id(&self) -> String {
        let id = self.generator.fetch_add(1, Ordering::SeqCst);
        format!("{id:0>8}")
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! should_render {
    ($name: ident, $template: literal) => {
        concat_idents::concat_idents!(fn_name = $name, _, sync {
            #[cfg(feature = "parse")]
            #[test]
            fn fn_name() {
                let opts = $crate::prelude::render::RenderOptions::default();
                let template = include_str!(concat!("../../resources/compare/success/", $template, ".mjml"));
                let expected = include_str!(concat!("../../resources/compare/success/", $template, ".html"));
                let root = $crate::parse(template).unwrap();
                html_compare::assert_similar(expected, root.render(&opts).unwrap().as_str());
            }
        });
        concat_idents::concat_idents!(fn_name = $name, _, "async" {
            #[cfg(all(feature = "async", feature = "parse"))]
            #[tokio::test]
            async fn fn_name() {
                let opts = $crate::prelude::render::RenderOptions::default();
                let template = include_str!(concat!("../../resources/compare/success/", $template, ".mjml"));
                let expected = include_str!(concat!("../../resources/compare/success/", $template, ".html"));
                let root = $crate::async_parse(template).await.unwrap();
                html_compare::assert_similar(expected, root.render(&opts).unwrap().as_str());
            }
        });
    };
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn header_should_increase() {
        let head = None;
        let header = Rc::new(RefCell::new(super::Header::new(&head)));
        assert_eq!(header.borrow().next_id(), "00000000");
        assert_eq!(header.borrow().next_id(), "00000001");
        assert_eq!(header.borrow().next_id(), "00000002");
    }
}
