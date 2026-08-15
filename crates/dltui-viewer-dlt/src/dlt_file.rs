use std::fmt::Write;

#[derive(Debug)]
pub struct DltFile {
    path: String,
}

impl DltFile {
    pub fn open(path: &str) -> Result<DltFile, crate::Error> {
        let dlt_file = std::fs::File::open(path).expect("failed to open file");
        let mut reader =
            dlt_parse::storage::DltStorageReader::new(std::io::BufReader::new(dlt_file));

        while let Some(msg_result) = reader.next_packet() {
            let msg = msg_result.expect("failed to parse dlt packet");

            // Process header
            let header = msg.packet.header();
            let index = header.message_counter;

            // the storage header contains the ecu id and the timestamp
            let date_time = Self::frame_storage_header_timestamp_as_date_time(&msg.storage_header)?;

            let ecu_id = msg.storage_header.ecu_id_str()?;
            print!("{:<4} | {} | {} | ", index, date_time, ecu_id);

            if let Some(extended_header) = &header.extended_header {
                let app_id = str::from_utf8(&extended_header.application_id)?;
                let ctx_id = str::from_utf8(&extended_header.context_id)?;
                let app_id = app_id.trim_end_matches('\0');
                let ctx_id = ctx_id.trim_end_matches('\0');
                let mut msg_type = String::new();
                let mut subtype = String::new();
                if let Some(dlt_msg_type) = &extended_header.message_info.into_message_type() {
                    match dlt_msg_type {
                        dlt_parse::DltMessageType::Control(ctrl) => {
                            match ctrl {
                                dlt_parse::DltControlMessageType::Request => {
                                    subtype = "request".into()
                                }
                                dlt_parse::DltControlMessageType::Response => {
                                    subtype = "response".into()
                                }
                            };
                            msg_type = "control".into();
                        }
                        dlt_parse::DltMessageType::Log(level) => {
                            match level {
                                dlt_parse::DltLogLevel::Debug => subtype = "debug".into(),
                                dlt_parse::DltLogLevel::Error => subtype = "error".into(),
                                dlt_parse::DltLogLevel::Fatal => subtype = "fatal".into(),
                                dlt_parse::DltLogLevel::Info => subtype = "info".into(),
                                dlt_parse::DltLogLevel::Verbose => subtype = "verbose".into(),
                                dlt_parse::DltLogLevel::Warn => subtype = "warn".into(),
                            };
                            msg_type = "log".into();
                        }
                        dlt_parse::DltMessageType::NetworkTrace(nt) => {
                            match nt {
                                dlt_parse::DltNetworkType::Can => subtype = "can".into(),
                                dlt_parse::DltNetworkType::Ethernet => subtype = "ethernet".into(),
                                dlt_parse::DltNetworkType::Flexray => subtype = "flexray".into(),
                                dlt_parse::DltNetworkType::Ipc => subtype = "ipc".into(),
                                dlt_parse::DltNetworkType::Most => subtype = "most".into(),
                                dlt_parse::DltNetworkType::SomeIp => subtype = "some ip".into(),
                                dlt_parse::DltNetworkType::UserDefined(ud) => {
                                    subtype = ud.to_string()
                                }
                            };
                            msg_type = "network trace".into();
                        }
                        dlt_parse::DltMessageType::Trace(t) => {
                            match t {
                                dlt_parse::DltTraceType::FunctionIn => {
                                    subtype = "funtion in".into()
                                }
                                dlt_parse::DltTraceType::FunctionOut => {
                                    subtype = "function out".into()
                                }
                                dlt_parse::DltTraceType::State => subtype = "state".into(),
                                dlt_parse::DltTraceType::Variable => subtype = "variable".into(),
                                dlt_parse::DltTraceType::Vfb => subtype = "vfb".into(),
                            };
                            msg_type = "trace".into();
                        }
                    }
                };
                print!(
                    "{: ^4} | {: ^4} | {} {:<7} | ",
                    app_id, ctx_id, msg_type, subtype
                )
            }

            // the dlt packet
            if msg.packet.is_verbose() {
                let text_payload = Self::parse_verbose_payload(&msg.packet)?;
                print!("{}", text_payload);
                println!("")
            } else {
                if let Some(nonvb_payload) = msg.packet.non_verbose_payload() {
                    let payload = match str::from_utf8(nonvb_payload) {
                        Ok(pld) => pld,
                        Err(e) => {
                            return Err(crate::Error::Utf8DecodeError(e));
                        }
                    };
                    println!("NON-VERBOSE PAYLOAD TEXT: {}", payload);
                }
            }
        }

        Ok(DltFile { path: path.into() })
    }

    pub fn frame_storage_header_timestamp_as_date_time(
        storage_header: &dlt_parse::storage::StorageHeader,
    ) -> Result<chrono::DateTime<chrono::Utc>, crate::Error> {
        match chrono::DateTime::from_timestamp(
            storage_header.timestamp_seconds as i64,
            storage_header.timestamp_microseconds * 1000,
        ) {
            Some(dt) => Ok(dt),
            None => Err(crate::Error::MissingStorageTimestampError),
        }
    }

    fn parse_verbose_payload<'a>(
        packet: &dlt_parse::DltPacketSlice<'a>,
    ) -> Result<String, crate::Error> {
        let Some(verbose_values) = packet.verbose_value_iter() else {
            return Ok("".into());
        };

        let mut text_payload = String::new();
        verbose_values.for_each(|val| match val {
            Ok(v) => {
                match v {
                    dlt_parse::verbose::VerboseValue::Bool(b) => {
                        if let Some(name) = b.name {
                            write!(&mut text_payload, "{}: ", name);
                        }
                        write!(&mut text_payload, "{}", b.value);
                    }
                    dlt_parse::verbose::VerboseValue::Str(s) => {
                        if let Some(name) = s.name {
                            write!(&mut text_payload, "{}: ", name);
                        }
                        write!(&mut text_payload, "{}", s.value);
                    }
                    dlt_parse::verbose::VerboseValue::TraceInfo(ti) => {
                        write!(&mut text_payload, "{}", ti.value);
                    }
                    dlt_parse::verbose::VerboseValue::I8(i_8) => {
                        if let Some(var_info) = &i_8.variable_info {
                            write!(&mut text_payload, "{:?}: ", var_info);
                        }
                        if let Some(scaling) = &i_8.scaling {
                            write!(&mut text_payload, "{:?} ", scaling);
                        }
                        write!(&mut text_payload, "{}", i_8.value);
                    }
                    dlt_parse::verbose::VerboseValue::I16(i_16) => {
                        write!(&mut text_payload, "{:?}", i_16);
                    }
                    dlt_parse::verbose::VerboseValue::I32(i_32) => {
                        write!(&mut text_payload, "{:?}", i_32);
                    }
                    dlt_parse::verbose::VerboseValue::I64(i_64) => {
                        write!(&mut text_payload, "{:?}", i_64);
                    }
                    dlt_parse::verbose::VerboseValue::I128(i_128) => {
                        write!(&mut text_payload, "{:?}", i_128);
                    }
                    dlt_parse::verbose::VerboseValue::U8(u_8) => {
                        write!(&mut text_payload, "{:?}", u_8);
                    }
                    dlt_parse::verbose::VerboseValue::U16(u_16) => {
                        write!(&mut text_payload, "{:?}", u_16);
                    }
                    dlt_parse::verbose::VerboseValue::U32(u_32) => {
                        write!(&mut text_payload, "{:?}", u_32);
                    }
                    dlt_parse::verbose::VerboseValue::U64(u_64) => {
                        write!(&mut text_payload, "{:?}", u_64);
                    }
                    dlt_parse::verbose::VerboseValue::U128(u_128) => {
                        write!(&mut text_payload, "{:?}", u_128);
                    }
                    dlt_parse::verbose::VerboseValue::F16(f_16) => {
                        write!(&mut text_payload, "{:?}", f_16);
                    }
                    dlt_parse::verbose::VerboseValue::F32(f_32) => {
                        write!(&mut text_payload, "{:?}", f_32);
                    }
                    dlt_parse::verbose::VerboseValue::F64(f_64) => {
                        write!(&mut text_payload, "{:?}", f_64);
                    }
                    dlt_parse::verbose::VerboseValue::F128(f_128) => {
                        write!(&mut text_payload, "{:?}", f_128);
                    }
                    dlt_parse::verbose::VerboseValue::ArrBool(ab) => {
                        write!(&mut text_payload, "{:?}", ab);
                    }
                    dlt_parse::verbose::VerboseValue::ArrI8(ai_9) => {
                        write!(&mut text_payload, "{:?}", ai_9);
                    }
                    dlt_parse::verbose::VerboseValue::ArrI16(ai_16) => {
                        write!(&mut text_payload, "{:?}", ai_16);
                    }
                    dlt_parse::verbose::VerboseValue::ArrI32(a_i32) => {
                        write!(&mut text_payload, "{:?}", a_i32);
                    }
                    dlt_parse::verbose::VerboseValue::ArrI64(a_i64) => {
                        write!(&mut text_payload, "{:?}", a_i64);
                    }
                    dlt_parse::verbose::VerboseValue::ArrI128(a_i128) => {
                        write!(&mut text_payload, "{:?}", a_i128);
                    }
                    dlt_parse::verbose::VerboseValue::ArrU8(au_8) => {
                        write!(&mut text_payload, "{:?}", au_8);
                    }
                    dlt_parse::verbose::VerboseValue::ArrU16(au_16) => {
                        write!(&mut text_payload, "{:?}", au_16);
                    }
                    dlt_parse::verbose::VerboseValue::ArrU32(au_32) => {
                        write!(&mut text_payload, "{:?}", au_32);
                    }
                    dlt_parse::verbose::VerboseValue::ArrU64(au_64) => {
                        write!(&mut text_payload, "{:?}", au_64);
                    }
                    dlt_parse::verbose::VerboseValue::ArrU128(au_128) => {
                        write!(&mut text_payload, "{:?}", au_128);
                    }
                    dlt_parse::verbose::VerboseValue::ArrF16(af_16) => {
                        write!(&mut text_payload, "{:?}", af_16);
                    }
                    dlt_parse::verbose::VerboseValue::ArrF32(af_32) => {
                        write!(&mut text_payload, "{:?}", af_32);
                    }
                    dlt_parse::verbose::VerboseValue::ArrF64(af_64) => {
                        write!(&mut text_payload, "{:?}", af_64);
                    }
                    dlt_parse::verbose::VerboseValue::ArrF128(af_128) => {
                        write!(&mut text_payload, "{:?}", af_128);
                    }
                    dlt_parse::verbose::VerboseValue::Struct(stct) => {
                        write!(&mut text_payload, "{:?}", stct);
                    }
                    dlt_parse::verbose::VerboseValue::Raw(r) => {
                        if let Some(name) = r.name {
                            write!(&mut text_payload, "{}: ", name);
                        }
                        write!(&mut text_payload, "{:?}", r.data);
                    }
                }
                text_payload.push(' ');
            }
            Err(e) => {
                println!("Failed to decode verbose value {:?}", e);
            } //Err(dlt_parse_error::DltParseError::DecodeFailed(e)),
        });

        Ok(text_payload)
    }
}
