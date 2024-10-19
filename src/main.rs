use clap::{CommandFactory, Parser};
use streamshare::upload;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
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
        if let Some((identifier, deltoken)) = parse_delete_param(&delete_param) {
            match streamshare::delete(identifier, deltoken).await {
                Ok(_) => println!("File deleted successfully"),
                Err(e) => eprintln!("Error deleting file: {}", e),
            }
        } else {
            eprintln!("Invalid format for --delete. Use 'file_identifier/deletion_token' (e.g., 'abc123/def456')");
        }
    } else if let Some(file_path) = args.file {
        match upload(&file_path).await {
            Ok((file_identifier, deletion_token)) => {
                let download_url = format!(
                    "https://streamshare.wireway.ch/download/{}",
                    file_identifier
                );

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
