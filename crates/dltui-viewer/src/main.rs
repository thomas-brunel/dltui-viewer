use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// DLT Project file
    #[arg(short, long)]
    dlp_file: String,
}

fn main() {
    let args = Args::parse();

    match dltui_viewer_serde::dlt_project::DltProject::open(&args.dlp_file) {
        Ok(dlt_project) => {
            println!("DLT PROJECT OPEN SUCCESSFULLY:\r\n{:#?}", dlt_project);
        }
        Err(e) => println!("FAILED TO OPEN DLT PROJECT: {:?}", e),
    }
}
