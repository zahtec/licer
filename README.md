# Licer

A simple CLI for quickly generating repository licenses.

## Installation

Licer is installable via two distinct scripts for macOS/Linux and Windows devices respectively

**macOS/Linux**

```sh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/zahtec/licer/HEAD/install.sh)"
```

**Windows (PowerShell)**

```sh
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser # Required only if running a remote script for the first time
irm "https://raw.githubusercontent.com/zahtec/licer/HEAD/install.ps1" | iex
```

There are also various platform binaries availible for download on the [releases](https://github.com/zahtec/licer/releases) page.

## Usage

Usage will output if you run `licer`, `licer -h` or `licer --help`

```
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

Version: 0.1.0
```

## Feature Requests

If you have feature requests for licer, please do not hesitate and create a
new issue with the "enhancement" tag.
