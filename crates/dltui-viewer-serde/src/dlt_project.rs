use crate::{
    deserialize_child, deserialize_children, deserialize_mut_children,
    dlt_ecu::{self, DltEcu},
    dlt_filter::{self, DltFilter},
    dlt_plugin::{self, DltPlugin},
    dlt_settings::{self, DltSettings},
};
use std::io::Read;

#[derive(Debug)]
pub struct DltProject {
    path: String,
    settings: DltSettings,
    ecus: Vec<DltEcu>,
    filters: Vec<DltFilter>,
    plugins: Vec<DltPlugin>,
}

impl DltProject {
    fn deserialize(
        path: &str,
        xml_dlt_project: &mut xmltree::Element,
    ) -> Result<Self, crate::Error> {
        let dlt_settings = deserialize_child(
            xml_dlt_project,
            "settings",
            &dlt_settings::DltSettings::deserialize,
        )?;

        let ecus = deserialize_mut_children(xml_dlt_project, "ecu", &dlt_ecu::DltEcu::deserialize)?;

        let filters = deserialize_children(
            xml_dlt_project,
            "pfilter",
            &dlt_filter::DltFilter::deserialize,
        )?;

        let plugins = deserialize_children(
            xml_dlt_project,
            "plugin",
            &dlt_plugin::DltPlugin::deserialize,
        )?;

        Ok(Self {
            path: path.to_string(),
            settings: dlt_settings,
            ecus: ecus,
            filters: filters,
            plugins: plugins,
        })
    }

    pub fn open(path: &str) -> Result<Self, crate::Error> {
        if !path.ends_with(".dlp") {
            return Err(crate::Error::UnsupportedExtension);
        }

        let mut file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(xmltree::Error::Io(e).into()),
        };

        let mut file_data = String::new();
        match file.read_to_string(&mut file_data) {
            Ok(nb_read) => println!("Read {} bytes", nb_read),
            Err(e) => return Err(xmltree::Error::Io(e).into()),
        };

        let mut xml_dlt_project = xmltree::Element::parse(file_data.as_bytes())?;

        Self::deserialize(path, &mut xml_dlt_project)
    }

    // pub fn save(&self) -> Result<(), xmltree::Error> {
    //     let file = match std::fs::File::open(self.path.as_str()) {
    //         Ok(f) => f,
    //         Err(e) => return Err(xmltree::Error::Io(e)),
    //     };

    //     self.xml_data.write(file)
    // }

    // pub fn save_as(&mut self, new_path: &'static str) -> Result<(), xmltree::Error> {
    //     let file = match std::fs::File::create_new(new_path) {
    //         Ok(f) => f,
    //         Err(e) => return Err(xmltree::Error::Io(e)),
    //     };

    //     // self.xml_data.write(file)?;

    //     self.path = new_path.to_string();

    //     Ok(())
    // }
}
