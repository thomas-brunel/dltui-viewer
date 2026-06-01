use crate::{get_value, try_get_value, value_as};

#[derive(Debug)]
pub struct DltContext {
    id: String,
    description: Option<String>,
    log_level: i8,
    trace_status: i8,
}

impl DltContext {
    pub(crate) fn deserialize(xml_context: &xmltree::Element) -> Result<Self, crate::Error> {
        let id = get_value(xml_context, "id")?;
        let description = try_get_value(xml_context, "description")?;
        let log_level = value_as(xml_context, "loglevel")?;
        let trace_status = value_as(xml_context, "tracestatus")?;

        Ok(Self {
            id,
            description,
            log_level,
            trace_status,
        })
    }
}
