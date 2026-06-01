use crate::{get_value, try_get_value, value_as};

#[derive(Debug)]
pub struct DltPlugin {
    name: String,
    filename: Option<String>,
    mode: u8,
    pg_type: u8,
    prio: u8,
}

impl DltPlugin {
    pub(crate) fn deserialize(xml_plugin: &xmltree::Element) -> Result<Self, crate::Error> {
        let name = get_value(xml_plugin, "name")?;
        let filename = try_get_value(xml_plugin, "filename")?;
        let mode = value_as(xml_plugin, "mode")?;
        let pg_type = value_as(xml_plugin, "type")?;
        let prio = value_as(xml_plugin, "prio")?;

        Ok(Self {
            name,
            filename,
            mode,
            pg_type,
            prio,
        })
    }
}
