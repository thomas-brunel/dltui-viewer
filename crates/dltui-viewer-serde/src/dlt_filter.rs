use crate::{DlpSerde, get_value, try_get_value, value_as, value_as_bool};

#[derive(Debug)]
pub struct DltFilter {
    filter_type: i8,
    name: String,
    ecu_id: Option<String>,
    application_id: Option<String>,
    context_id: Option<String>,
    header_text: Option<String>,
    payload_text: Option<String>,
    regex_search: Option<String>,
    regex_replace: Option<String>,
    message_id_min: u8,
    message_id_max: u8,
    enable_reg_exp_app_id: bool,
    enable_reg_exp_context: bool,
    enable_reg_exp_header: bool,
    enable_reg_exp_payload: bool,
    ignore_case_header: bool,
    ignore_case_payload: bool,
    enable_filter: bool,
    enable_ecu_id: bool,
    enable_application_id: bool,
    enable_context_id: bool,
    enable_header_text: bool,
    enable_payload_text: bool,
    enable_ctrl_msgs: bool,
    enable_log_level_min: bool,
    enable_log_level_max: bool,
    enable_marker: bool,
    enable_message_id: bool,
    enable_regex_search_replace: bool,
    filter_colour: String,
    log_level_max: u8,
    log_level_min: u8,
}

impl DlpSerde for DltFilter {
    fn deserialize(xml_filter: &mut xmltree::Element) -> Result<Self, crate::Error> {
        let filter_type = value_as(xml_filter, "type")?;
        let name = get_value(xml_filter, "name")?;
        let ecu_id = try_get_value(xml_filter, "ecuid")?;
        let application_id = try_get_value(xml_filter, "applicationid")?;
        let context_id = try_get_value(xml_filter, "contextid")?;
        let header_text = try_get_value(xml_filter, "headertext")?;
        let payload_text = try_get_value(xml_filter, "payloadtext")?;
        let regex_search = try_get_value(xml_filter, "regex_search")?;
        let regex_replace = try_get_value(xml_filter, "regex_replace")?;
        let message_id_min = value_as(xml_filter, "messageIdMin")?;
        let message_id_max = value_as(xml_filter, "messageIdMax")?;
        let enable_reg_exp_app_id = value_as_bool(xml_filter, "enableregexp_Appid")?;
        let enable_reg_exp_context = value_as_bool(xml_filter, "enableregexp_Context")?;
        let enable_reg_exp_header = value_as_bool(xml_filter, "enableregexp_Header")?;
        let enable_reg_exp_payload = value_as_bool(xml_filter, "enableregexp_Payload")?;
        let ignore_case_header = value_as_bool(xml_filter, "ignoreCase_Header")?;
        let ignore_case_payload = value_as_bool(xml_filter, "ignoreCase_Payload")?;
        let enable_filter = value_as_bool(xml_filter, "enablefilter")?;
        let enable_ecu_id = value_as_bool(xml_filter, "enableecuid")?;
        let enable_application_id = value_as_bool(xml_filter, "enableapplicationid")?;
        let enable_context_id = value_as_bool(xml_filter, "enablecontextid")?;
        let enable_header_text = value_as_bool(xml_filter, "enableheadertext")?;
        let enable_payload_text = value_as_bool(xml_filter, "enablepayloadtext")?;
        let enable_ctrl_msgs = value_as_bool(xml_filter, "enablectrlmsgs")?;
        let enable_log_level_min = value_as_bool(xml_filter, "enableLogLevelMin")?;
        let enable_log_level_max = value_as_bool(xml_filter, "enableLogLevelMax")?;
        let enable_marker = value_as_bool(xml_filter, "enableMarker")?;
        let enable_message_id = value_as_bool(xml_filter, "enableMessageId")?;
        let enable_regex_search_replace = value_as_bool(xml_filter, "enableRegexSearchReplace")?;
        let filter_colour = get_value(xml_filter, "filterColour")?;
        let log_level_max = value_as(xml_filter, "logLevelMax")?;
        let log_level_min = value_as(xml_filter, "logLevelMin")?;

        Ok(Self {
            filter_type,
            name,
            ecu_id,
            application_id,
            context_id,
            header_text,
            payload_text,
            regex_search,
            regex_replace,
            message_id_min,
            message_id_max,
            enable_reg_exp_app_id,
            enable_reg_exp_context,
            enable_reg_exp_header,
            enable_reg_exp_payload,
            ignore_case_header,
            ignore_case_payload,
            enable_filter,
            enable_ecu_id,
            enable_application_id,
            enable_context_id,
            enable_header_text,
            enable_payload_text,
            enable_ctrl_msgs,
            enable_log_level_min,
            enable_log_level_max,
            enable_marker,
            enable_message_id,
            enable_regex_search_replace,
            filter_colour,
            log_level_max,
            log_level_min,
        })
    }

    fn serialize(&self) -> xmltree::Element {
        let mut xml_filter = xmltree::Element::new("pfilter");

        crate::to_value(&mut xml_filter, "type", &self.filter_type);
        crate::to_value(&mut xml_filter, "name", &self.name);
        crate::try_to_value(&mut xml_filter, "ecuid", &self.ecu_id);
        crate::try_to_value(&mut xml_filter, "applicationid", &self.application_id);
        crate::try_to_value(&mut xml_filter, "contextid", &self.context_id);
        crate::try_to_value(&mut xml_filter, "headertext", &self.header_text);
        crate::try_to_value(&mut xml_filter, "payloadtext", &self.payload_text);
        crate::try_to_value(&mut xml_filter, "regex_search", &self.regex_search);
        crate::try_to_value(&mut xml_filter, "regex_replace", &self.regex_replace);
        crate::to_value(&mut xml_filter, "messageIdMin", &self.message_id_min);
        crate::to_value(&mut xml_filter, "messageIdMax", &self.message_id_max);
        crate::to_value_bool(
            &mut xml_filter,
            "enableregexp_Appid",
            &self.enable_reg_exp_app_id,
        );
        crate::to_value_bool(
            &mut xml_filter,
            "enableregexp_Context",
            &self.enable_reg_exp_context,
        );
        crate::to_value_bool(
            &mut xml_filter,
            "enableregexp_Header",
            &self.enable_reg_exp_header,
        );
        crate::to_value_bool(
            &mut xml_filter,
            "enableregexp_Payload",
            &self.enable_reg_exp_payload,
        );
        crate::to_value_bool(
            &mut xml_filter,
            "ignoreCase_Header",
            &self.ignore_case_header,
        );
        crate::to_value_bool(
            &mut xml_filter,
            "ignoreCase_Payload",
            &self.ignore_case_payload,
        );
        crate::to_value_bool(&mut xml_filter, "enablefilter", &self.enable_filter);
        crate::to_value_bool(&mut xml_filter, "enableecuid", &self.enable_ecu_id);
        crate::to_value_bool(
            &mut xml_filter,
            "enableapplicationid",
            &self.enable_application_id,
        );
        crate::to_value_bool(&mut xml_filter, "enablecontextid", &self.enable_context_id);
        crate::to_value_bool(
            &mut xml_filter,
            "enableheadertext",
            &self.enable_header_text,
        );
        crate::to_value_bool(
            &mut xml_filter,
            "enablepayloadtext",
            &self.enable_payload_text,
        );
        crate::to_value_bool(&mut xml_filter, "enablectrlmsgs", &self.enable_ctrl_msgs);
        crate::to_value_bool(
            &mut xml_filter,
            "enableLogLevelMin",
            &self.enable_log_level_min,
        );
        crate::to_value_bool(
            &mut xml_filter,
            "enableLogLevelMax",
            &self.enable_log_level_max,
        );
        crate::to_value_bool(&mut xml_filter, "enableMarker", &self.enable_marker);
        crate::to_value_bool(&mut xml_filter, "enableMessageId", &self.enable_message_id);
        crate::to_value_bool(
            &mut xml_filter,
            "enableRegexSearchReplace",
            &self.enable_regex_search_replace,
        );
        crate::to_value(&mut xml_filter, "filterColour", &self.filter_colour);
        crate::to_value(&mut xml_filter, "logLevelMax", &self.log_level_max);
        crate::to_value(&mut xml_filter, "logLevelMin", &self.log_level_min);

        xml_filter
    }
}
