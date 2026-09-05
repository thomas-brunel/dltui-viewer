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

impl DltFilter {
    pub fn new(
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
    ) -> Self {
        Self {
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
        }
    }

    pub fn filter_type(&self) -> i8 {
        self.filter_type
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ecu_id(&self) -> Option<&str> {
        self.ecu_id.as_deref()
    }

    pub fn application_id(&self) -> Option<&str> {
        self.application_id.as_deref()
    }

    pub fn context_id(&self) -> Option<&str> {
        self.context_id.as_deref()
    }

    pub fn header_text(&self) -> Option<&str> {
        self.header_text.as_deref()
    }

    pub fn payload_text(&self) -> Option<&str> {
        self.payload_text.as_deref()
    }

    pub fn regex_search(&self) -> Option<&str> {
        self.regex_search.as_deref()
    }

    pub fn regex_replace(&self) -> Option<&str> {
        self.regex_replace.as_deref()
    }

    pub fn message_id_min(&self) -> u8 {
        self.message_id_min
    }

    pub fn message_id_max(&self) -> u8 {
        self.message_id_max
    }

    pub fn enable_reg_exp_app_id(&self) -> bool {
        self.enable_reg_exp_app_id
    }

    pub fn enable_reg_exp_context(&self) -> bool {
        self.enable_reg_exp_context
    }

    pub fn enable_reg_exp_header(&self) -> bool {
        self.enable_reg_exp_header
    }

    pub fn enable_reg_exp_payload(&self) -> bool {
        self.enable_reg_exp_payload
    }

    pub fn ignore_case_header(&self) -> bool {
        self.ignore_case_header
    }

    pub fn ignore_case_payload(&self) -> bool {
        self.ignore_case_payload
    }

    pub fn enable_filter(&self) -> bool {
        self.enable_filter
    }

    pub fn enable_ecu_id(&self) -> bool {
        self.enable_ecu_id
    }

    pub fn enable_application_id(&self) -> bool {
        self.enable_application_id
    }

    pub fn enable_context_id(&self) -> bool {
        self.enable_context_id
    }

    pub fn enable_header_text(&self) -> bool {
        self.enable_header_text
    }

    pub fn enable_payload_text(&self) -> bool {
        self.enable_payload_text
    }

    pub fn enable_ctrl_msgs(&self) -> bool {
        self.enable_ctrl_msgs
    }

    pub fn enable_log_level_min(&self) -> bool {
        self.enable_log_level_min
    }

    pub fn enable_log_level_max(&self) -> bool {
        self.enable_log_level_max
    }

    pub fn enable_marker(&self) -> bool {
        self.enable_marker
    }

    pub fn enable_message_id(&self) -> bool {
        self.enable_message_id
    }

    pub fn enable_regex_search_replace(&self) -> bool {
        self.enable_regex_search_replace
    }

    pub fn filter_colour(&self) -> &str {
        &self.filter_colour
    }

    pub fn log_level_max(&self) -> u8 {
        self.log_level_max
    }

    pub fn log_level_min(&self) -> u8 {
        self.log_level_min
    }

    pub fn set_fitler_type(&mut self, filter_type: i8) {
        self.filter_type = filter_type;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_ecu_id(&mut self, ecu_id: Option<String>) {
        self.ecu_id = ecu_id;
    }

    pub fn set_application_id(&mut self, application_id: Option<String>) {
        self.application_id = application_id;
    }

    pub fn set_context_id(&mut self, context_id: Option<String>) {
        self.context_id = context_id;
    }

    pub fn set_header_text(&mut self, header_text: Option<String>) {
        self.header_text = header_text;
    }

    pub fn set_payload_text(&mut self, payload_text: Option<String>) {
        self.payload_text = payload_text;
    }

    pub fn set_regex_search(&mut self, regex_search: Option<String>) {
        self.regex_search = regex_search;
    }

    pub fn set_regex_replace(&mut self, regex_replace: Option<String>) {
        self.regex_replace = regex_replace;
    }

    pub fn set_message_id_min(&mut self, message_id_min: u8) {
        self.message_id_min = message_id_min;
    }

    pub fn set_message_id_max(&mut self, message_id_max: u8) {
        self.message_id_max = message_id_max;
    }

    pub fn set_enable_reg_exp_app_id(&mut self, enable_reg_exp_app_id: bool) {
        self.enable_reg_exp_app_id = enable_reg_exp_app_id;
    }

    pub fn set_enable_reg_exp_context(&mut self, enable_reg_exp_context: bool) {
        self.enable_reg_exp_context = enable_reg_exp_context;
    }

    pub fn set_enable_reg_exp_header(&mut self, enable_reg_exp_header: bool) {
        self.enable_reg_exp_header = enable_reg_exp_header;
    }

    pub fn set_enable_reg_exp_payload(&mut self, enable_reg_exp_payload: bool) {
        self.enable_reg_exp_payload = enable_reg_exp_payload;
    }

    pub fn set_ignore_case_header(&mut self, ignore_case_header: bool) {
        self.ignore_case_header = ignore_case_header;
    }

    pub fn set_ignore_case_payload(&mut self, ignore_case_payload: bool) {
        self.ignore_case_payload = ignore_case_payload;
    }

    pub fn set_enable_filter(&mut self, enable_filter: bool) {
        self.enable_filter = enable_filter;
    }

    pub fn set_enable_ecu_id(&mut self, enable_ecu_id: bool) {
        self.enable_ecu_id = enable_ecu_id;
    }

    pub fn set_enable_application_id(&mut self, enable_application_id: bool) {
        self.enable_application_id = enable_application_id;
    }

    pub fn set_enable_context_id(&mut self, enable_context_id: bool) {
        self.enable_context_id = enable_context_id;
    }

    pub fn set_enable_header_text(&mut self, enable_header_text: bool) {
        self.enable_header_text = enable_header_text;
    }

    pub fn set_enable_payload_text(&mut self, enable_payload_text: bool) {
        self.enable_payload_text = enable_payload_text;
    }

    pub fn set_enable_ctrl_msgs(&mut self, enable_ctrl_msgs: bool) {
        self.enable_ctrl_msgs = enable_ctrl_msgs;
    }

    pub fn set_enable_log_level_min(&mut self, enable_log_level_min: bool) {
        self.enable_log_level_min = enable_log_level_min;
    }

    pub fn set_enable_log_level_max(&mut self, enable_log_level_max: bool) {
        self.enable_log_level_max = enable_log_level_max;
    }

    pub fn set_enable_marker(&mut self, enable_marker: bool) {
        self.enable_marker = enable_marker;
    }

    pub fn set_enable_message_id(&mut self, enable_message_id: bool) {
        self.enable_message_id = enable_message_id;
    }

    pub fn set_enable_regex_search_replace(&mut self, enable_regex_search_replace: bool) {
        self.enable_regex_search_replace = enable_regex_search_replace;
    }

    pub fn set_filter_colour(&mut self, filter_colour: String) {
        self.filter_colour = filter_colour;
    }

    pub fn set_log_level_max(&mut self, log_level_max: u8) {
        self.log_level_max = log_level_max;
    }

    pub fn set_log_level_min(&mut self, log_level_min: u8) {
        self.log_level_min = log_level_min;
    }
}
