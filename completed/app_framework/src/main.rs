// ============================================================================
// <one line to give the program's name and a brief idea of what it does.>
//
//  Copyright (C) <yyyy> <Author Name> <author@mail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// ============================================================================


// ============================================================================
//
// <Put documentation here>
//
// <yyyy>-<mm>-<dd> <Author Name> <author@mail.com>
//
// * main.rs: created.
//
// ============================================================================


// ============================================================================
//
// References (this section should be deleted in the release version)
//
// * For coding style visit Rust Style Guide page at
//   <https://doc.rust-lang.org/beta/style-guide/index.html>.
//
// * For command line arguments parsing using clap consult documentation at
//   <https://docs.rs/clap/latest/clap/>
//
//   and examples at
//   <https://github.com/clap-rs/clap/tree/master/examples>
//
// ============================================================================


// ============================================================================
// Used libraries section
// ============================================================================
use const_format::formatcp;
use clap::{Command};


// ============================================================================
// Global constants section
// ============================================================================

const APP_NAME:     &str = "app_framework";
const VERSION:      &str = "0.1.0";
const YEAR:         &str = "yyyy";
const AUTHOR:       &str = "Ljubomir Kurij";
const AUTHOR_EMAIL: &str = "ljubomir_kurij@protonmail.com";
const APP_ABOUT:    &str = "\
Framework for developing command line applications using \'clap\' command\n\
line argument parsing library.\n\nMandatory arguments to long options are \
mandatory for short options too.";
const APP_LICENSE:  &str = "\
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>\n\
This is free software: you are free to change and redistribute it.\n\
There is NO WARRANTY, to the extent permitted by law.\n";


// ============================================================================
// Main function section
// ============================================================================

fn main() {
    let matches = Command::new(APP_NAME)
        .version(VERSION)
        .author(formatcp!("{} <{}>", AUTHOR, AUTHOR_EMAIL))
        .about(APP_ABOUT)
        .after_help(formatcp!("Report bugs to <{AUTHOR_EMAIL}>"))
        .long_version(
            formatcp!(
                "{} Copyright (C) {} {}\n{}",
                VERSION,
                YEAR,
                AUTHOR,
                APP_LICENSE
                )
            )
        .get_matches();

    println!("{}: Hello, world!", APP_NAME);

}


// ============================================================================
// User defined types section
// ============================================================================


// ============================================================================
// User defined functions section
// ============================================================================


