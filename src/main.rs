use clap::Parser;
use std::fs::File;
use std::io::{
	BufRead,
	BufReader,
	ErrorKind,
	Result
};
use std::path::Path;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	/// Path of the text file
	path: String,

	/// Display line number
	#[clap(short)]
	line_number: bool,

	/// Render whitespace characters
	#[clap(short)]
	whitespace: bool,
}

fn get_file(path: String) -> File {
	let file = Path::new(&path);
	if file.is_dir() {
		eprintln!("{} is a directory", &path);
		std::process::exit(2);
	}

	match File::open(&path) {
		Ok(file) => file,
		Err(error) => {
			if error.kind() == ErrorKind::PermissionDenied {
				eprintln!("Unable to read `{}`", &path);
			}

			if error.kind() == ErrorKind::NotFound {
				eprintln!("File `{}` not found", &path);
			}

			std::process::exit(1);
		}
	}
}

fn print_line(line_number: u16, line: String, print_line_number: bool, print_whitespace: bool) {
	let line_to_print = if print_whitespace {
		line
			.replace(" ", "\x1b[38;5;240m•\x1b[0m")
			.replace("\t", "\x1b[38;5;240m>\x1b[0m\t")
		} else {
			line
		};

	if print_line_number {
		println!("\x1b[38;5;241m{:>3}\x1b[0m {}", line_number, line_to_print);
	} else {
		println!("{}", line_to_print);
	}
}

fn print_lines(file: File, print_line_number: bool, print_whitespace: bool) {
	let reader = BufReader::new(file);
	let mut line_counter: u16 = 0;

	for line in reader.lines() {
		line_counter += 1;

		match line {
			Ok(line) => print_line(line_counter, line, print_line_number, print_whitespace),
			Err(error) => eprintln!("{}", error),
		}
	}
}

fn main() -> Result<()> {
	let cli = Cli::parse();
	let file = get_file(cli.path);

	print_lines(file, cli.line_number, cli.whitespace);

	Ok(())
}
