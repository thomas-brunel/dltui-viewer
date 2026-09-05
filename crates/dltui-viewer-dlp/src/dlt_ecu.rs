use crate::{
    DlpSerde, deserialize_children, dlt_application::DltApplication, get_value, serialize_children,
    try_get_value, value_as, value_as_bool,
};

#[derive(Debug)]
pub struct DltEcu {
    id: String,
    description: Option<String>,
    interface: String,
    hostname: Option<String>,
    mc_interface: Option<String>,
    mc_ip: String,
    ip_port: u16,
    udp_port: u16,
    port: Option<String>,
    baudrate: u64,
    send_serial_header_tcp: bool,
    send_serial_header_serial: bool,
    sync_to_serial_header_tcp: bool,
    sync_to_serial_header_serial: bool,
    log_level: u8,
    trace_status: u8,
    verbose_mode: bool,
    timing_packets: u16,
    send_get_log_info: bool,
    send_default_log_level: bool,
    send_get_software_version: bool,
    update_data: bool,
    multicast: bool,
    auto_reconnect: bool,
    auto_reconnect_timeout: u16,
    write_dltv2_storage_header: bool,
    applications: Vec<DltApplication>,
}

impl DlpSerde for DltEcu {
    fn deserialize(xml_ecu: &mut xmltree::Element) -> Result<Self, crate::Error> {
        let id = get_value(xml_ecu, "id")?;
        let description = try_get_value(xml_ecu, "description")?;
        let interface = get_value(xml_ecu, "interface")?;
        let hostname = try_get_value(xml_ecu, "hostname")?;
        let mc_interface = try_get_value(xml_ecu, "mcinterface")?;
        let mc_ip = get_value(xml_ecu, "mcIP")?;
        let ip_port = value_as(xml_ecu, "ipport")?;
        let udp_port = value_as(xml_ecu, "udpport")?;
        let port = try_get_value(xml_ecu, "port")?;
        let baudrate: u64 = value_as(xml_ecu, "baudrate")?;
        let send_serial_header_tcp = value_as_bool(xml_ecu, "sendserialheadertcp")?;
        let send_serial_header_serial = value_as_bool(xml_ecu, "sendserialheaderserial")?;
        let sync_to_serial_header_tcp = value_as_bool(xml_ecu, "synctoserialheadertcp")?;
        let sync_to_serial_header_serial = value_as_bool(xml_ecu, "synctoserialheaderserial")?;
        let log_level = value_as(xml_ecu, "loglevel")?;
        let trace_status = value_as(xml_ecu, "tracestatus")?;
        let verbose_mode = value_as_bool(xml_ecu, "verbosemode")?;
        let timing_packets = value_as(xml_ecu, "timingpackets")?;
        let send_get_log_info = value_as_bool(xml_ecu, "sendgetloginfo")?;
        let send_default_log_level = value_as_bool(xml_ecu, "sendDefaultLogLevel")?;
        let send_get_software_version = value_as_bool(xml_ecu, "sendGetSoftwareVersion")?;
        let update_data = value_as_bool(xml_ecu, "updatedata")?;
        let multicast = value_as_bool(xml_ecu, "multicast")?;
        let auto_reconnect = value_as_bool(xml_ecu, "autoReconnect")?;
        let auto_reconnect_timeout = value_as(xml_ecu, "autoReconnectTimeout")?;
        let write_dltv2_storage_header = value_as_bool(xml_ecu, "writeDLTv2StorageHeader")?;
        let applications = deserialize_children(xml_ecu, "application")?;

        Ok(Self {
            id,
            description,
            interface,
            hostname,
            mc_interface,
            mc_ip,
            ip_port,
            udp_port,
            port,
            baudrate,
            send_serial_header_tcp,
            send_serial_header_serial,
            sync_to_serial_header_tcp,
            sync_to_serial_header_serial,
            log_level,
            trace_status,
            verbose_mode,
            timing_packets,
            send_get_log_info,
            send_default_log_level,
            send_get_software_version,
            update_data,
            multicast,
            auto_reconnect,
            auto_reconnect_timeout,
            write_dltv2_storage_header,
            applications,
        })
    }

    fn serialize(&self) -> xmltree::Element {
        let mut xml_ecu = xmltree::Element::new("ecu");

        crate::to_value(&mut xml_ecu, "id", &self.id);
        crate::try_to_value(&mut xml_ecu, "description", &self.description);
        crate::to_value(&mut xml_ecu, "interface", &self.interface);
        crate::try_to_value(&mut xml_ecu, "hostname", &self.hostname);
        crate::try_to_value(&mut xml_ecu, "mcinterface", &self.mc_interface);
        crate::to_value(&mut xml_ecu, "mcIP", &self.mc_ip);
        crate::to_value(&mut xml_ecu, "ipport", &self.ip_port);
        crate::to_value(&mut xml_ecu, "udpport", &self.udp_port);
        crate::try_to_value(&mut xml_ecu, "port", &self.port);
        crate::to_value(&mut xml_ecu, "baudrate", &self.baudrate);
        crate::to_value_bool(
            &mut xml_ecu,
            "sendserialheadertcp",
            &self.send_serial_header_tcp,
        );
        crate::to_value_bool(
            &mut xml_ecu,
            "sendserialheaderserial",
            &self.send_serial_header_serial,
        );
        crate::to_value_bool(
            &mut xml_ecu,
            "synctoserialheadertcp",
            &self.sync_to_serial_header_tcp,
        );
        crate::to_value_bool(
            &mut xml_ecu,
            "synctoserialheaderserial",
            &self.sync_to_serial_header_serial,
        );
        crate::to_value(&mut xml_ecu, "loglevel", &self.log_level);
        crate::to_value(&mut xml_ecu, "tracestatus", &self.trace_status);
        crate::to_value_bool(&mut xml_ecu, "verbosemode", &self.verbose_mode);
        crate::to_value(&mut xml_ecu, "timingpackets", &self.timing_packets);
        crate::to_value_bool(&mut xml_ecu, "sendgetloginfo", &self.send_get_log_info);
        crate::to_value_bool(
            &mut xml_ecu,
            "sendDefaultLogLevel",
            &self.send_default_log_level,
        );
        crate::to_value_bool(
            &mut xml_ecu,
            "sendGetSoftwareVersion",
            &self.send_get_software_version,
        );
        crate::to_value_bool(&mut xml_ecu, "updatedata", &self.update_data);
        crate::to_value_bool(&mut xml_ecu, "multicast", &self.multicast);
        crate::to_value_bool(&mut xml_ecu, "autoReconnect", &self.auto_reconnect);
        crate::to_value(
            &mut xml_ecu,
            "autoReconnectTimeout",
            &self.auto_reconnect_timeout,
        );
        crate::to_value_bool(
            &mut xml_ecu,
            "writeDLTv2StorageHeader",
            &self.write_dltv2_storage_header,
        );
        serialize_children(&mut xml_ecu, &self.applications);

        xml_ecu
    }
}

impl DltEcu {
    pub fn new(
        id: String,
        description: Option<String>,
        interface: String,
        hostname: Option<String>,
        mc_interface: Option<String>,
        mc_ip: String,
        ip_port: u16,
        udp_port: u16,
        port: Option<String>,
        baudrate: u64,
        send_serial_header_tcp: bool,
        send_serial_header_serial: bool,
        sync_to_serial_header_tcp: bool,
        sync_to_serial_header_serial: bool,
        log_level: u8,
        trace_status: u8,
        verbose_mode: bool,
        timing_packets: u16,
        send_get_log_info: bool,
        send_default_log_level: bool,
        send_get_software_version: bool,
        update_data: bool,
        multicast: bool,
        auto_reconnect: bool,
        auto_reconnect_timeout: u16,
        write_dltv2_storage_header: bool,
    ) -> Self {
        Self {
            id,
            description,
            interface,
            hostname,
            mc_interface,
            mc_ip,
            ip_port,
            udp_port,
            port,
            baudrate,
            send_serial_header_tcp,
            send_serial_header_serial,
            sync_to_serial_header_tcp,
            sync_to_serial_header_serial,
            log_level,
            trace_status,
            verbose_mode,
            timing_packets,
            send_get_log_info,
            send_default_log_level,
            send_get_software_version,
            update_data,
            multicast,
            auto_reconnect,
            auto_reconnect_timeout,
            write_dltv2_storage_header,
            applications: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn interface(&self) -> &str {
        &self.interface
    }

    pub fn hostname(&self) -> Option<&str> {
        self.hostname.as_deref()
    }

    pub fn mc_interface(&self) -> Option<&str> {
        self.mc_interface.as_deref()
    }

    pub fn mc_ip(&self) -> &str {
        &self.mc_ip
    }

    pub fn ip_port(&self) -> u16 {
        self.ip_port
    }

    pub fn udp_port(&self) -> u16 {
        self.udp_port
    }

    pub fn port(&self) -> Option<&str> {
        self.port.as_deref()
    }

    pub fn baudrate(&self) -> u64 {
        self.baudrate
    }

    pub fn send_serial_header_tcp(&self) -> bool {
        self.send_serial_header_tcp
    }

    pub fn send_serial_header_serial(&self) -> bool {
        self.send_serial_header_serial
    }

    pub fn sync_to_serial_header_tcp(&self) -> bool {
        self.sync_to_serial_header_tcp
    }

    pub fn sync_to_serial_header_serial(&self) -> bool {
        self.sync_to_serial_header_serial
    }

    pub fn log_level(&self) -> u8 {
        self.log_level
    }

    pub fn trace_status(&self) -> u8 {
        self.trace_status
    }

    pub fn verbose_mode(&self) -> bool {
        self.verbose_mode
    }

    pub fn timing_packets(&self) -> u16 {
        self.timing_packets
    }

    pub fn send_get_log_info(&self) -> bool {
        self.send_get_log_info
    }

    pub fn send_default_log_level(&self) -> bool {
        self.send_default_log_level
    }

    pub fn send_get_software_version(&self) -> bool {
        self.send_get_software_version
    }

    pub fn update_data(&self) -> bool {
        self.update_data
    }

    pub fn multicast(&self) -> bool {
        self.multicast
    }

    pub fn auto_reconnect(&self) -> bool {
        self.auto_reconnect
    }

    pub fn auto_reconnect_timeout(&self) -> u16 {
        self.auto_reconnect_timeout
    }

    pub fn write_dltv2_storage_header(&self) -> bool {
        self.write_dltv2_storage_header
    }

    pub fn applications(&self) -> &Vec<DltApplication> {
        &self.applications
    }

    pub fn add_application(&mut self, application: DltApplication) {
        self.applications.push(application);
    }

    pub fn remove_application(&mut self, application_id: &str) {
        self.applications.retain(|app| app.id() != application_id);
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_interface(&mut self, interface: String) {
        self.interface = interface;
    }

    pub fn set_hostname(&mut self, hostname: Option<String>) {
        self.hostname = hostname;
    }

    pub fn set_mc_interface(&mut self, mc_interface: Option<String>) {
        self.mc_interface = mc_interface;
    }

    pub fn set_mc_ip(&mut self, mc_ip: String) {
        self.mc_ip = mc_ip;
    }

    pub fn set_ip_port(&mut self, ip_port: u16) {
        self.ip_port = ip_port;
    }

    pub fn set_udp_port(&mut self, udp_port: u16) {
        self.udp_port = udp_port;
    }

    pub fn set_port(&mut self, port: Option<String>) {
        self.port = port;
    }

    pub fn set_baudrate(&mut self, baudrate: u64) {
        self.baudrate = baudrate;
    }

    pub fn set_send_serial_header_tcp(&mut self, send_serial_header_tcp: bool) {
        self.send_serial_header_tcp = send_serial_header_tcp;
    }

    pub fn set_send_serial_header_serial(&mut self, send_serial_header_serial: bool) {
        self.send_serial_header_serial = send_serial_header_serial;
    }

    pub fn set_sync_to_serial_header_tcp(&mut self, sync_to_serial_header_tcp: bool) {
        self.sync_to_serial_header_tcp = sync_to_serial_header_tcp;
    }

    pub fn set_sync_to_serial_header_serial(&mut self, sync_to_serial_header_serial: bool) {
        self.sync_to_serial_header_serial = sync_to_serial_header_serial;
    }

    pub fn set_log_level(&mut self, log_level: u8) {
        self.log_level = log_level;
    }

    pub fn set_trace_status(&mut self, trace_status: u8) {
        self.trace_status = trace_status;
    }

    pub fn set_verbose_mode(&mut self, verbose_mode: bool) {
        self.verbose_mode = verbose_mode;
    }

    pub fn set_timing_packets(&mut self, timing_packets: u16) {
        self.timing_packets = timing_packets;
    }

    pub fn set_send_get_log_info(&mut self, send_get_log_info: bool) {
        self.send_get_log_info = send_get_log_info;
    }

    pub fn set_send_default_log_level(&mut self, send_default_log_level: bool) {
        self.send_default_log_level = send_default_log_level;
    }

    pub fn set_send_get_software_version(&mut self, send_get_software_version: bool) {
        self.send_get_software_version = send_get_software_version;
    }

    pub fn set_update_data(&mut self, update_data: bool) {
        self.update_data = update_data;
    }

    pub fn set_multicast(&mut self, multicast: bool) {
        self.multicast = multicast;
    }

    pub fn set_auto_reconnect(&mut self, auto_reconnect: bool) {
        self.auto_reconnect = auto_reconnect;
    }

    pub fn set_auto_reconnect_timeout(&mut self, auto_reconnect_timeout: u16) {
        self.auto_reconnect_timeout = auto_reconnect_timeout;
    }

    pub fn set_write_dltv2_storage_header(&mut self, write_dltv2_storage_header: bool) {
        self.write_dltv2_storage_header = write_dltv2_storage_header;
    }
}
