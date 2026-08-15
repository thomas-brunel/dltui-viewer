use crate::{
    DlpSerde, deserialize_children, dlt_context::DltContext, get_value, serialize_children,
    to_value, try_get_value, try_to_value,
};

#[derive(Debug)]
pub struct DltApplication {
    id: String,
    description: Option<String>,
    contexts: Vec<DltContext>,
}

impl DlpSerde for DltApplication {
    fn deserialize(xml_application: &mut xmltree::Element) -> Result<Self, crate::Error> {
        let id = get_value(xml_application, "id")?;
        let description = try_get_value(xml_application, "description")?;
        let contexts = deserialize_children(xml_application, "context")?;

        Ok(Self {
            id,
            description,
            contexts,
        })
    }

    fn serialize(&self) -> xmltree::Element {
        let mut xml_application = xmltree::Element::new("application");

        to_value(&mut xml_application, "id", &self.id);
        try_to_value(&mut xml_application, "description", &self.description);
        serialize_children(&mut xml_application, &self.contexts);

        xml_application
    }
}
