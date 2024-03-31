mod utils;

use std::collections::HashSet;

use url::Url;

static ALLOWED_IFRAME_HOSTS: [(&str, &str); 4] = [
    ("www.openstreetmap.org", "/export/"),
    ("youtube.com", "/embed/"),
    ("www.youtube.com", "/embed/"),
    ("embed.api.video", "/vod/"),
];

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SanitizeOptions {}

#[wasm_bindgen]
pub fn sanitize(input: String, opts: SanitizeOptions) -> String {
    ammonia::Builder::new()
        .link_rel(None)
        .add_generic_attribute_prefixes(&["aria-", "data-"])
        .add_generic_attributes(&["class", "id", "style"])
        .add_tag_attributes("img", &["src"])
        .add_tags(["iframe", "input", "fieldset", "label"])
        .add_tag_attributes(
            "iframe",
            &["src", "allowfullscreen", "scrolling", "width", "height"],
        )
        .add_tag_attributes(
            "input",
            &["type", "id", "name", "checked", "value", "disabled"],
        )
        .add_tag_attributes("label", &["for"])
        .add_tag_attributes("fieldset", &["class", "name"])
        .attribute_filter(|element, attribute, value| match (element, attribute) {
            ("iframe", "src") => match Url::parse(value) {
                Ok(url) => ALLOWED_IFRAME_HOSTS
                    .iter()
                    .any(|(host, path)| {
                        url.host_str() == Some(*host) && url.path().starts_with(path)
                    })
                    .then_some(value.into()),
                _ => None,
            },
            ("input", "type") => {
                if ["radio", "checkbox"].contains(&value) {
                    Some(value.into())
                } else {
                    None
                }
            }
            _ => Some(value.into()),
        })
        .url_schemes(HashSet::from(["http", "https", "mailto", "tel", "asset"]))
        .clean(&input)
        .to_string()
}
