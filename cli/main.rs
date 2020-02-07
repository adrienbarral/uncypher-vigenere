extern crate clap;
use clap::{Arg, App};
use std::fs;

mod cyphered_text_formater;
mod key_length_estimator;

fn main() -> std::io::Result<()> {
    let matches = App::new("Vigenère Uncypher")
                          .version("1.0")
                          .author("Adrien BARRAL <adrien.barral@gmail.com>")
                          .about("Uncypher Vigenère text")
                          .arg(Arg::with_name("--language")
                               .short("-l")
                               .long("--language")
                               .value_name("fr")
                               .help("Choose the language (currently only fr)")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                                .required(true)
                        ).get_matches();

    let file_name = matches.value_of("INPUT").unwrap();
    let cyphered_text = fs::read_to_string(file_name)?;
    let _simplified_cyphered_text = cyphered_text_formater::format_cyphered_text(&cyphered_text);

    Ok(())
}
