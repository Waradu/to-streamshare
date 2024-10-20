use clap::{CommandFactory, Parser};
use kdam::{term::Colorizer, tqdm, BarExt, Column, RichProgress, Spinner};
use std::fs;
use std::io::{stderr, IsTerminal};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use streamshare::StreamShare;

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

    #[arg(
        short,
        long,
        value_name = "SERVER",
        help = "Specify a server",
        default_value = "streamshare.wireway.ch"
    )]
    server: Option<String>,

    #[arg(
        short,
        long,
        value_name = "CHUNK-SIZE",
        help = "Specify a chunk size",
        default_value = "1048576"
    )]
    chunk_size: Option<String>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let server = args.server.unwrap();
    let chunk_size = match args.chunk_size.unwrap().parse::<usize>() {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error: The provided chunk size is not a valid integer.");
            std::process::exit(1);
        }
    };

    let client = StreamShare::new(server.clone(), chunk_size);

    if let Some(delete) = args.delete {
        if let Some((file_identifier, deletion_token)) = parse_delete_param(&delete) {
            match client.delete(file_identifier, deletion_token).await {
                Ok(_) => println!("File deleted successfully"),
                Err(e) => eprintln!("Error deleting file: {}", e),
            }
        } else {
            eprintln!("Invalid format for --delete. Use 'file_identifier/deletion_token' (e.g., 'abc123/def456')");
        }
    } else if let Some(file_path) = args.file {
        kdam::term::init(stderr().is_terminal());
        kdam::term::hide_cursor()?;

        let file_size = fs::metadata(&file_path)?.len();

        let pb = RichProgress::new(
            tqdm!(
                total = file_size as usize,
                unit_scale = true,
                unit_divisor = 1024,
                unit = "B",
                mininterval = 0.01,
                dynamic_ncols = true,
                colour = "green"
            ),
            vec![
                Column::Spinner(Spinner::new(
                    &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
                    80.0,
                    1.0,
                )),
                Column::Percentage(1),
                Column::Text("•".to_owned()),
                Column::Animation,
                Column::Text("•".to_owned()),
                Column::CountTotal,
                Column::Text("•".to_owned()),
                Column::Rate,
                Column::Text("•".to_owned()),
                Column::RemainingTime,
            ],
        );

        let pb_arc = Arc::new(Mutex::new(pb));
        let current_progress = Arc::new(Mutex::new(0));

        let pb_arc_clone = pb_arc.clone();
        let current_progress_clone = current_progress.clone();

        let update_thread = thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(50));
            let progress = *current_progress_clone.lock().unwrap();
            if progress >= file_size {
                break;
            }
            pb_arc_clone
                .lock()
                .unwrap()
                .update_to(progress as usize)
                .unwrap();
        });

        match client
            .upload(&file_path, move |current, _total| {
                *current_progress.lock().unwrap() = current;
            })
            .await
        {
            Ok((file_identifier, deletion_token)) => {
                let mut pb = pb_arc.lock().unwrap();
                pb.update_to(file_size as usize).unwrap();
                pb.clear().unwrap();

                println!("\n{}", "┌".to_owned() + &"─".repeat(79) + "┐");
                println!("│{:^90}│", "Upload Complete!".colorize("bold green"));
                println!("├{}┤", "─".repeat(79));

                let download_url = format!(
                    "https://streamshare.wireway.ch/download/{}",
                    file_identifier
                );
                println!(
                    "│ {:<15} {:<31} │",
                    "URL:".colorize("bold yellow"),
                    download_url
                );
                println!(
                    "│ {:<15} {:<68} │",
                    "File ID:".colorize("bold yellow"),
                    file_identifier
                );
                println!(
                    "│ {:<15} {:<61} │",
                    "Deletion Token:".colorize("bold yellow"),
                    deletion_token
                );

                println!("{}", "└".to_owned() + &"─".repeat(79) + "┘");
                println!()
            }
            Err(e) => eprintln!("{}", format!("Error: {}", e).colorize("bold red")),
        }

        update_thread.join().unwrap();
        kdam::term::show_cursor()?;
    } else {
        Args::command().print_help().unwrap();
    }

    Ok(())
}

fn parse_delete_param(param: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = param.splitn(2, '/').collect();
    if parts.len() == 2 {
        Some((parts[0], parts[1]))
    } else {
        None
    }
}
