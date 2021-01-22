use clap::App;


fn main() {
    println!("Hello, world!");
    App::new("jq-rs")
        .version("0.1.0")
        .author("Kristoffer Andersson <kod.kristoff@gmail.com>")
        .get_matches();
}
