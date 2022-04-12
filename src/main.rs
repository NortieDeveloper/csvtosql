mod sql_builder;
mod csv_helper;

use std::fs;
use std::path::Path;
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the generated table
    #[clap(short, long, default_value = "default_table")]
    table: String,

    /// Name of the database used in the sql builder
    #[clap(short, long, default_value = "default_db")]
    database: String,

    #[clap()]
    file_path: String,
}

fn main() {
    // Parse the program arguments
    let args = Args::parse();

    //Extract the headers from the csv file.
    let file_path = Path::new(&args.file_path);
    let headers = csv_helper::extract_headers(file_path);
    let headers = match headers {
        Ok(h) => h,
        Err(e) => panic!("Failed to read headers from file: {}", e)
    };

    let statement = sql_builder::build_sql_statement(headers, args.table.clone(), args.database);

    // Write statement to sql file.
    let output_file = format!("./{}.sql", args.table);
    match fs::write(&output_file, statement) {
        Ok(_) => println!("Wrote statement to {}", output_file),
        Err(e) => {println!("Failed to write statement to file: {}", e)}
    }
}


