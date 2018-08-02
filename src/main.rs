extern crate docopt;
extern crate vstatus;
#[macro_use]
extern crate serde_derive;

const USAGE: &str = "
Usage: vstatus <format>
       vstatus -h | --help
       vstatus --version

Arguments:
    format      Version control status string format

Options:
    -h, --help  Show this screen.
    --version   Show version.
";

#[derive(Deserialize)]
struct Args {
    arg_format: String,
    flag_version: bool,
}

fn main() {

}
