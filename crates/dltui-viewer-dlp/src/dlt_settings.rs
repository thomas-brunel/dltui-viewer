use crate::{DlpSerde, get_value, to_value, to_value_bool, value_as, value_as_bool};

#[derive(Debug)]
pub struct DltTable {
    font_size: u8,
    section_size: u8,
    font_name: String,
    automatic_time_settings: bool,
    automatic_timezone_from_dlt: bool,
    utc_offset: i16,
    dst: bool,
    show_index: bool,
    show_time: bool,
    show_timestamp: bool,
    show_count: bool,
    show_ecu_id: bool,
    show_app_id: bool,
    show_app_id_description: bool,
    show_context_id: bool,
    show_context_id_description: bool,
    show_type: bool,
    show_subtype: bool,
    show_mode: bool,
    show_noar: bool,
    show_payload: bool,
    show_arguments: bool,
    show_msg_id: bool,
    marker_color: String, // Todo: create a Color enum ?
}

impl Default for DltTable {
    fn default() -> Self {
        Self {
            font_size: 8,
            section_size: 16,
            font_name: "Segoe UI,8".into(),
            automatic_time_settings: true,
            automatic_timezone_from_dlt: true,
            utc_offset: -1,
            dst: false,
            show_index: true,
            show_time: true,
            show_timestamp: true,
            show_count: false,
            show_ecu_id: true,
            show_app_id: true,
            show_app_id_description: false,
            show_context_id: true,
            show_context_id_description: false,
            show_type: true,
            show_subtype: false,
            show_mode: false,
            show_noar: false,
            show_payload: true,
            show_arguments: false,
            show_msg_id: false,
            marker_color: "#808080".into(),
        }
    }
}

impl DlpSerde for DltTable {
    fn deserialize(xml_table: &mut xmltree::Element) -> Result<Self, crate::Error> {
        let font_size = value_as(xml_table, "fontSize")?;
        let section_size = value_as(xml_table, "sectionSize")?;
        let font_name = get_value(xml_table, "fontName")?;
        let automatic_time_settings: bool = value_as_bool(xml_table, "automaticTimeSettings")?;
        let automatic_timezone_from_dlt = value_as_bool(xml_table, "automaticTimezoneFromDlt")?;
        let utc_offset: i16 = value_as(xml_table, "utcOffset")?;
        let dst = value_as_bool(xml_table, "dst")?;
        let show_index = value_as_bool(xml_table, "showIndex")?;
        let show_time = value_as_bool(xml_table, "showTime")?;
        let show_timestamp = value_as_bool(xml_table, "showTimestamp")?;
        let show_count = value_as_bool(xml_table, "showCount")?;
        let show_ecu_id = value_as_bool(xml_table, "showEcuId")?;
        let show_app_id = value_as_bool(xml_table, "showApId")?;
        let show_app_id_description = value_as_bool(xml_table, "showApIdDesc")?;
        let show_context_id = value_as_bool(xml_table, "showCtId")?;
        let show_context_id_description = value_as_bool(xml_table, "showCtIdDesc")?;
        let show_type = value_as_bool(xml_table, "showType")?;
        let show_subtype = value_as_bool(xml_table, "showSubtype")?;
        let show_mode = value_as_bool(xml_table, "showMode")?;
        let show_noar = value_as_bool(xml_table, "showNoar")?;
        let show_payload = value_as_bool(xml_table, "showPayload")?;
        let show_arguments = value_as_bool(xml_table, "showArguments")?;
        let show_msg_id = value_as_bool(xml_table, "showMsgId")?;
        let marker_color = get_value(xml_table, "markercolor")?;

        Ok(Self {
            font_size,
            section_size,
            font_name,
            automatic_time_settings,
            automatic_timezone_from_dlt,
            utc_offset,
            dst,
            show_index,
            show_time,
            show_timestamp,
            show_count,
            show_ecu_id,
            show_app_id,
            show_app_id_description,
            show_context_id,
            show_context_id_description,
            show_type,
            show_subtype,
            show_mode,
            show_noar,
            show_payload,
            show_arguments,
            show_msg_id,
            marker_color,
        })
    }

    fn serialize(&self) -> xmltree::Element {
        let mut xml_table = xmltree::Element::new("table");

        to_value(&mut xml_table, "fontSize", &self.font_size);
        to_value(&mut xml_table, "sectionSize", &self.section_size);
        to_value(&mut xml_table, "fontName", &self.font_name);
        to_value_bool(
            &mut xml_table,
            "automaticTimeSettings",
            &self.automatic_time_settings,
        );
        to_value_bool(
            &mut xml_table,
            "automaticTimezoneFromDlt",
            &self.automatic_timezone_from_dlt,
        );
        to_value(&mut xml_table, "utcOffset", &self.utc_offset);
        to_value_bool(&mut xml_table, "dst", &self.dst);
        to_value_bool(&mut xml_table, "showIndex", &self.show_index);
        to_value_bool(&mut xml_table, "showTime", &self.show_time);
        to_value_bool(&mut xml_table, "showTimestamp", &self.show_timestamp);
        to_value_bool(&mut xml_table, "showCount", &self.show_count);
        to_value_bool(&mut xml_table, "showEcuId", &self.show_ecu_id);
        to_value_bool(&mut xml_table, "showApId", &self.show_app_id);
        to_value_bool(
            &mut xml_table,
            "showApIdDesc",
            &self.show_app_id_description,
        );
        to_value_bool(&mut xml_table, "showCtId", &self.show_context_id);
        to_value_bool(
            &mut xml_table,
            "showCtIdDesc",
            &self.show_context_id_description,
        );
        to_value_bool(&mut xml_table, "showType", &self.show_type);
        to_value_bool(&mut xml_table, "showSubtype", &self.show_subtype);
        to_value_bool(&mut xml_table, "showMode", &self.show_mode);
        to_value_bool(&mut xml_table, "showNoar", &self.show_noar);
        to_value_bool(&mut xml_table, "showPayload", &self.show_payload);
        to_value_bool(&mut xml_table, "showArguments", &self.show_arguments);
        to_value_bool(&mut xml_table, "showMsgId", &self.show_msg_id);
        to_value(&mut xml_table, "markercolor", &self.marker_color);

        xml_table
    }
}

impl DltTable {
    pub fn new(
        automatic_time_settings: bool,
        automatic_timezone_from_dlt: bool,
        utc_offset: i16,
        dst: bool,
        show_index: bool,
        show_time: bool,
        show_timestamp: bool,
        show_count: bool,
        show_ecu_id: bool,
        show_app_id: bool,
        show_app_id_description: bool,
        show_context_id: bool,
        show_context_id_description: bool,
        show_type: bool,
        show_subtype: bool,
        show_mode: bool,
        show_noar: bool,
        show_payload: bool,
        show_arguments: bool,
        show_msg_id: bool,
        marker_color: String,
    ) -> Self {
        Self {
            automatic_time_settings,
            automatic_timezone_from_dlt,
            utc_offset,
            dst,
            show_index,
            show_time,
            show_timestamp,
            show_count,
            show_ecu_id,
            show_app_id,
            show_app_id_description,
            show_context_id,
            show_context_id_description,
            show_type,
            show_subtype,
            show_mode,
            show_noar,
            show_payload,
            show_arguments,
            show_msg_id,
            marker_color,
            ..Default::default()
        }
    }

    pub fn automatic_time_settings(&self) -> bool {
        self.automatic_time_settings
    }

    pub fn automatic_timezone_from_dlt(&self) -> bool {
        self.automatic_timezone_from_dlt
    }

    pub fn utc_offset(&self) -> i16 {
        self.utc_offset
    }

    pub fn dst(&self) -> bool {
        self.dst
    }

    pub fn show_index(&self) -> bool {
        self.show_index
    }

    pub fn show_time(&self) -> bool {
        self.show_time
    }

    pub fn show_timestamp(&self) -> bool {
        self.show_timestamp
    }

    pub fn show_count(&self) -> bool {
        self.show_count
    }

    pub fn show_ecu_id(&self) -> bool {
        self.show_ecu_id
    }

    pub fn show_app_id(&self) -> bool {
        self.show_app_id
    }

    pub fn show_app_id_description(&self) -> bool {
        self.show_app_id_description
    }

    pub fn show_context_id(&self) -> bool {
        self.show_context_id
    }

    pub fn show_context_id_description(&self) -> bool {
        self.show_context_id_description
    }

    pub fn show_type(&self) -> bool {
        self.show_type
    }

    pub fn show_subtype(&self) -> bool {
        self.show_subtype
    }

    pub fn show_mode(&self) -> bool {
        self.show_mode
    }

    pub fn show_noar(&self) -> bool {
        self.show_noar
    }

    pub fn show_payload(&self) -> bool {
        self.show_payload
    }

    pub fn show_arguments(&self) -> bool {
        self.show_arguments
    }

    pub fn show_msg_id(&self) -> bool {
        self.show_msg_id
    }

    pub fn marker_color(&self) -> &str {
        &self.marker_color
    }

    pub fn set_automatic_time_settings(&mut self, v: bool) {
        self.automatic_time_settings = v;
    }

    pub fn set_automatic_timezone_from_dlt(&mut self, v: bool) {
        self.automatic_timezone_from_dlt = v;
    }

    pub fn set_utc_offset(&mut self, utc_offset: i16) {
        self.utc_offset = utc_offset;
    }

    pub fn set_dst(&mut self, dst: bool) {
        self.dst = dst;
    }

    pub fn set_show_index(&mut self, v: bool) {
        self.show_index = v;
    }

    pub fn set_show_time(&mut self, v: bool) {
        self.show_time = v;
    }

    pub fn set_show_timestamp(&mut self, v: bool) {
        self.show_timestamp = v;
    }

    pub fn set_show_count(&mut self, v: bool) {
        self.show_count = v;
    }

    pub fn set_show_ecu_id(&mut self, v: bool) {
        self.show_ecu_id = v;
    }

    pub fn set_show_app_id(&mut self, v: bool) {
        self.show_app_id = v;
    }

    pub fn set_show_app_id_description(&mut self, v: bool) {
        self.show_app_id_description = v;
    }

    pub fn set_show_context_id(&mut self, v: bool) {
        self.show_context_id = v;
    }

    pub fn set_show_context_id_description(&mut self, v: bool) {
        self.show_context_id_description = v;
    }

    pub fn set_show_type(&mut self, v: bool) {
        self.show_type = v;
    }

    pub fn set_show_subtype(&mut self, v: bool) {
        self.show_subtype = v;
    }

    pub fn set_show_mode(&mut self, v: bool) {
        self.show_mode = v;
    }

    pub fn set_show_noar(&mut self, v: bool) {
        self.show_noar = v;
    }

    pub fn set_show_payload(&mut self, v: bool) {
        self.show_payload = v;
    }

    pub fn set_show_arguments(&mut self, v: bool) {
        self.show_arguments = v;
    }

    pub fn set_show_msg_id(&mut self, v: bool) {
        self.show_msg_id = v;
    }

    pub fn set_marker_color(&mut self, marker_color: String) {
        self.marker_color = marker_color;
    }
}

#[derive(Debug)]
pub struct DltOther {
    auto_connect: bool,
    auto_scroll: bool,
    auto_mark_fatal_error: bool,
    auto_mark_warn: bool,
    auto_mark_marker: bool,
    update_context_loading_file: bool,
    update_contexts_unregister: bool,
    logging_only_mode: bool,
    split_log_file: bool,
    fmax_file_size_mb: u16,
    append_date_time: bool,
    msg_id_format: String,
}

impl Default for DltOther {
    fn default() -> Self {
        Self {
            auto_connect: false,
            auto_scroll: true,
            auto_mark_fatal_error: false,
            auto_mark_warn: false,
            auto_mark_marker: true,
            update_context_loading_file: true,
            update_contexts_unregister: false,
            logging_only_mode: false,
            split_log_file: false,
            fmax_file_size_mb: 100,
            append_date_time: false,
            msg_id_format: "[%08u]".into(),
        }
    }
}

impl DlpSerde for DltOther {
    fn deserialize(xml_other: &mut xmltree::Element) -> Result<Self, crate::Error> {
        let auto_connect = value_as_bool(xml_other, "autoConnect")?;
        let auto_scroll = value_as_bool(xml_other, "autoScroll")?;
        let auto_mark_fatal_error = value_as_bool(xml_other, "autoMarkFatalError")?;
        let auto_mark_warn = value_as_bool(xml_other, "autoMarkWarn")?;
        let auto_mark_marker = value_as_bool(xml_other, "autoMarkMarker")?;
        let update_context_loading_file = value_as_bool(xml_other, "updateContextLoadingFile")?;
        let update_contexts_unregister = value_as_bool(xml_other, "updateContextsUnregister")?;
        let logging_only_mode = value_as_bool(xml_other, "loggingOnlyMode")?;
        let split_log_file = value_as_bool(xml_other, "splitlogfile")?;
        let fmax_file_size_mb = value_as(xml_other, "fmaxFileSizeMB")?;
        let append_date_time = value_as_bool(xml_other, "appendDateTime")?;
        let msg_id_format = get_value(xml_other, "msgIdFormat")?;

        Ok(Self {
            auto_connect,
            auto_scroll,
            auto_mark_fatal_error,
            auto_mark_warn,
            auto_mark_marker,
            update_context_loading_file,
            update_contexts_unregister,
            logging_only_mode,
            split_log_file,
            fmax_file_size_mb,
            append_date_time,
            msg_id_format,
        })
    }

    fn serialize(&self) -> xmltree::Element {
        let mut xml_other = xmltree::Element::new("other");

        to_value_bool(&mut xml_other, "autoConnect", &self.auto_connect);
        to_value_bool(&mut xml_other, "autoScroll", &self.auto_scroll);
        to_value_bool(
            &mut xml_other,
            "autoMarkFatalError",
            &self.auto_mark_fatal_error,
        );
        to_value_bool(&mut xml_other, "autoMarkWarn", &self.auto_mark_warn);
        to_value_bool(&mut xml_other, "autoMarkMarker", &self.auto_mark_marker);
        to_value_bool(
            &mut xml_other,
            "updateContextLoadingFile",
            &self.update_context_loading_file,
        );
        to_value_bool(
            &mut xml_other,
            "updateContextsUnregister",
            &self.update_contexts_unregister,
        );
        to_value_bool(&mut xml_other, "loggingOnlyMode", &self.logging_only_mode);
        to_value_bool(&mut xml_other, "splitlogfile", &self.split_log_file);
        to_value(&mut xml_other, "fmaxFileSizeMB", &self.fmax_file_size_mb);
        to_value_bool(&mut xml_other, "appendDateTime", &self.append_date_time);
        to_value(&mut xml_other, "msgIdFormat", &self.msg_id_format);

        xml_other
    }
}

impl DltOther {
    pub fn new(
        auto_connect: bool,
        auto_scroll: bool,
        auto_mark_fatal_error: bool,
        auto_mark_warn: bool,
        auto_mark_marker: bool,
        update_context_loading_file: bool,
        update_contexts_unregister: bool,
        logging_only_mode: bool,
        split_log_file: bool,
        fmax_file_size_mb: u16,
        append_date_time: bool,
        msg_id_format: String,
    ) -> Self {
        Self {
            auto_connect,
            auto_scroll,
            auto_mark_fatal_error,
            auto_mark_warn,
            auto_mark_marker,
            update_context_loading_file,
            update_contexts_unregister,
            logging_only_mode,
            split_log_file,
            fmax_file_size_mb,
            append_date_time,
            msg_id_format,
        }
    }

    pub fn auto_connect(&self) -> bool {
        self.auto_connect
    }

    pub fn auto_scroll(&self) -> bool {
        self.auto_scroll
    }

    pub fn auto_mark_fatal_error(&self) -> bool {
        self.auto_mark_fatal_error
    }

    pub fn auto_mark_warn(&self) -> bool {
        self.auto_mark_warn
    }

    pub fn auto_mark_marker(&self) -> bool {
        self.auto_mark_marker
    }

    pub fn update_context_loading_file(&self) -> bool {
        self.update_context_loading_file
    }

    pub fn update_contexts_unregister(&self) -> bool {
        self.update_contexts_unregister
    }

    pub fn logging_only_mode(&self) -> bool {
        self.logging_only_mode
    }

    pub fn split_log_file(&self) -> bool {
        self.split_log_file
    }

    pub fn fmax_file_size_mb(&self) -> u16 {
        self.fmax_file_size_mb
    }

    pub fn append_date_time(&self) -> bool {
        self.append_date_time
    }

    pub fn msg_id_format(&self) -> &str {
        &self.msg_id_format
    }

    pub fn set_auto_connect(&mut self, v: bool) {
        self.auto_connect = v;
    }

    pub fn set_auto_scroll(&mut self, v: bool) {
        self.auto_scroll = v;
    }

    pub fn set_auto_mark_fatal_error(&mut self, v: bool) {
        self.auto_mark_fatal_error = v;
    }

    pub fn set_auto_mark_warn(&mut self, v: bool) {
        self.auto_mark_warn = v;
    }

    pub fn set_auto_mark_marker(&mut self, v: bool) {
        self.auto_mark_marker = v;
    }

    pub fn set_update_context_loading_file(&mut self, v: bool) {
        self.update_context_loading_file = v;
    }

    pub fn set_update_contexts_unregister(&mut self, v: bool) {
        self.update_contexts_unregister = v;
    }

    pub fn set_logging_only_mode(&mut self, v: bool) {
        self.logging_only_mode = v;
    }

    pub fn set_split_log_file(&mut self, v: bool) {
        self.split_log_file = v;
    }

    pub fn set_fmax_file_size_mb(&mut self, v: u16) {
        self.fmax_file_size_mb = v;
    }

    pub fn set_append_date_time(&mut self, v: bool) {
        self.append_date_time = v;
    }

    pub fn set_msg_id_format(&mut self, v: String) {
        self.msg_id_format = v;
    }

    pub fn toggle_auto_scroll(&mut self) {
        self.auto_scroll = !self.auto_scroll;
    }
}

#[derive(Debug)]
pub struct DltSettings {
    table: DltTable,
    other: DltOther,
}

impl Default for DltSettings {
    fn default() -> Self {
        Self {
            table: DltTable::default(),
            other: DltOther::default(),
        }
    }
}

impl DlpSerde for DltSettings {
    fn deserialize(xml_settings: &mut xmltree::Element) -> Result<Self, crate::Error> {
        let dlt_table = match xml_settings.take_child("table") {
            Some(mut el) => DltTable::deserialize(&mut el)?,
            None => {
                return Err(crate::Error::FieldMissing("table".to_string()));
            }
        };

        let dlt_other = match xml_settings.take_child("other") {
            Some(mut el) => DltOther::deserialize(&mut el)?,
            None => {
                return Err(crate::Error::FieldMissing("other".to_string()));
            }
        };

        Ok(Self {
            table: dlt_table,
            other: dlt_other,
        })
    }

    fn serialize(&self) -> xmltree::Element {
        let mut xml_settings = xmltree::Element::new("settings");

        let xml_table = self.table.serialize();
        let xml_other = self.other.serialize();

        xml_settings
            .children
            .push(xmltree::XMLNode::Element(xml_table));
        xml_settings
            .children
            .push(xmltree::XMLNode::Element(xml_other));

        xml_settings
    }
}

impl DltSettings {
    pub fn new(table: DltTable, other: DltOther) -> Self {
        Self { table, other }
    }

    pub fn table(&self) -> &DltTable {
        &self.table
    }

    pub fn other(&self) -> &DltOther {
        &self.other
    }

    pub fn table_mut(&mut self) -> &mut DltTable {
        &mut self.table
    }

    pub fn other_mut(&mut self) -> &mut DltOther {
        &mut self.other
    }
}
