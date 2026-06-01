use crate::{dlt_context::DltContext, get_value, try_get_value};

#[derive(Debug)]
pub struct DltApplication {
    id: String,
    description: Option<String>,
    contexts: Vec<DltContext>,
}

impl DltApplication {
    pub(crate) fn deserialize(
        xml_application: &mut xmltree::Element,
    ) -> Result<Self, crate::Error> {
        let id = get_value(xml_application, "id")?;
        let description = try_get_value(xml_application, "description")?;

        let mut contexts = Vec::new();
        while let Some(xml_context) = xml_application.take_child("context") {
            let dlt_context = DltContext::deserialize(&xml_context)?;
            contexts.push(dlt_context);
        }

        Ok(Self {
            id,
            description,
            contexts,
        })
    }
}
