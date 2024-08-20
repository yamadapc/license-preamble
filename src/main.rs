// MIT License
// 
// Copyright (c) 2024 Pedro Tacla Yamada
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Adds a license pre-amble to source files.
//!
//! Usage
//!
//!     license-preamble init --license MIT
//!     license-preamble add ./src
//!
//! ## License
//!
//! MIT license

use std::collections::HashMap;
use std::path::Path;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use yaml_front_matter::Document;

static LICENSE_FILES: [&str; 47] = [
    include_str!("../choosealicense.com/_licenses/0bsd.txt"),
    include_str!("../choosealicense.com/_licenses/afl-3.0.txt"),
    include_str!("../choosealicense.com/_licenses/agpl-3.0.txt"),
    include_str!("../choosealicense.com/_licenses/apache-2.0.txt"),
    include_str!("../choosealicense.com/_licenses/artistic-2.0.txt"),
    include_str!("../choosealicense.com/_licenses/blueoak-1.0.0.txt"),
    include_str!("../choosealicense.com/_licenses/bsd-2-clause.txt"),
    include_str!("../choosealicense.com/_licenses/bsd-2-clause-patent.txt"),
    include_str!("../choosealicense.com/_licenses/bsd-3-clause.txt"),
    include_str!("../choosealicense.com/_licenses/bsd-3-clause-clear.txt"),
    include_str!("../choosealicense.com/_licenses/bsd-4-clause.txt"),
    include_str!("../choosealicense.com/_licenses/bsl-1.0.txt"),
    include_str!("../choosealicense.com/_licenses/cc0-1.0.txt"),
    include_str!("../choosealicense.com/_licenses/cc-by-4.0.txt"),
    include_str!("../choosealicense.com/_licenses/cc-by-sa-4.0.txt"),
    include_str!("../choosealicense.com/_licenses/cecill-2.1.txt"),
    include_str!("../choosealicense.com/_licenses/cern-ohl-p-2.0.txt"),
    include_str!("../choosealicense.com/_licenses/cern-ohl-s-2.0.txt"),
    include_str!("../choosealicense.com/_licenses/cern-ohl-w-2.0.txt"),
    include_str!("../choosealicense.com/_licenses/ecl-2.0.txt"),
    include_str!("../choosealicense.com/_licenses/epl-1.0.txt"),
    include_str!("../choosealicense.com/_licenses/epl-2.0.txt"),
    include_str!("../choosealicense.com/_licenses/eupl-1.1.txt"),
    include_str!("../choosealicense.com/_licenses/eupl-1.2.txt"),
    include_str!("../choosealicense.com/_licenses/gfdl-1.3.txt"),
    include_str!("../choosealicense.com/_licenses/gpl-2.0.txt"),
    include_str!("../choosealicense.com/_licenses/gpl-3.0.txt"),
    include_str!("../choosealicense.com/_licenses/isc.txt"),
    include_str!("../choosealicense.com/_licenses/lgpl-2.1.txt"),
    include_str!("../choosealicense.com/_licenses/lgpl-3.0.txt"),
    include_str!("../choosealicense.com/_licenses/lppl-1.3c.txt"),
    include_str!("../choosealicense.com/_licenses/mit.txt"),
    include_str!("../choosealicense.com/_licenses/mit-0.txt"),
    include_str!("../choosealicense.com/_licenses/mpl-2.0.txt"),
    include_str!("../choosealicense.com/_licenses/ms-pl.txt"),
    include_str!("../choosealicense.com/_licenses/ms-rl.txt"),
    include_str!("../choosealicense.com/_licenses/mulanpsl-2.0.txt"),
    include_str!("../choosealicense.com/_licenses/ncsa.txt"),
    include_str!("../choosealicense.com/_licenses/odbl-1.0.txt"),
    include_str!("../choosealicense.com/_licenses/ofl-1.1.txt"),
    include_str!("../choosealicense.com/_licenses/osl-3.0.txt"),
    include_str!("../choosealicense.com/_licenses/postgresql.txt"),
    include_str!("../choosealicense.com/_licenses/unlicense.txt"),
    include_str!("../choosealicense.com/_licenses/upl-1.0.txt"),
    include_str!("../choosealicense.com/_licenses/vim.txt"),
    include_str!("../choosealicense.com/_licenses/wtfpl.txt"),
    include_str!("../choosealicense.com/_licenses/zlib.txt"),
];

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize LICENSE and PREAMBLE files
    Init {
        /// The license name
        license: String,
    },
    /// List available licenses
    List,
    /// Add the preamble to files
    Add {
        source_root: Option<Vec<String>>,
    },
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct LicenseInfo {
    title: String,
    description: String,
    how: String,
    using: Option<HashMap<String, String>>,
    permissions: Vec<String>,
    conditions: Vec<String>,
    limitations: Vec<String>,
    featured: Option<bool>,
    #[serde(rename = "spdx-id")]
    spdx_id: String,
}

fn main() {
    let cli = Cli::parse();

    let licenses: Vec<Document<LicenseInfo>> = LICENSE_FILES.iter().map(|license| yaml_front_matter::YamlFrontMatter::parse(license).unwrap()).collect();
    let license_path = Path::new("LICENSE");
    let preamble_path = Path::new("PREAMBLE");

    match cli.command {
        Commands::Init { license } => {
            let license_document = licenses.iter().find(|document| {
                let info = &document.metadata;
                info.title == license || info.spdx_id == license
            }).expect("Invalid license, list available licenses with `list`");

            if !license_path.exists() {
                std::fs::write(license_path, &license_document.content.trim()).unwrap();
            } else {
                eprintln!("Refusing to overwrite LICENSE file")
            }
            if !preamble_path.exists() {
                #[allow(deprecated)]
                std::fs::soft_link(license_path, preamble_path).unwrap();
            } else {
                eprintln!("Refusing to overwrite PREAMBLE file")
            }
        }
        Commands::List => {
            for document in licenses {
                let info = document.metadata;
                println!("{:<60}   -  short:  {:}", info.title, info.spdx_id);
            }
        }
        Commands::Add { source_root } => {
            if !preamble_path.exists() {
                panic!("Run init first");
            }
            let source_roots = source_root.unwrap_or_else(|| vec![
                String::from("src"),
                String::from("lib")
            ]);
            let extensions = vec![
                ("rs", "//"),
                ("swift", "//"),
                ("js", "//"),
                ("ts", "//"),
                ("tsx", "//"),
                ("jsx", "//"),
            ];

            let preamble_contents = std::fs::read_to_string(preamble_path).unwrap();

            for source_root in source_roots {
                if std::fs::metadata(&source_root).is_err() {
                    continue;
                }

                let walk = jwalk::WalkDir::new(source_root);
                for file in walk {
                    let file = file.unwrap();
                    let path = file.path();
                    if !file.file_type.is_file() {
                        continue;
                    }

                    if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                        if let Some((_, comment_syntax)) = extensions.iter().find(|(e, _)| *e == extension) {
                            let prefixed_preamble = preamble_contents.lines().map(|line| {
                                format!("{comment_syntax} {line}").trim().to_string()
                            }).collect::<Vec<String>>().join("\n");
                            let file_contents = std::fs::read_to_string(&path).unwrap();
                            if file_contents.contains(&prefixed_preamble) {
                                eprintln!("Skipping {path:?}");
                                continue;
                            }


                            println!("Adding preamble to file {path:?}");
                            let new_contents = format!("{prefixed_preamble}\n\n{file_contents}");
                            std::fs::write(&path, new_contents).unwrap();
                        }
                    }
                }
            }
        }
    }
}
