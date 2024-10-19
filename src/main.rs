use clap::Parser;
use streamshare::upload;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Some(file_path) = args.file {
        match upload(&file_path).await {
            Ok((file_identifier, _deletion_token)) => {
                let download_url = format!(
                    "https://streamshare.wireway.ch/download/{}",
                    file_identifier
                );

                println!("File uploaded successfully");
                println!("Download URL: {}", download_url);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        eprintln!("Please provide a file path");
    }
}
