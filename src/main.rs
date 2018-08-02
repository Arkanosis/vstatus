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
    let args: Args =
        docopt::Docopt::new(USAGE)
            .and_then(|docopts|
                docopts.argv(std::env::args().into_iter())
                   .deserialize()
            )
            .unwrap_or_else(|error|
                error.exit()
            );

    if args.flag_version {
        println!("vstatus v{}", vstatus::version());
    } else {
        let exit_code = vstatus::run(&args.arg_format);
        if exit_code != 0 {
            std::process::exit(exit_code)
        }
    }
}
