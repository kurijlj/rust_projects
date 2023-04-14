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


// ============================================================================
// Main function section
// ============================================================================

fn main() {
    println!(
        "{} {} {} by {} <{}>\n",
        AppAbout::name(),
        AppAbout::version(),
        AppAbout::year(),
        AppAbout::author(),
        AppAbout::author_email()
        );
    println!("{}", AppAbout::doc());
    println!("{}", AppAbout::license());
}


// ============================================================================
// User defined types section
// ============================================================================
struct AppAbout {
}

impl AppAbout {
    #[inline]
    pub fn name() -> &'static str {
        return "app_name";
    }

    pub fn version() -> &'static str {
        return "0.1.0";
    }

    pub fn year() -> &'static str {
        return "yyyy";
    }

    pub fn author() -> &'static str {
        return "Ljubomir Kurij";
    }

    pub fn author_email() -> &'static str {
        return "ljubomir_kurij@protonmail.com";
    }

    pub fn doc() -> &'static str {
        return "Framework for developing command line applications using \
            \'clap\' command\nline argument parsing library.\n\nMandatory \
            arguments to long options are mandatory for short options too.\n";

    }

    pub fn license() -> &'static str {
        return "License GPLv3+: GNU GPL version 3 or later \
            <http://gnu.org/licenses/gpl.html>\nThis is free software: you are \
            free to change and redistribute it.\nThere is NO WARRANTY, to the \
            extent permitted by law.\n";

    }

}


// ============================================================================
// User defined functions section
// ============================================================================

