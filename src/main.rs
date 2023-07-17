use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;

use clap::Parser;

static VERSION: OnceLock<String> = OnceLock::new();

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// base path, default: current path
	#[arg(short, long)]
	path: Option<PathBuf>,
}

struct Session {
	base: PathBuf,
	input: String,
}

fn main() -> Result<(), String> {
	// print project version.
	let full_version = VERSION.get_or_init(|| {
		let git_sha = option_env!("VERGEN_GIT_SHA").unwrap_or_default();
		let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
		let timestamp = option_env!("VERGEN_BUILD_TIMESTAMP").unwrap_or_default();
		format!("FileSQL Version: {}\nGit Sha: {}\nBuild Time: {}", version, git_sha, timestamp)
	});
	println!("{}", full_version);

	// parse base path.
	let args = Args::parse();
	let base_path =
		args.path.unwrap_or_else(|| env::current_dir().expect("Failed to get current dir"));
	println!("Base Path: {:?}", &base_path);

	// process instructions.
	let mut session = Session { base: base_path, input: Default::default() };
	loop {
		let line = read_line()?;
		let line = line.trim();
		if line.is_empty() {
			continue;
		}

		if matches!(line, "quit" | "exit") {
			break;
		} else if line.ends_with(';') {
			session.input.push_str(&line[..line.len() - 1]);
			println!("{}", &session.input);
			session.input.clear();
		} else {
			session.input.push_str(line);
			session.input.push(' ');
		}
	}

	Ok(())
}

fn read_line() -> Result<String, String> {
	write!(std::io::stdout(), "FileSQL> ").map_err(|e| e.to_string())?;
	std::io::stdout().flush().map_err(|e| e.to_string())?;
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).map_err(|e| e.to_string())?;
	Ok(buffer)
}
