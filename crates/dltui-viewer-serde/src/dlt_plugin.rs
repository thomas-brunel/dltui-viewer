use crate::{DlpSerde, get_value, to_value, try_get_value, try_to_value, value_as};

#[derive(Debug)]
pub struct DltPlugin {
    name: String,
    filename: Option<String>,
    mode: u8,
    pg_type: u8,
    prio: u8,
}

impl DlpSerde for DltPlugin {
    fn deserialize(xml_plugin: &mut xmltree::Element) -> Result<Self, crate::Error> {
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

    fn serialize(&self) -> xmltree::Element {
        let mut xml_plugin = xmltree::Element::new("plugin");

        to_value(&mut xml_plugin, "name", &self.name);
        try_to_value(&mut xml_plugin, "filename", &self.filename);
        to_value(&mut xml_plugin, "mode", &self.mode);
        to_value(&mut xml_plugin, "type", &self.pg_type);
        to_value(&mut xml_plugin, "prio", &self.prio);

        xml_plugin
    }
}
