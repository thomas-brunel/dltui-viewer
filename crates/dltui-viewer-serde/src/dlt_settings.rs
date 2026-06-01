use crate::{get_value, value_as, value_as_bool};

#[derive(Debug)]
struct DltTable {
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

impl DltTable {
    fn deserialize(xml_table: &xmltree::Element) -> Result<Self, crate::Error> {
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
}

#[derive(Debug)]
struct DltOther {
    auto_connect: bool,
    auto_scroll: i8,
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

impl DltOther {
    fn deserialize(xml_other: &xmltree::Element) -> Result<Self, crate::Error> {
        let auto_connect = value_as_bool(xml_other, "autoConnect")?;
        let auto_scroll = value_as(xml_other, "autoScroll")?;
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
}

#[derive(Debug)]
pub struct DltSettings {
    table: DltTable,
    other: DltOther,
}

impl DltSettings {
    pub(crate) fn deserialize(xml_settings: &xmltree::Element) -> Result<Self, crate::Error> {
        let dlt_table = match xml_settings.get_child("table") {
            Some(el) => DltTable::deserialize(el)?,
            None => {
                return Err(crate::Error::FieldMissing("table".to_string()));
            }
        };

        let dlt_other = match xml_settings.get_child("other") {
            Some(el) => DltOther::deserialize(el)?,
            None => {
                return Err(crate::Error::FieldMissing("other".to_string()));
            }
        };

        Ok(Self {
            table: dlt_table,
            other: dlt_other,
        })
    }
}
