use clap::{App, Arg};
use clap::{crate_authors, crate_description, crate_name, crate_version};


pub fn app() -> App<'static, 'static>  {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("<jq filter>")
             .help("the filter to apply")
             .required(true))
        .arg(Arg::with_name("FILE")
             .help("the input to work on"))
}
