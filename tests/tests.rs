use licer::{get_license, read_cargo, read_git, read_node, write_pkg};
use regex::Regex;
use std::{
	env::{set_current_dir, temp_dir},
	fs::{read_to_string, remove_file, write},
};

fn author_regex() -> Regex {
	Regex::new(r"(?P<name>[^<>()\s]+)|(?:<(?P<email>.+?)>)|(?:\(.+?\))").unwrap()
}

fn temp_file(name: &'static str, contents: &'static str) -> impl FnOnce() {
	set_current_dir(temp_dir()).unwrap();

	write(name, contents.trim()).unwrap();

	move || remove_file(name).unwrap()
}

#[test]
fn node_package_multiple_authors() {
	let regex = author_regex();

	let del_file = temp_file(
		"package.json",
		r#"
		{
			"author": "John Doe <johndoe@gmail.com> (https://johndoe.com)",
			"contributors": [
				"<janedoe@gmail.com> Jane Doe (https://janedoe.com)",
				{
					"name": "John Cena",
					"email": "johncena@gmail.com"
				}
			]
		}"#,
	);

	let authors = read_node(&regex).unwrap().authors;

	del_file();

	assert_eq!(authors.len(), 3);
	assert_eq!(authors[0].name, "John Doe");
	assert_eq!(authors[0].email, Some("johndoe@gmail.com".to_owned()));
	assert_eq!(authors[1].name, "Jane Doe");
	assert_eq!(authors[1].email, Some("janedoe@gmail.com".to_owned()));
	assert_eq!(authors[2].name, "John Cena");
	assert_eq!(authors[2].email, Some("johncena@gmail.com".to_owned()));
}

#[test]
fn node_package_singular_author() {
	let regex = author_regex();

	let del_file = temp_file(
		"package.json",
		r#"
		{
			"author": {
				"name": "John Doe",
				"email": "johndoe@yahoo.org",
				"url": "https://johndoe.rs"
			}
		}"#,
	);

	let authors = read_node(&regex).unwrap().authors;

	del_file();

	assert_eq!(authors.len(), 1);
	assert_eq!(authors[0].name, "John Doe");
	assert_eq!(authors[0].email, Some("johndoe@yahoo.org".to_owned()));
}

#[test]
fn cargo_package_multiple_authors() {
	let regex = author_regex();

	let del_file = temp_file(
		"Cargo.toml",
		r#"
		[package]
		authors = ["John Appleseed (https://johnny.com) <johnappleseed@microsoft.us>", "Jane"]"#,
	);

	let authors = read_cargo(&regex).unwrap().authors;

	del_file();

	assert_eq!(authors.len(), 2);
	assert_eq!(authors[0].name, "John Appleseed");
	assert_eq!(
		authors[0].email,
		Some("johnappleseed@microsoft.us".to_owned())
	);
	assert_eq!(authors[1].name, "Jane");
	assert_eq!(authors[1].email, None);
}

#[test]
fn cargo_package_singular_author() {
	let regex = author_regex();

	let del_file = temp_file(
		"Cargo.toml",
		r#"
		[package]
		authors = ["<johndoe@gmail.com> John Doe"]
		"#,
	);

	let authors = read_cargo(&regex).unwrap().authors;

	del_file();

	assert_eq!(authors.len(), 1);
	assert_eq!(authors[0].name, "John Doe");
	assert_eq!(authors[0].email, Some("johndoe@gmail.com".to_owned()));
}

#[test]
fn mit_flag_provided() {
	assert_eq!(
		get_license(
			"mit",
			Vec::new(),
			vec!["John".to_owned()],
			None,
			None,
			Some("2025".to_owned())
		)
		.0
		.trim(),
		r#"
MIT License

Copyright (c) 2025 John

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."#
			.trim()
	);
}

#[test]
fn mit_git() {
	let author = read_git(
		r#"
		user.email jane.user.email@yahoo.net
		user.name Jane Janes"#
			.to_owned(),
	)
	.unwrap();

	assert_eq!(
		get_license(
			"mit",
			Vec::new(),
			vec![author.name],
			None,
			None,
			Some("2025".to_owned())
		)
		.0
		.trim(),
		r#"
MIT License

Copyright (c) 2025 Jane Janes

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."#
			.trim()
	);
}

#[test]
fn bsd_two_node_package() {
	let del_file = temp_file(
		"package.json",
		r#"
		{
			"author": {
				"name": "Jane Doe"
			},
			"contributors": [
				"John Doe",
				{
					"name": "John Cena"
				}
			]
		}"#,
	);

	let license = get_license(
		"bsd_two",
		Vec::new(),
		Vec::new(),
		None,
		None,
		Some("1983".to_owned()),
	);

	del_file();

	assert_eq!(
		license.0.trim(),
		r#"
BSD 2-Clause License

Copyright (c) 1983, Jane Doe, John Doe, John Cena

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
this list of conditions and the following disclaimer in the documentation
and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."#
			.trim()
	)
}

#[test]
fn bsd_cargo_package() {
	let del_file = temp_file(
		"Cargo.toml",
		r#"
[package]
name = "licar"
version = "0.1.0"
authors = ["John Doe", "<janedoe@gmail.com> Jane Doe"]"#,
	);

	let license = get_license(
		"bsd_two",
		Vec::new(),
		Vec::new(),
		None,
		None,
		Some("2017".to_owned()),
	);

	del_file();

	assert_eq!(
		license.0.trim(),
		r#"
BSD 2-Clause License

Copyright (c) 2017, John Doe, Jane Doe

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
this list of conditions and the following disclaimer in the documentation
and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."#
			.trim()
	)
}

#[test]
fn ncsa_flag_provded() {
	assert_eq!(
		get_license(
			"ncsa",
			Vec::new(),
			vec!["John".to_owned(), "Jane".to_owned()],
			Some("LICER".to_owned()),
			Some("https://github.com".to_owned()),
			Some("2025".to_owned())
		)
		.0
		.trim(),
		r#"
University of Illinois/NCSA Open Source License

Copyright (c) 2025 John, Jane. All rights reserved.

Developed by: LICER
              John, Jane
              https://github.com

Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation files
(the "Software"), to deal with the Software without restriction,
including without limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of the Software,
and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

* Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimers.

* Redistributions in binary form must reproduce the above copyright
    notice, this list of conditions and the following disclaimers in the
    documentation and/or other materials provided with the distribution.

* Neither the names of [fullname], [project] nor the names of its
    contributors may be used to endorse or promote products derived from
    this Software without specific prior written permission.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
CONTRIBUTORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS WITH
THE SOFTWARE."#
			.trim()
	);
}

#[test]
fn ncsa_node_package() {
	let del_file = temp_file(
		"package.json",
		r#"
		{
			"name": "gnu-js",
			"homepage": "https://js.gnu.org",
			"author": {
				"name": "Linus Torvalds"
			},
			"contributors": [
				{
					"name": "John Cena",
					"url": "https://johncena.com"
				},
				"(https://jane.org) Jane Doe"
			]
		}
		"#,
	);

	let license = get_license(
		"ncsa",
		Vec::new(),
		Vec::new(),
		None,
		None,
		Some("2000".to_owned()),
	);

	del_file();

	assert_eq!(
		license.0.trim(),
		r#"
University of Illinois/NCSA Open Source License

Copyright (c) 2000 Linus Torvalds, John Cena, Jane Doe. All rights reserved.

Developed by: gnu-js
              Linus Torvalds, John Cena, Jane Doe
              https://js.gnu.org

Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation files
(the "Software"), to deal with the Software without restriction,
including without limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of the Software,
and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

* Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimers.

* Redistributions in binary form must reproduce the above copyright
    notice, this list of conditions and the following disclaimers in the
    documentation and/or other materials provided with the distribution.

* Neither the names of [fullname], [project] nor the names of its
    contributors may be used to endorse or promote products derived from
    this Software without specific prior written permission.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
CONTRIBUTORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS WITH
THE SOFTWARE."#
			.trim()
	);
}

#[test]
fn ncsa_cargo_package() {
	let del_file = temp_file(
		"Cargo.toml",
		r#"
		[package]
        name = "gnu-rs"
        homepage = "https://rs.gnu.org"
        authors = ["(https://zahtec.com) Zahtec", "John"]
		"#,
	);

	let license = get_license(
		"ncsa",
		Vec::new(),
		Vec::new(),
		None,
		None,
		Some("2000".to_owned()),
	);

	del_file();

	assert_eq!(
		license.0.trim(),
		r#"
University of Illinois/NCSA Open Source License

Copyright (c) 2000 Zahtec, John. All rights reserved.

Developed by: gnu-rs
              Zahtec, John
              https://rs.gnu.org

Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation files
(the "Software"), to deal with the Software without restriction,
including without limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of the Software,
and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

* Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimers.

* Redistributions in binary form must reproduce the above copyright
    notice, this list of conditions and the following disclaimers in the
    documentation and/or other materials provided with the distribution.

* Neither the names of [fullname], [project] nor the names of its
    contributors may be used to endorse or promote products derived from
    this Software without specific prior written permission.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
CONTRIBUTORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS WITH
THE SOFTWARE."#
			.trim()
	);
}

#[test]
fn git_singular_author() {
	let author = read_git(
		r#"
		user.name John Doe
		user.email johndoe@hotmail.io"#
			.to_owned(),
	)
	.unwrap();

	assert_eq!(author.name, "John Doe");
	assert_eq!(author.email.unwrap(), "johndoe@hotmail.io");
}

#[test]
fn node_package_mit_write() {
	let del_file = temp_file(
		"package.json",
		r#"
        {
            "name": "gnu-js",
            "author": {
                "name": "John Doe"
            }
        }
        "#,
	);

	write_pkg("MIT");

	assert_eq!(
		read_to_string("package.json").unwrap().trim(),
		r#"
{
	"name": "gnu-js",
	"author": {
		"name": "John Doe"
	},
	"license": "MIT"
}"#
		.trim()
	);

	del_file();
}

#[test]
fn cargo_package_mit_write() {
	let del_file = temp_file(
		"Cargo.toml",
		r#"
[package]
authors = ["John Appleseed (https://johnny.com) <johnappleseed@microsoft.us>", "Jane"]"#,
	);

	write_pkg("MIT");

	assert_eq!(
		read_to_string("Cargo.toml").unwrap().trim(),
		r#"
[package]
authors = ["John Appleseed (https://johnny.com) <johnappleseed@microsoft.us>", "Jane"]
license = "MIT""#
			.trim()
	);

	del_file();
}
