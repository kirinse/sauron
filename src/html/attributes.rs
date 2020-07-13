//! https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
//!
use crate::Attribute;
pub use attribute_macros::*;
pub use sauron_vdom::builder::attr;
use sauron_vdom::{
    Style,
    Value,
};
pub use style_macro::*;

#[macro_use]
mod attribute_macros;
#[macro_use]
mod style_macro;

/// create a style attribute
pub fn style<V, MSG>(style_name: &'static str, value: V) -> Attribute<MSG>
where
    V: Into<Value> + Clone,
{
    Attribute::from_styles(vec![Style::new(style_name, value.into())])
}

/// A helper function which creates a style attribute by assembling the tuples into a string for the style value.
/// ```ignore
///  div([styles([("display", "flex"), ("flex-direction", "row")])], [])
/// ```
/// is the same way of writing
/// ```ignore
/// div([style("display:flex;flex-direction:row;")],[])
/// ```
pub fn styles<V, MSG, P>(pairs: P) -> Attribute<MSG>
where
    V: Into<Value> + Clone,
    P: AsRef<[(&'static str, V)]>,
{
    let mut styles = vec![];
    for (key, value) in pairs.as_ref() {
        styles.push(Style::new(*key, Into::<Value>::into(value.clone())));
    }
    Attribute::from_styles(styles)
}

/// A helper function to build styles by accepting pairs
pub fn styles_values<MSG, P>(pairs: P) -> Attribute<MSG>
where
    P: AsRef<[(&'static str, Value)]>,
{
    let mut styles = vec![];
    for (key, value) in pairs.as_ref() {
        styles.push(Style::new(*key, value.clone()));
    }
    Attribute::from_styles(styles)
}

/// A helper function which creates a style attribute by assembling only the parts that passed the
/// boolean flag
/// ```ignore
///    styles_flag([
///        ("display", "block", self.is_active),
///        ("display", "none", !self.is_active),
///    ]),
/// ```
/// This could also be written as
/// ```ignore
///     styles([("display", if self.is_active { "block" }else{ "none" })])
/// ```
pub fn styles_flag<V, MSG, P>(trio: P) -> Attribute<MSG>
where
    V: Into<Value> + Clone,
    P: AsRef<[(&'static str, V, bool)]>,
{
    let mut styles = vec![];
    for (key, value, flag) in trio.as_ref() {
        if *flag {
            styles.push(Style::new(*key, Into::<Value>::into(value.clone())));
        }
    }
    Attribute::from_styles(styles)
}

/// ```ignore
///    classes_flag([
///        ("dashed", self.is_hidden),
///        ("error", self.has_error),
///    ]),
/// ```
pub fn classes_flag<P, MSG>(pair: P) -> Attribute<MSG>
where
    P: AsRef<[(&'static str, bool)]>,
{
    let mut class_list = Vec::with_capacity(pair.as_ref().len());
    for (class, flag) in pair.as_ref() {
        if *flag {
            class_list.push((*class).to_string());
        }
    }
    class(class_list.join(" "))
}

/// a helper function to add multiple classes to a node
///
/// ```ignore
///    div(vec![classes(["dashed", "error"])], vec![])
/// ```
pub fn classes<C, V, MSG>(class_list: C) -> Attribute<MSG>
where
    V: ToString,
    C: AsRef<[V]>,
{
    class(
        class_list
            .as_ref()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" "),
    )
}

/// A helper function for setting attributes with no values such as checked
/// in checkbox input type
/// This is best called to be appended to the node since this
/// returns an array of attributes which doesn't play well with the others
/// Example:
/// ```ignore
/// input([r#type("checkbox")], []).attributes(attrs_flag([(
///                             "checked",
///                             "checked",
///                             is_checked,
///                         )])),
/// ```
pub fn attrs_flag<V, MSG, P>(trio: P) -> Vec<Attribute<MSG>>
where
    V: Into<Value> + Clone,
    P: AsRef<[(&'static str, V, bool)]>,
{
    let mut attributes: Vec<Attribute<MSG>> =
        Vec::with_capacity(trio.as_ref().len());
    for (key, value, flag) in trio.as_ref() {
        if *flag {
            attributes.push(attr(key, value.clone()));
        }
    }
    attributes
}

/// set the checked value, used checkbox and radio buttons
pub fn checked<MSG>(is_checked: bool) -> Attribute<MSG> {
    if is_checked {
        attr("checked", "checked")
    } else {
        attr("", "")
    }
}

/// set the inner html of this element without comparing in the diff
/// this always sets the value
/// This is for optimization purposes
/// and will lead to some hacks in the implementation
pub fn inner_html<V, MSG>(inner_html: V) -> Attribute<MSG>
where
    V: Into<Value> + Clone,
{
    Attribute::function_call("inner_html", inner_html.into())
}
