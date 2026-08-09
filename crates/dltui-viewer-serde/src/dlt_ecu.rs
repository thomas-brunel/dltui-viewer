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
