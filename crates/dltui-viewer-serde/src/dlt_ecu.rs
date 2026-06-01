use crate::{dlt_application::DltApplication, get_value, try_get_value, value_as, value_as_bool};

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

impl DltEcu {
    pub(crate) fn deserialize(xml_ecu: &mut xmltree::Element) -> Result<Self, crate::Error> {
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

        let mut applications = Vec::new();
        while let Some(mut xml_application) = xml_ecu.take_child("application") {
            let dlt_application = DltApplication::deserialize(&mut xml_application)?;
            applications.push(dlt_application);
        }

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
}
