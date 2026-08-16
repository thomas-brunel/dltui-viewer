use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// DLT Project file
    #[arg(short, long, default_value_t="".to_string())]
    dlp_file: String,
    #[arg(short, long, default_value_t="".to_string())]
    save_as_path: String,
    #[arg(long, default_value_t="".to_string())]
    dlt_file: String,
    #[arg(long, action)]
    no_tui: bool,
}

fn main() {
    let args = Args::parse();

    if !args.dlp_file.is_empty() {
        match dltui_viewer_dlp::dlt_project::DltProject::open(&args.dlp_file.into()) {
            Ok(mut dlt_project) => {
                println!("DLT PROJECT OPEN SUCCESSFULLY:\r\n{:#?}", dlt_project);
                if !args.save_as_path.is_empty() {
                    // SAVE DLT PROJECT
                    match dlt_project.save_as(&args.save_as_path.clone().into()) {
                        Ok(()) => {
                            println!("DLT PROJECT SAVE SUCCESSFULLY AT {}", args.save_as_path)
                        }
                        Err(e) => println!(
                            "FAILED TO SAVE DLT PROJECT AT {} - {:?}",
                            args.save_as_path, e
                        ),
                    }
                }
            }
            Err(e) => println!("FAILED TO OPEN DLT PROJECT: {:?}", e),
        }
    }

    if !args.dlt_file.is_empty() {
        match dltui_viewer_dlt::dlt_file::DltFile::open(&args.dlt_file.into()) {
            Ok(dlt_file) => println!("DLT FILE OPENED SUCCESSFULLY:\r\n{:#?}", dlt_file),
            Err(e) => println!("FAILED TO OPEN DLT FILE! {:?}", e),
        };
    }

    if !args.no_tui {
        match dltui_viewer_tui::start_tui() {
            Ok(_) => println!("TUI OK"),
            Err(e) => println!("TUI FAILED! {:?}", e),
        }
    }
}
