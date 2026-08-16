use crate::{
    DlpSerde, deserialize_child, deserialize_children, dlt_ecu::DltEcu, dlt_filter::DltFilter,
    dlt_plugin::DltPlugin, dlt_settings::DltSettings, serialize_child, serialize_children,
};
use std::{io::Read, path::PathBuf};

#[derive(Debug)]
pub struct DltProject {
    path: PathBuf,
    pub settings: DltSettings,
    pub ecus: Vec<DltEcu>,
    pub filters: Vec<DltFilter>,
    plugins: Vec<DltPlugin>,
}

impl Default for DltProject {
    fn default() -> Self {
        Self {
            path: "".into(),
            settings: DltSettings::default(),
            ecus: Vec::new(),
            filters: Vec::new(),
            plugins: Vec::new(),
        }
    }
}

impl DltProject {
    pub fn open(path: &PathBuf) -> Result<Self, crate::Error> {
        match path.extension() {
            Some(ext) => {
                if !ext.eq_ignore_ascii_case("dlp") {
                    return Err(crate::Error::UnsupportedExtension);
                }
            }
            None => {
                return Err(crate::Error::MissingFileExtension);
            }
        }

        let mut file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(xmltree::Error::Io(e))?,
        };

        let mut file_data = String::new();
        match file.read_to_string(&mut file_data) {
            Ok(nb_read) => println!("Read {} bytes", nb_read),
            Err(e) => return Err(xmltree::Error::Io(e))?,
        };

        let mut xml_dlt_project = xmltree::Element::parse(file_data.as_bytes())?;

        let dlt_project = Self::deserialize(&mut xml_dlt_project)?.with_path(path);
        Ok(dlt_project)
    }

    pub fn save(&self) -> Result<(), crate::Error> {
        let file = match std::fs::File::open(&self.path) {
            Ok(f) => f,
            Err(e) => return Err(xmltree::Error::Io(e))?,
        };

        self.write_dlp(file, true)
    }

    pub fn save_as(&mut self, new_path: &PathBuf) -> Result<(), crate::Error> {
        let file = match std::fs::File::create(new_path) {
            Ok(f) => f,
            Err(e) => return Err(xmltree::Error::Io(e))?,
        };

        self.write_dlp(file, true)?;
        self.set_path(new_path);

        Ok(())
    }

    fn write_dlp(&self, file: std::fs::File, pretty: bool) -> Result<(), crate::Error> {
        let xml_dlt_project = self.serialize();
        let emitter_config = xmltree::EmitterConfig::new().perform_indent(pretty);
        xml_dlt_project.write_with_config(file, emitter_config)?;
        Ok(())
    }

    fn with_path(mut self, path: &PathBuf) -> Self {
        self.path = path.into();
        self
    }

    fn set_path(&mut self, new_path: &PathBuf) -> &mut Self {
        self.path = new_path.into();
        self
    }
}

impl DlpSerde for DltProject {
    fn deserialize(xml_dlt_project: &mut xmltree::Element) -> Result<Self, crate::Error> {
        let dlt_settings = deserialize_child(xml_dlt_project, "settings")?;
        let ecus = deserialize_children(xml_dlt_project, "ecu")?;
        let filters = deserialize_children(xml_dlt_project, "pfilter")?;
        let plugins = deserialize_children(xml_dlt_project, "plugin")?;

        Ok(Self {
            path: "".into(),
            settings: dlt_settings,
            ecus: ecus,
            filters: filters,
            plugins: plugins,
        })
    }

    fn serialize(&self) -> xmltree::Element {
        let mut xml_dlt_project = xmltree::Element::new("dltproject");

        serialize_child(&mut xml_dlt_project, &self.settings);
        serialize_children(&mut xml_dlt_project, &self.ecus);
        serialize_children(&mut xml_dlt_project, &self.filters);
        serialize_children(&mut xml_dlt_project, &self.plugins);

        xml_dlt_project
    }
}
