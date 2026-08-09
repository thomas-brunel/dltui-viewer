use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// DLT Project file
    #[arg(short, long)]
    dlp_file: String,
    #[arg(short, long, default_value_t="".to_string())]
    save_as_path: String,
}

fn main() {
    let args = Args::parse();

    match dltui_viewer_serde::dlt_project::DltProject::open(&args.dlp_file) {
        Ok(mut dlt_project) => {
            println!("DLT PROJECT OPEN SUCCESSFULLY:\r\n{:#?}", dlt_project);
            if !args.save_as_path.is_empty() {
                // SAVE DLT PROJECT
                match dlt_project.save_as(&args.save_as_path) {
                    Ok(()) => println!("DLT PROJECT SAVE SUCCESSFULLY AT {}", args.save_as_path),
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
