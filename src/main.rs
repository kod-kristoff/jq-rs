use args::Args;

mod args;
mod app;

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(err) = Args::parse().and_then(try_main) {
        eprintln!("{}", err);
        std::process::exit(2);
    }
    // let jq_filter = matches.value_of("<jq filter>").unwrap();
    // let input = matches.value_of("FILE").unwrap_or("<stdin>");
}

fn try_main(args: Args) -> Result<()> {

    println!("jq filter: {}", args.jq_filter);
    println!("Using file '{}' as input", args.input);
    Ok(())
}
