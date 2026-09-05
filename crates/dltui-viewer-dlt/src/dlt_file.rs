use std::path::PathBuf;

use crate::dlt_frame::DltFrame;

#[derive(Debug)]
pub struct DltFile {
    path: PathBuf,
    frames: Vec<DltFrame>,
}

impl DltFile {
    pub fn open(path: &PathBuf) -> Result<DltFile, crate::Error> {
        match path.extension() {
            Some(ext) => {
                if !ext.eq_ignore_ascii_case("dlt") {
                    return Err(crate::Error::UnsupportedExtension);
                }
            }
            None => {
                return Err(crate::Error::MissingFileExtension);
            }
        }

        let dlt_file = std::fs::File::open(path).expect("failed to open file");
        let mut reader =
            dlt_parse::storage::DltStorageReader::new(std::io::BufReader::new(dlt_file));

        let mut frames = Vec::new();
        while let Some(msg_result) = reader.next_packet() {
            let msg = msg_result.expect("failed to parse dlt packet");

            let dlt_frame = DltFrame::new(
                msg.storage_header,
                msg.packet.header().clone(),
                msg.packet.header().extended_header.clone(),
                msg.packet.slice().to_vec(),
                msg.packet.payload().to_vec(),
            );

            frames.push(dlt_frame);
        }

        Ok(DltFile {
            path: path.clone(),
            frames,
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn frames(&self) -> &[DltFrame] {
        &self.frames
    }

    pub fn frames_mut(&mut self) -> &mut Vec<DltFrame> {
        &mut self.frames
    }

    pub fn add_frame(&mut self, frame: DltFrame) {
        self.frames.push(frame);
    }

    pub fn remove_frame(&mut self, index: usize) -> Option<DltFrame> {
        if index < self.frames.len() {
            Some(self.frames.remove(index))
        } else {
            None
        }
    }

    pub fn clear_frames(&mut self) {
        self.frames.clear();
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    pub fn get_frame(&self, index: usize) -> Option<&DltFrame> {
        self.frames.get(index)
    }

    pub fn get_frame_mut(&mut self, index: usize) -> Option<&mut DltFrame> {
        self.frames.get_mut(index)
    }

    pub fn iter_frames(&self) -> impl Iterator<Item = &DltFrame> {
        self.frames.iter()
    }

    pub fn iter_frames_mut(&mut self) -> impl Iterator<Item = &mut DltFrame> {
        self.frames.iter_mut()
    }

    pub fn pretty_print(&self) -> Result<(), crate::Error> {
        for frame in &self.frames {
            let index = frame.header_message_counter();
            let date_time = frame.get_date_time()?;
            let ecu_id = frame.get_ecu_id()?;
            print!("{:<4} | {} | {} | ", index, date_time, ecu_id);

            if let Some((app_id, ctx_id, msg_type, subtype)) = frame.try_parse_extended_header()? {
                print!(
                    "{: ^4} | {: ^4} | {} {:<7} | ",
                    app_id.0, ctx_id.0, msg_type.0, subtype.0
                )
            }

            let text_payload = frame.try_parse_payload()?;
            print!("{}", text_payload);
            println!("");
        }
        Ok(())
    }
}
