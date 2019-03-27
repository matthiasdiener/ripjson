use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} <regex> <files> [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    println!("Ripjson v{}", VERSION.unwrap_or("unknown"));
}

fn add_options(opts: &mut Options) {
    opts.optflag("i", "ignore-case", "Search case insensitively.");
    opts.optflag("s", "sensitive-case", "Search case sensitively [default].");
    opts.optflag("h", "help", "Print this help menu.");
    opts.optflag("v", "version", "Print version.");
    opts.optopt(
        "",
        "color",
        "Color output.\nWHEN can be never, always, or auto [default].",
        "<WHEN>",
    );
}

pub fn parse_options() -> (String, Vec<String>, bool) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    add_options(&mut opts);

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_f) => {
            print_usage(&program, opts);
            std::process::exit(1)
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }

    if matches.opt_present("v") {
        print_version();
        std::process::exit(0);
    }

    let color_output = if matches.opt_present("color") {
        match matches.opt_str("color").unwrap().as_ref() {
            "never" => false,
            "always" => true,
            "auto" => atty::is(atty::Stream::Stdout),
            _ => {
                println!(
                    "Error: unknown color mode '{}' specified.\n",
                    matches.opt_str("color").unwrap()
                );
                print_usage(&program, opts);
                std::process::exit(1)
            }
        }
    } else {
        atty::is(atty::Stream::Stdout)
    };

    let ignore_case = matches.opt_present("i") && !matches.opt_present("s");

    let regex = format!(
        "{}({})",
        if ignore_case { "(?i)" } else { "" },
        if !matches.free.is_empty() {
            matches.free[0].clone()
        } else {
            println!("Error: no regex specified.\n");
            print_usage(&program, opts);
            std::process::exit(1)
        }
    );

    let files = if matches.free.len() > 1 {
        matches.free[1..].to_vec()
    } else {
        println!("Error: no files specified.\n");
        print_usage(&program, opts);
        std::process::exit(1)
    };

    (regex, files, color_output)
}
