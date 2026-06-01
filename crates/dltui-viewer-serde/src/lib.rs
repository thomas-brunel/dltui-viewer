use std::str::FromStr;

pub mod dlt_application;
pub mod dlt_context;
pub mod dlt_ecu;
pub mod dlt_filter;
pub mod dlt_plugin;
pub mod dlt_project;
pub mod dlt_settings;

// pub enum Color {
//     Hex(String),
//     RGB(u8, u8, u8),
//     HSL(u8, u8, u8),
// }

#[derive(Debug)]
pub enum Error {
    UnsupportedExtension,
    XmlTree(xmltree::Error),
    XmlTreeParse(xmltree::ParseError),
    FieldMissing(String),
    FieldTextMissing(String),
    FieldTextParseError(String),
}

impl From<xmltree::Error> for Error {
    fn from(value: xmltree::Error) -> Self {
        Self::XmlTree(value)
    }
}

impl From<xmltree::ParseError> for Error {
    fn from(value: xmltree::ParseError) -> Self {
        Self::XmlTreeParse(value)
    }
}

pub(crate) fn deserialize_child<T>(
    parent: &xmltree::Element,
    field: &str,
    deserializer: &dyn Fn(&xmltree::Element) -> Result<T, Error>,
) -> Result<T, Error> {
    let xml_child = parent
        .get_child(field)
        .ok_or_else(|| Error::FieldMissing(field.to_string()))?;
    deserializer(xml_child)
}

pub(crate) fn deserialize_children<T>(
    parent: &mut xmltree::Element,
    field: &str,
    deserializer: &dyn Fn(&xmltree::Element) -> Result<T, Error>,
) -> Result<Vec<T>, Error> {
    let mut de_children = Vec::new();
    while let Some(child) = parent.take_child(field) {
        de_children.push(deserializer(&child)?)
    }
    Ok(de_children)
}

pub(crate) fn deserialize_mut_children<T>(
    parent: &mut xmltree::Element,
    field: &str,
    deserializer: &dyn Fn(&mut xmltree::Element) -> Result<T, Error>,
) -> Result<Vec<T>, Error> {
    let mut de_children = Vec::new();
    while let Some(mut child) = parent.take_child(field) {
        de_children.push(deserializer(&mut child)?)
    }
    Ok(de_children)
}

pub(crate) fn try_get_value(
    parent: &xmltree::Element,
    field: &str,
) -> Result<Option<String>, Error> {
    match parent.get_child(field) {
        Some(el) => match el.get_text() {
            Some(text) => Ok(Some(format!("{}", text))),
            None => Ok(None),
        },
        None => return Err(Error::FieldMissing(field.to_string())),
    }
}

pub(crate) fn get_value(parent: &xmltree::Element, field: &str) -> Result<String, Error> {
    match parent.get_child(field) {
        Some(el) => match el.get_text() {
            Some(text) => Ok(text.to_string()),
            None => return Err(Error::FieldTextMissing(field.to_string())),
        },
        None => return Err(Error::FieldMissing(field.to_string())),
    }
}

pub(crate) fn try_value_as<T>(parent: &xmltree::Element, field: &str) -> Result<Option<T>, Error>
where
    T: FromStr,
{
    if let Some(value) = try_get_value(parent, field)? {
        let Ok(value_t) = value.parse() else {
            return Err(Error::FieldTextParseError(field.to_string()));
        };
        Ok(Some(value_t))
    } else {
        Ok(None)
    }
}

pub(crate) fn value_as<T>(parent: &xmltree::Element, field: &str) -> Result<T, Error>
where
    T: FromStr,
{
    let value = get_value(parent, field)?;
    let Ok(value_t) = value.parse() else {
        return Err(Error::FieldTextParseError(field.to_string()));
    };
    Ok(value_t)
}

pub(crate) fn value_as_bool(parent: &xmltree::Element, field: &str) -> Result<bool, Error> {
    let value: u8 = value_as(parent, field)?;
    let value_bool = if value == 1 { true } else { false };
    Ok(value_bool)
}
