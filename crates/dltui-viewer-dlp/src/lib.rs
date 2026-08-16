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
    MissingFileExtension,
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

pub(crate) trait DlpSerde: Sized {
    fn deserialize(xml: &mut xmltree::Element) -> Result<Self, Error>;
    fn serialize(&self) -> xmltree::Element;
}

pub(crate) fn deserialize_child<T>(parent: &mut xmltree::Element, field: &str) -> Result<T, Error>
where
    T: DlpSerde,
{
    let mut xml_child = parent
        .take_child(field)
        .ok_or_else(|| Error::FieldMissing(field.to_string()))?;
    T::deserialize(&mut xml_child)
}

pub(crate) fn deserialize_children<T>(
    parent: &mut xmltree::Element,
    field: &str,
) -> Result<Vec<T>, Error>
where
    T: DlpSerde,
{
    let mut de_children = Vec::new();
    while let Some(mut child) = parent.take_child(field) {
        de_children.push(T::deserialize(&mut child)?)
    }
    Ok(de_children)
}

pub(crate) fn serialize_child<T>(parent: &mut xmltree::Element, child: &T)
where
    T: DlpSerde,
{
    let xml_child = child.serialize();
    parent.children.push(xmltree::XMLNode::Element(xml_child));
}

pub(crate) fn serialize_children<T>(parent: &mut xmltree::Element, children: &Vec<T>)
where
    T: DlpSerde,
{
    let mut xml_children = Vec::new();
    for child in children {
        let xml_child = child.serialize();
        xml_children.push(xmltree::XMLNode::Element(xml_child));
    }
    parent.children.extend_from_slice(&xml_children);
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

pub(crate) fn try_to_value<T>(parent: &mut xmltree::Element, field: &str, value: &Option<T>)
where
    T: ToString,
{
    let mut xml_child = xmltree::Element::new(field);
    xml_child.children.push(xmltree::XMLNode::Text(match value {
        Some(v) => v.to_string(),
        None => "".into(),
    }));
    parent.children.push(xmltree::XMLNode::Element(xml_child));
}

pub(crate) fn to_value<T>(parent: &mut xmltree::Element, field: &str, value: &T)
where
    T: ToString,
{
    let mut xml_child = xmltree::Element::new(field);
    xml_child
        .children
        .push(xmltree::XMLNode::Text(value.to_string()));
    parent.children.push(xmltree::XMLNode::Element(xml_child));
}

pub(crate) fn to_value_bool(parent: &mut xmltree::Element, field: &str, value: &bool) {
    let mut xml_child = xmltree::Element::new(field);
    xml_child.children.push(xmltree::XMLNode::Text(if *value {
        "1".into()
    } else {
        "0".into()
    }));
    parent.children.push(xmltree::XMLNode::Element(xml_child));
}
