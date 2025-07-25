use clap::{Arg, Command};
use std::fs;
use std::io::{self, Read};
use cirko::{cyr_to_lat, lat_to_cir};

fn main() -> io::Result<()> {
    let matches = Command::new("ћирко")
        .disable_help_flag(true)
        .arg(Arg::new("help")
             .short('h')  // Остављамо -h због компатибилности
             .long("помоћ")
             .help("Прикажи помоћ")
             .action(clap::ArgAction::Help))
        .about("Ћирко - конвертор српске латинице у ћирилицу и обрнуто.")
        .arg(Arg::new("улаз")
             .short('у')
             .long("улаз")
             .value_name("FILE")
             .help("Улазни фајл (stdin подразумевано)"))
        .arg(Arg::new("излаз")
             .short('и')
             .long("излаз")
             .value_name("FILE")
             .help("Излазни фајл (stdout подразумевано)"))
        .arg(Arg::new("латиница")
             .short('л')
             .long("латиница")
             .help("Конвертуј у латиницу"))
        .arg(Arg::new("ћирилица")
             .short('ћ')
             .long("ћирилица")
             .help("Конвертуј у ћирилицу"))
        .get_matches();

    let input = if let Some(file) = matches.get_one::<String>("улаз") {
        fs::read_to_string(file)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Смер конерзије се може задати опцијама команде
    let output = if matches.contains_id("латиница") {
        crate::cyr_to_lat(&input)
    } else if matches.contains_id("ћирилица") {
        crate::lat_to_cir(&input)
    } else {
        // Аутоматска детекција смера конверзије
        if input.chars().any(|c| ('а'..='ш').contains(&c) || ('А'..='Ш').contains(&c)) {
            crate::cyr_to_lat(&input)
        } else {
            crate::lat_to_cir(&input)
        }
    };

    if let Some(file) = matches.get_one::<String>("излаз") {
        fs::write(file, output)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}
