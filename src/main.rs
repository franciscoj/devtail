#[macro_use]
extern crate clap;

use clap::Arg;
use devtail::tail::{TailF, TailS};

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("file")
                .short("f")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    if let Some(file_name) = matches.value_of("file") {
        let tail = TailF::new(file_name.to_string());

        tail.run()
    } else {
        let stdin = std::io::stdin();
        let tail = TailS::new(&stdin);

        tail.run()
    }
}
