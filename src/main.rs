use clap::{Parser, ValueEnum};

mod dates;

use dates::{now, parse_date};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Status {
    Reading,
    WantToRead,
    Done,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Title of the book
    #[arg(short, long)]
    title: String,

    /// Book author
    #[arg(short, long)]
    author: String,

    /// Date you started reading
    #[arg(short, long)]
    date_started: Option<String>,

    /// Date you finished reading (will default status to 'Done')
    #[arg(short('f'), long)]
    date_finished: Option<String>,

    /// URL for the book
    #[arg(short, long)]
    url: Option<String>,

    /// Are you currently Reading, Want to read, or Done with this book?
    #[arg(short, long)]
    status: Option<String>,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let date_started = parse_date(args.date_started, true, now)?;
    let date_finished = parse_date(args.date_finished, false, now)?;
    let url = args.url.unwrap_or("".to_string());
    let status = parse_status(args.status)?;

    let book = Book {
        title: args.title,
        date_started,
        date_finished,
        url,
        status,
    };

    println!("{:?}", book);

    Ok(())
}

fn parse_status(status: Option<String>) -> Result<Status, String> {
    match status {
        Some(s) => match s.as_str() {
            "done" => Ok(Status::Done),
            "reading" => Ok(Status::Reading),
            "want" => Ok(Status::WantToRead),
            _ => {
                return Err(
                    "Invalid status. Use one of the following: done | reading | want".to_string(),
                )
            }
        },
        None => Ok(Status::Reading),
    }
}

#[derive(Debug)]
struct Book {
    title: String,
    date_started: String,
    date_finished: String,
    url: String,
    status: Status,
}
