use colored::Colorize;
use licer::{get_license, write_pkg, CliError};
use std::{fs::write, path::PathBuf};

fn main() {
	let args: Vec<String> = std::env::args().collect();

	match (args.get(1), &"-h".to_owned()) {
		(None, arg) | (Some(arg), _) if arg == "-h" || arg == "--help" => {
			println!(
				r#"
Usage: licer <license>
[-d | --directory <directory>]
[-e | --email <email> (repeated)]
[-f | --file <file name>]
[-h | --help (independent)]
[-n | --name <name> (repeated)]
[-p | --project <project name>]
[-u | --url <project url>]
[-v | --version (independent)]
[-y | --year <year>]

Repeated flags are used to define multiple authors which will be inserted in-order from left to right
Example: licer mit --name Zahtec --email email@example.com -n Fireship -e email@fireship.io
The name "Zahtec" will be associated with the email "email@example.com" and "Fireship" "email@fireship.io"

Licer will automatically grab the year using the current year set on the operating system
Licer will automatically grab your name and email via the local or global git config
Licer will automatically grab the project name and url via a Cargo.toml (Rust), package.json (Node.js), or pyproject.toml (Python)
Licer will automatically write the license type to a Cargo.toml (Rust), package.json (Node.js), or pyproject.toml (Python)

List of licenses:

Licer Name     Full Name                                                     Info Required
------------------------------------------------------------------------------------------------------------------
afl            Academic Free License                                         None
agpl           GNU Affero General Public License v3.0                        None
apache_two     Apache License 2.0                                            None
art            Artistic License 2.0                                          None
boost          Boost Software License 1.0                                    None
bsd_clear      BSD 3-Clause Clear License                                    Year, Name(s)
bsd_four       BSD 4-Clause “Original” or “Old” License                      Year, Name(s)
bsd_three      BSD 3-Clause “New” or “Revised” License                       Year, Name(s)
bsd_two        BSD 2-Clause “Simplified” License                             Year, Name(s)
bsd_zero       BSD Zero Clause License                                       Year, Name(s)
cc             Creative Commons Attribution 4.0 International                None
cc_sa          Creative Commons Attribution Share Alike 4.0 International    None
cc_zero        Creative Commons Zero v1.0 Universal                          None
cecill         CeCILL Free Software License Agreement v2.1                   None
ecl            Educational Community License v2.0                            None
eclipse_one    Eclipse Public License 1.0                                    None
eclipse_two    Eclipse Public License 2.0                                    None
eu_one         European Union Public License 1.1                             None
eu_two         European Union Public License 1.2                             None
fdl            GNU Free Documentation License v1.3                           None
gpl_three      GNU General Public License v3.0                               None
gpl_two        GNU General Public License v2.0                               None
isc            ISC License                                                   Year, Name(s)
latex          LaTeX Project Public License v1.3c                            None
lgpl_three     GNU Lesser General Public License v3.0                        None
lgpl_two       GNU Lesser General Public License v2.1                        None
micpl          Microsoft Public License                                      None
mit            MIT License                                                   Year, Name(s)
mit_na         MIT No Attribution                                            Year, Name(s)
mozpl          Mozilla Public License 2.0                                    None
mrl            Microsoft Reciprocal License                                  None
mulpl          Mulan Permissive Software License, Version 2                  None
ncsa           University of Illinois/NCSA Open Source License               Year, Name(s), Project Name, Project URL
odl            Open Data Commons Open Database License v1.0                  None
ofl            SIL Open Font License 1.1                                     Year, Name(s), Email(s)
osl            Open Software License 3.0                                     None
postgres       PostgreSQL License                                            Year, Name(s)
unl            The Unlicense                                                 None
upl            Universal Permissive License v1.0                             Year, Name(s)
vim            Vim License                                                   None
zlib           zlib License                                                  Year, Name(s)

Version: {}"#,
				env!("CARGO_PKG_VERSION")
			);

			return;
		}
		(Some(arg), _) if arg == "-v" || arg == "--version" => {
			println!("Licer v{}", env!("CARGO_PKG_VERSION"));

			return;
		}
		_ => (),
	}

	#[allow(clippy::type_complexity)]
	let (mut dir, mut emails, mut file, mut names, mut project, mut url, mut year): (
		Option<String>,
		Vec<String>,
		Option<String>,
		Vec<String>,
		Option<String>,
		Option<String>,
		Option<String>,
	) = (None, Vec::new(), None, Vec::new(), None, None, None);

	{
		let mut skip = false;

		let get = |i: usize, flag: &str| match args.get(i + 1) {
			Some(arg) => Some(arg.to_owned()),
			None => CliError::InvalidFlag {
				flag: flag.to_owned(),
				reason: "Please provide a value!".to_owned(),
			}
			.throw(),
		};

		for (i, arg) in args.iter().enumerate() {
			if skip || i < 2 {
				skip = false;
				continue;
			}

			match arg.as_str() {
				"-d" | "--directory" => dir = get(i, "directory"),
				"-e" | "--email" => emails.push(get(i, "email").unwrap()),
				"-f" | "--file" => file = get(i, "file name"),
				"-n" | "--name" => names.push(get(i, "name").unwrap()),
				"-p" | "--project" => project = get(i, "project name"),
				"-u" | "--url" => url = get(i, "project url"),
				"-v" | "--version" => {
					println!("Licer version {}", env!("CARGO_PKG_VERSION"));
					return;
				}
				"-y" | "--year" => year = get(i, "year"),
				flag if flag.starts_with("--") && flag.contains('=') => {
					let split = flag.split('=').collect::<Vec<&str>>();
					let arg = split[1].to_owned();

					if arg.is_empty() {
						CliError::InvalidFlag {
							flag: flag[2..flag.len() - 1].to_owned(),
							reason: "Please provide a value!".to_owned(),
						}
						.throw()
					}

					match split[0] {
						"--directory" => dir = Some(arg),
						"--email" => emails.push(arg),
						"--file" => file = Some(arg),
						"--name" => names.push(arg),
						"--project" => project = Some(arg),
						"--url" => url = Some(arg),
						"--year" => year = Some(arg),
						_ => CliError::UnknownFlag {
							flag: flag.to_owned(),
						}
						.throw(),
					}
				}
				flag if flag.starts_with('-') || flag.starts_with("--") => CliError::UnknownFlag {
					flag: flag.to_owned(),
				}
				.throw(),
				arg => CliError::UnknownArg {
					arg: arg.to_owned(),
				}
				.throw(),
			}

			skip = true;
		}
	}

	let dir = dir.map(PathBuf::from);
	let file = file.map(PathBuf::from);

	if dir.is_some() && !dir.as_ref().unwrap().is_dir() {
		CliError::InvalidFlag {
			flag: "directory".to_owned(),
			reason: format!(
				"The provided directory '{}' is not a directory! Please provide the file name with the -f or --file flag!",
				dir.unwrap().to_str().unwrap()
			),
		}
		.throw()
	}

	if file.is_some() && !file.as_ref().unwrap().is_file() {
		CliError::InvalidFlag {
			flag: "file name".to_owned(),
			reason: format!(
				"The provided file name '{}' contains a directory! Please provide the directory with the -d or --directory flag!",
				file.unwrap().to_str().unwrap()
			)
		}
		.throw()
	}

	if year.is_some() && !year.as_ref().unwrap().chars().all(char::is_numeric) {
		CliError::InvalidFlag {
			flag: "year".to_owned(),
			reason: format!(
				"The provided year '{}' contains non-numeric characters!",
				year.unwrap()
			),
		}
		.throw()
	}

	let (license, license_type) = get_license(&args[1], emails, names, project, url, year);

	let path = dir
		.unwrap_or_else(|| PathBuf::from("./"))
		.join(file.unwrap_or_else(|| PathBuf::from("LICENSE")));

	match write(&path, (license + "\n").trim_start()) {
		Ok(_) => {
			println!(
				"{} Wrote license '{}' at '{}'",
				"SUCCESS".green().bold(),
				args[1],
				path.to_str().unwrap()
			);
		}
		Err(_) => CliError::FailedWrite { path }.throw(),
	}

	write_pkg(license_type);
}
