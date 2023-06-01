use colored::Colorize;

pub enum CliError {
	UnknownArg { arg: String },
	UnknownFlag { flag: String },
	UnknownLicense { license: String },
	InvalidFlag { flag: String, reason: String },
	MissingFlag { flag: String, reason: String },
	FailedWrite { path: std::path::PathBuf },
	FailedRead { kind: String },
	FailedParse { kind: String },
}

impl CliError {
	pub fn throw(&self) -> ! {
		eprintln!(
			"{} {}",
			"ERROR".red().bold(),
			match self {
				CliError::UnknownArg { arg } => format!("Unknown argument '{}'!", arg),
				CliError::UnknownFlag { flag } => format!("Unknown flag '{}'!", flag),
				CliError::UnknownLicense { license } => format!("Unknown license '{}'!", license),
				CliError::InvalidFlag { flag, reason } =>
					format!("Invalid {} flag! {}", flag, reason),
				CliError::MissingFlag { flag, reason } =>
					format!("Missing {} flag! {}", flag, reason),
				CliError::FailedWrite { path } =>
					format!("Failed to write to '{}'!", path.to_str().unwrap()),
				_ => unreachable!(),
			},
		);

		std::process::exit(1);
	}

	pub fn warn(&self) {
		eprintln!(
			"{} {}",
			"WARN".yellow().bold(),
			match self {
				CliError::FailedRead { kind } =>
					format!("A '{}' was found but failed to be read!", kind),
				CliError::FailedParse { kind } =>
					format!("A '{}' was found but failed to be parsed!", kind),
				_ => unreachable!(),
			}
		);
	}
}
