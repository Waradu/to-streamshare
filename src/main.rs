use std::{io::Write, time::Instant};

use clap::{CommandFactory, Parser};
use streamshare::{delete, upload};

#[derive(Parser, Debug)]
#[command(name = "toss", version, about, long_about = None)]
struct Args {
    file: Option<String>,

    #[arg(
        short,
        long,
        value_name = "DELETE",
        help = "Specify a file to delete in the format 'file_identifier/deletion_token' (e.g., 'abc123/def456')"
    )]
    delete: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Some(delete_param) = args.delete {
        if let Some((file_identifier, deletion_token)) = parse_delete_param(&delete_param) {
            match delete(file_identifier, deletion_token).await {
                Ok(_) => println!("File deleted successfully"),
                Err(e) => eprintln!("Error deleting file: {}", e),
            }
        } else {
            eprintln!("Invalid format for --delete. Use 'file_identifier/deletion_token' (e.g., 'abc123/def456')");
        }
    } else if let Some(file_path) = args.file {
        let start_time = Instant::now();
        let mut file_size: u64 = 0;
        
        let show_progress = |uploaded_bytes, total_bytes| {
            let percentage = (uploaded_bytes as f64 / total_bytes as f64) * 100.0;
            let uploaded = readable(uploaded_bytes);
            let total = readable(total_bytes);
            let elapsed_secs = start_time.elapsed().as_secs_f64();
            let speed = readable((uploaded_bytes as f64 / elapsed_secs) as u64);
            file_size = total_bytes;

            print!(
                "\r\x1b[2K{:.2}% {}/{} ({}/s)",
                percentage, uploaded, total, speed
            );
            std::io::stdout().flush().unwrap();
        };

        match upload(&file_path, show_progress).await {
            Ok((file_identifier, deletion_token)) => {
                let download_url = format!(
                    "https://streamshare.wireway.ch/download/{}",
                    file_identifier
                );

                let elapsed_secs = start_time.elapsed().as_secs_f64();
                println!(
                    "\r\x1b[2K100.00% {}/{} (Upload completed in {:.2}s)",
                    readable(file_size),
                    readable(file_size),
                    elapsed_secs
                );
                println!();

                println!("File uploaded successfully");
                println!("Download URL: {}", download_url);
                println!("File Identifier: {}", file_identifier);
                println!("Deletion Token: {}", deletion_token);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        Args::command().print_help().unwrap();
    }
}

fn parse_delete_param(param: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = param.splitn(2, '/').collect();
    if parts.len() == 2 {
        Some((parts[0], parts[1]))
    } else {
        None
    }
}

fn readable(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    if bytes as f64 >= GB {
        format!("{:.2}gb", bytes as f64 / GB)
    } else if bytes as f64 >= MB {
        format!("{:.2}mb", bytes as f64 / MB)
    } else if bytes as f64 >= KB {
        format!("{:.2}kb", bytes as f64 / KB)
    } else {
        format!("{}b", bytes)
    }
}
