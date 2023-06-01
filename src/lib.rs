use colored::Colorize;
pub use error::CliError;
use json::parse;
use regex::Regex;
use std::{
	fs::{read_to_string, write},
	io::ErrorKind,
	path::PathBuf,
	time::{SystemTime, UNIX_EPOCH},
};
use toml_edit::{value, Document};
mod error;
mod licenses;

pub struct Author {
	pub name: String,
	pub email: Option<String>,
}

pub struct PackageInfo {
	pub name: Option<String>,
	pub authors: Vec<Author>,
	pub url: Option<String>,
}

pub fn read_node(extract_regex: &Regex) -> Option<PackageInfo> {
	match read_to_string("package.json") {
		Err(err) => {
			if err.kind() == ErrorKind::NotFound {
				return None;
			}

			CliError::FailedRead {
				kind: "package.json".to_owned(),
			}
			.warn();

			None
		}
		Ok(string) => match parse(&string) {
			Err(_) => {
				CliError::FailedParse {
					kind: "package.json".to_owned(),
				}
				.warn();

				None
			}
			Ok(json) => {
				drop(string);

				let extract_info = |author: &json::JsonValue| {
					if author.is_string() {
						let mut result = Author {
							name: String::new(),
							email: None,
						};

						for cap in extract_regex.captures_iter(author.as_str().unwrap()) {
							if let Some(name) = cap.name("name") {
								result.name.push_str(name.as_str());
								result.name.push(' ');
							} else if let Some(email) = cap.name("email") {
								result.email = Some(email.as_str().to_owned());
							}
						}

						result.name = result.name.trim().to_owned();

						if result.name.is_empty() {
							None
						} else {
							Some(result)
						}
					} else if author.is_object() {
						Some(Author {
							name: author["name"].as_str()?.to_owned(),
							email: author["email"].as_str().map(|e| e.to_owned()),
						})
					} else {
						None
					}
				};

				let mut pkg = PackageInfo {
					name: json["name"].as_str().map(|s| s.to_owned()),
					authors: if let Some(author) = extract_info(&json["author"]) {
						vec![author]
					} else {
						Vec::new()
					},
					url: json["homepage"].as_str().map(|url| url.to_owned()),
				};

				if let json::JsonValue::Array(contributors) = &json["contributors"] {
					if !contributors.is_empty() {
						pkg.authors.append(
							&mut contributors
								.iter()
								.filter_map(|contributor| extract_info(contributor))
								.collect(),
						)
					}
				}

				if !pkg.authors.is_empty() {
					Some(pkg)
				} else {
					None
				}
			}
		},
	}
}

fn failed_parse<T>() -> Option<T> {
	CliError::FailedParse {
		kind: "Cargo.toml".to_owned(),
	}
	.warn();

	None
}

pub fn read_cargo(extract_regex: &Regex) -> Option<PackageInfo> {
	match read_to_string("Cargo.toml") {
		Err(err) => {
			if err.kind() == ErrorKind::NotFound {
				return None;
			}

			CliError::FailedRead {
				kind: "Cargo.toml".to_owned(),
			}
			.warn();

			None
		}
		Ok(string) => match string.parse::<Document>() {
			Err(_) => failed_parse(),
			Ok(toml) => {
				let toml = toml.get("package").or_else(failed_parse)?;

				Some(PackageInfo {
					name: toml
						.get("name")
						.and_then(|name| Some(name.as_str()?.to_owned())),
					authors: toml
						.get("authors")
						.and_then(|authors| {
							Some(
								authors
									.as_array()?
									.iter()
									.filter_map(|author| {
										let mut result = Author {
											name: String::new(),
											email: None,
										};

										for cap in
											extract_regex.captures_iter(author.as_str().unwrap())
										{
											if let Some(name) = cap.name("name") {
												result.name.push_str(name.as_str());
												result.name.push(' ');
											} else if let Some(email) = cap.name("email") {
												result.email = Some(email.as_str().to_owned());
											}
										}

										result.name = result.name.trim().to_owned();

										if result.name.is_empty() {
											None
										} else {
											Some(result)
										}
									})
									.collect::<Vec<Author>>(),
							)
						})
						.unwrap_or_default(),
					url: toml
						.get("homepage")
						.and_then(|url| Some(url.as_str()?.to_owned())),
				})
			}
		},
	}
}

pub fn read_python(extract_regex: &Regex) -> Option<PackageInfo> {
	read_cargo(extract_regex)
}

pub fn read_git(stdout: String) -> Option<Author> {
	let mut result = Author {
		name: String::new(),
		email: None,
	};

	for cap in Regex::new(r"(?m)(?:user\.name\s+)(?P<name>.+)|(?:user\.email\s+)(?P<email>.+)")
		.unwrap()
		.captures_iter(&stdout)
	{
		if let Some(name) = cap.name("name") {
			result.name.push_str(name.as_str().trim());
		} else if let Some(email) = cap.name("email") {
			result.email = Some(email.as_str().to_owned());
		}
	}

	if result.name.is_empty() {
		None
	} else {
		Some(result)
	}
}

fn get_info(
	emails: Vec<String>,
	mut names: Vec<String>,
	project: Option<String>,
	url: Option<String>,
	needs: (bool, bool, bool),
) -> PackageInfo {
	if !names.is_empty()
		&& (
			project.is_some() && needs.0,
			url.is_some() && needs.1,
			!emails.is_empty() && needs.2,
		) == needs
	{
		return PackageInfo {
			name: project,
			authors: names
				.drain(0..)
				.enumerate()
				.map(|(i, name)| Author {
					name,
					email: emails.get(i).map(|email| email.to_owned()),
				})
				.collect(),
			url,
		};
	}

	let extract_regex =
		Regex::new(r"(?P<name>[^<>()\s]+)|(?:<(?P<email>.+?)>)|(?:\(.+?\))").unwrap();

	let mut pkg = read_node(&extract_regex)
		.or_else(|| read_cargo(&extract_regex))
		.or_else(|| {
			Some(PackageInfo {
				name: None,
				authors: Vec::new(),
				url: None,
			})
		})
		.unwrap();

	drop(extract_regex);

	if pkg.authors.is_empty() {
		if !names.is_empty() {
			pkg.authors = names
				.drain(0..)
				.enumerate()
				.map(|(i, name)| Author {
					name,
					email: emails.get(i).map(|email| email.to_owned()),
				})
				.collect();
		} else {
			pkg.authors = match std::process::Command::new("git")
				.args(["config", "--get-regexp", "name|email"])
				.output()
			{
				Err(_) => None,
				Ok(output) => match String::from_utf8(output.stdout) {
					Err(_) => None,
					Ok(stdout) => read_git(stdout),
				},
			}
			.map(|author| vec![author])
			.unwrap_or(pkg.authors)
		}
	}

	if pkg.name.is_none() {
		pkg.name = project;
	}

	if pkg.url.is_none() {
		pkg.url = url;
	}

	if needs.0 && pkg.name.is_none() {
		CliError::MissingFlag {
			flag: "-p or --project".to_owned(),
			reason: "The project name could not be obtained via a Cargo.toml (Rust) or package.json (Node.js)!".to_owned()
		}
		.throw()
	} else if needs.1 && pkg.url.is_none() {
		CliError::MissingFlag {
			flag: "-u or --url".to_owned(),
			reason:
				"The url could not be obtained via a Cargo.toml (Rust) or package.json (Node.js)!"
					.to_owned(),
		}
		.throw()
	} else if pkg.authors.is_empty() {
		CliError::MissingFlag {
			flag: "-n or --name".to_owned(),
			reason: "The name(s) could not be obtained via a Cargo.toml (Rust), package.json (Node.js) or git config!".to_owned()
		}
		.throw()
	} else if needs.2 && !pkg.authors.iter().any(|author| author.email.is_some()) {
		CliError::MissingFlag {
			flag: "-e or --email".to_owned(),
			reason:
				"There must be at least one email to accompany the name(s)! Either provide it via the flag, in a Cargo.toml (Rust), or in a package.json (Node.js)!"
				.to_owned(),
			}
		.throw()
	}

	pkg
}

pub fn get_license(
	license: &str,
	emails: Vec<String>,
	names: Vec<String>,
	project: Option<String>,
	url: Option<String>,
	year: Option<String>,
) -> (String, &'static str) {
	let year = year.unwrap_or_else(|| {
		(SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap_or_else(|_| {
				CliError::MissingFlag {
					flag: "-y or --year".to_owned(),
					reason: "The current year could not be obtained from the system time!"
						.to_owned(),
				}
				.throw()
			})
			.as_secs() / 31_534_000
			+ 1970)
			.to_string()
	});

	fn drain_names(
		emails: Vec<String>,
		names: Vec<String>,
		project: Option<String>,
		url: Option<String>,
	) -> Vec<String> {
		get_info(emails, names, project, url, (false, false, false))
			.authors
			.drain(0..)
			.map(|author| author.name)
			.collect()
	}

	match license {
		"afl" => (licenses::afl::get(), "AFL-3.0"),
		"agpl" => (licenses::agpl::get(), "AGPL-3.0-only"),
		"apache_two" => (licenses::apache_two::get(), "Apache-2.0"),
		"art" => (licenses::art::get(), "Artistic-2.0"),
		"boost" => (licenses::boost::get(), "BSL-1.0"),
		"bsd_clear" => (
			licenses::bsd_clear::get(year, drain_names(emails, names, project, url)),
			"BSD-3-Clause-Clear",
		),
		"bsd_four" => (
			licenses::bsd_four::get(year, drain_names(emails, names, project, url)),
			"BSD-4-Clause",
		),
		"bsd_three" => (
			licenses::bsd_three::get(year, drain_names(emails, names, project, url)),
			"BSD-3-Clause",
		),
		"bsd_two" => (
			licenses::bsd_two::get(year, drain_names(emails, names, project, url)),
			"BSD-2-Clause",
		),
		"bsd_zero" => (
			licenses::bsd_zero::get(year, drain_names(emails, names, project, url)),
			"0BSD",
		),
		"cc" => (licenses::cc::get(), "CC-BY-4.0"),
		"cc_sa" => (licenses::cc_sa::get(), "CC-BY-SA-4.0"),
		"cc_zero" => (licenses::cc_zero::get(), "CC0-1.0"),
		"cecill" => (licenses::cecill::get(), "CECILL-2.1"),
		"ecl" => (licenses::ecl::get(), "ECL-2.0"),
		"eclipse_one" => (licenses::eclipse_one::get(), "EPL-1.0"),
		"eclipse_two" => (licenses::eclipse_two::get(), "EPL-2.0"),
		"eu_one" => (licenses::eu_one::get(), "EUPL-1.1"),
		"eu_two" => (licenses::eu_two::get(), "EUPL-1.2"),
		"fdl" => (licenses::fdl::get(), "FDL-1.3"),
		"gpl_three" => (licenses::gpl_three::get(), "GPL-3.0-only"),
		"gpl_two" => (licenses::gpl_two::get(), "GPL-2.0-only"),
		"isc" => (
			licenses::isc::get(year, drain_names(emails, names, project, url)),
			"ISC",
		),
		"latex" => (licenses::latex::get(), "LPPL-1.3c"),
		"lgpl_three" => (licenses::lgpl_three::get(), "LGPL-3.0-only"),
		"lgpl_two" => (licenses::lgpl_two::get(), "LGPL-2.1-only"),
		"micpl" => (licenses::micpl::get(), "MICROSOFT-PL"),
		"mit" => (
			licenses::mit::get(year, drain_names(emails, names, project, url)),
			"MIT",
		),
		"mit_na" => (
			licenses::mit_na::get(year, drain_names(emails, names, project, url)),
			"MIT-0",
		),
		"mozpl" => (licenses::mozpl::get(), "MPL-2.0"),
		"mrl" => (licenses::mrl::get(), "MIROSL"),
		"mulpl" => (licenses::mulpl::get(), "MULPL-1.0"),
		"ncsa" => {
			let mut pkg = get_info(emails, names, project, url, (true, true, false));

			(
				licenses::ncsa::get(
					year,
					pkg.authors.drain(0..).map(|author| author.name).collect(),
					pkg.name.unwrap(),
					pkg.url.unwrap(),
				),
				"NCSA",
			)
		}
		"odl" => (licenses::odl::get(), "ODbL-1.0"),
		"ofl" => {
			let mut pkg = get_info(emails, names, project, url, (false, false, true));

			let (mut pkg_names, mut pkg_emails) = (Vec::new(), Vec::new());

			for author in pkg.authors.drain(0..) {
				pkg_names.push(author.name);

				if let Some(email) = author.email {
					pkg_emails.push(email);
				}
			}

			(licenses::ofl::get(year, pkg_names, pkg_emails), "OFL-1.1")
		}
		"osl" => (licenses::osl::get(), "OSL-3.0"),
		"postgres" => (licenses::postgres::get(year, names), "PostgreSQL"),
		"unl" => (licenses::unl::get(), "Unlicense"),
		"upl" => (licenses::upl::get(year, names), "UPL-1.0"),
		"vim" => (licenses::vim::get(), "Vim"),
		"zlib" => (licenses::zlib::get(year, names), "Zlib"),
		l => CliError::UnknownLicense {
			license: l.to_owned(),
		}
		.throw(),
	}
}

pub fn write_pkg(license_type: &str) {
	let success = |pkg_type: &str| {
		println!(
			"{} Wrote license type '{}' to '{}'",
			"SUCCESS".green().bold(),
			license_type,
			pkg_type
		);
	};

	match read_to_string("package.json") {
		Err(err) => {
			if err.kind() != ErrorKind::NotFound {
				CliError::FailedRead {
					kind: "package.json".to_owned(),
				}
				.warn();
			}
		}
		Ok(string) => match parse(&string) {
			Err(_) => CliError::FailedParse {
				kind: "package.json".to_owned(),
			}
			.warn(),
			Ok(mut json) => {
				drop(string);

				json["license"] = license_type.into();

				write("package.json", json.pretty(4).replace("    ", "	")).unwrap_or_else(|_| {
					CliError::FailedWrite {
						path: PathBuf::from("package.json"),
					}
					.throw()
				});

				return success("package.json");
			}
		},
	}

	match read_to_string("Cargo.toml") {
		Err(err) => {
			if err.kind() != ErrorKind::NotFound {
				CliError::FailedRead {
					kind: "Cargo.toml".to_owned(),
				}
				.warn();
			}
		}
		Ok(string) => match string.parse::<Document>() {
			Err(_) => CliError::FailedParse {
				kind: "Cargo.toml".to_owned(),
			}
			.warn(),
			Ok(mut toml) => {
				drop(string);

				toml["package"]["license"] = value(license_type);

				write("Cargo.toml", toml.to_string()).unwrap_or_else(|_| {
					CliError::FailedWrite {
						path: PathBuf::from("Cargo.toml"),
					}
					.throw()
				});

				return success("Cargo.toml");
			}
		},
	}

	match read_to_string("pyproject.toml") {
		Err(err) => {
			if err.kind() != ErrorKind::NotFound {
				CliError::FailedRead {
					kind: "Cargo.toml".to_owned(),
				}
				.warn();
			}
		}
		Ok(string) => match string.parse::<Document>() {
			Err(_) => CliError::FailedParse {
				kind: "Cargo.toml".to_owned(),
			}
			.warn(),
			Ok(mut toml) => {
				drop(string);

				toml["license"] = value(license_type);

				write("pyproject.toml", toml.to_string()).unwrap_or_else(|_| {
					CliError::FailedWrite {
						path: PathBuf::from("pyproject.toml"),
					}
					.throw()
				});

				success("Cargo.toml")
			}
		},
	}
}
