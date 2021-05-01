mod cli;



fn main() {
    let matches = cli::build_cli().get_matches();
    let jq_filter = matches.value_of("<jq filter>").unwrap();
    println!("jq filter: {}", jq_filter);
    let input = matches.value_of("FILE").unwrap_or("<stdin>");
    println!("Using file '{}' as input", input);
}
