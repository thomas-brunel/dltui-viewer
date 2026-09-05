use crate::{DlpSerde, get_value, to_value, try_get_value, try_to_value, value_as};

#[derive(Debug)]
pub struct DltContext {
    id: String,
    description: Option<String>,
    log_level: i8,
    trace_status: i8,
}

impl DlpSerde for DltContext {
    fn deserialize(xml_context: &mut xmltree::Element) -> Result<Self, crate::Error> {
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

    fn serialize(&self) -> xmltree::Element {
        let mut xml_context = xmltree::Element::new("context");

        to_value(&mut xml_context, "id", &self.id);
        try_to_value(&mut xml_context, "description", &self.description);
        to_value(&mut xml_context, "loglevel", &self.log_level);
        to_value(&mut xml_context, "tracestatus", &self.trace_status);

        xml_context
    }
}

impl DltContext {
    pub fn new(id: String, description: Option<String>, log_level: i8, trace_status: i8) -> Self {
        Self {
            id,
            description,
            log_level,
            trace_status,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn log_level(&self) -> i8 {
        self.log_level
    }

    pub fn trace_status(&self) -> i8 {
        self.trace_status
    }

    pub fn set_log_level(&mut self, log_level: i8) {
        self.log_level = log_level;
    }

    pub fn set_trace_status(&mut self, trace_status: i8) {
        self.trace_status = trace_status;
    }
}
