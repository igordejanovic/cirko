use phf::{phf_map, phf_set};

static CYR_TO_LAT: phf::Map<char, &'static str> = phf_map! {
    'а' => "a",
    'б' => "b",
    'в' => "v",
    'г' => "g",
    'д' => "d",
    'ђ' => "đ",
    'е' => "e",
    'ж' => "ž",
    'з' => "z",
    'и' => "i",
    'ј' => "j",
    'к' => "k",
    'л' => "l",
    'љ' => "lj",
    'м' => "m",
    'н' => "n",
    'њ' => "nj",
    'о' => "o",
    'п' => "p",
    'р' => "r",
    'с' => "s",
    'т' => "t",
    'ћ' => "ć",
    'у' => "u",
    'ф' => "f",
    'х' => "h",
    'ц' => "c",
    'ч' => "č",
    'џ' => "dž",
    'ш' => "š",
};

static LAT_TO_CYR: phf::Map<&'static str, char> = phf_map! {
    "a" => 'а',
    "b" => 'б',
    "v" => 'в',
    "g" => 'г',
    "d" => 'д',
    "đ" => 'ђ',
    "e" => 'е',
    "ž" => 'ж',
    "z" => 'з',
    "i" => 'и',
    "j" => 'ј',
    "k" => 'к',
    "l" => 'л',
    "m" => 'м',
    "n" => 'н',
    "o" => 'о',
    "p" => 'п',
    "r" => 'р',
    "s" => 'с',
    "t" => 'т',
    "ć" => 'ћ',
    "u" => 'у',
    "f" => 'ф',
    "h" => 'х',
    "c" => 'ц',
    "č" => 'ч',
    "š" => 'ш',

    // Мапирање двословних секвенци
    "dž" => 'џ',
    "lj" => 'љ',
    "nj" => 'њ',
};

// Изузеци преузети из OOOTranslit екстензије за Либре Офис: https://extensions.libreoffice.org/en/extensions/show/oootranslit
static EXCEPTIONS: phf::Set<&'static str> = phf_set! {
    "tanjug",
    "adžive",
    "nadže",
    "odžive",
    "odžvaka",
    "odžuri",
    "džubori",
    "onjugacij",
    "njukcij",
    "njekcij",
    "anjezičn",
};
// Дужина најдужег изузетка
const MAX_EXCEPTION_LEN: usize = 9;

/// Конвертује дато ћирилично слово у латинични еквивалент
fn cyr_to_lat_char(c: char) -> Option<&'static str> {
    CYR_TO_LAT.get(&c).copied()
}

/// Конверзија српске ћирилице на латиницу
pub fn cyr_to_lat(input: &str) -> String {
    let mut output = String::with_capacity(input.len() * 2); // Латинични облик може бити већи
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        let is_upper = c.is_uppercase();
        let c_low = c.to_lowercase().next().unwrap();
        match cyr_to_lat_char(c_low) {
            Some(lat) => {
                let converted_chars = lat.chars().collect::<Vec<char>>();

                if is_upper {
                    // ако је прво ћирилично слово било велико прво слово латинице
                    // ће увек бити велико.
                    output.push_str(&converted_chars[0].to_uppercase().collect::<String>());
                } else {
                   output.push(converted_chars[0])
                }

                // Ако је двословна секвенца
                if converted_chars.len() > 1 {
                    if let Some(&c_next) = chars.peek() {
                        if c_next.is_uppercase() {
                            // Ако је ћирилично слово које следи велико тада ће
                            // и друго слово латинице бити велико
                            output.push_str(&converted_chars[1].to_uppercase().collect::<String>());
                            continue;
                        }
                    }
                    output.push(converted_chars[1]);
                }
            },
            None => output.push(c), // Ако није српска ћирилица не конвертуј,
        }
    }
    output
}

/// Конверзија српске латинице на ћирилицу
pub fn lat_to_cir(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.char_indices().peekable();
    let mut skip_until = 0; // Колико карактера да прескочимо до следеће провере изузетака

    while let Some((pos, c)) = chars.next() {
        // Ако смо већ нашли изузетак радимо нормалну карактер-по-карактер транслацију
        // за дужину изузетка.
        if pos < skip_until {
            process_char(c, &mut chars, &mut output, false);
            continue;
        }

        // Провера изузетака
        let remaining_len = input.len() - pos;
        let check_len = std::cmp::min(MAX_EXCEPTION_LEN, remaining_len);
        let mut found_exception = None;

        for len in (1..=check_len).rev() {
            if let Some(substr) = input.get(pos..pos+len) {
                if EXCEPTIONS.contains(substr.to_lowercase().as_str()) {
                    found_exception = Some(len);
                    break;
                }
            }
        }

        if let Some(len) = found_exception {
            skip_until = pos + len;
            process_char(c, &mut chars, &mut output, false);
        } else {
            process_char(c, &mut chars, &mut output, true);
        }
    }

    output
}

fn process_char(
    c: char,
    chars: &mut std::iter::Peekable<std::str::CharIndices>,
    output: &mut String,
    doubles: bool,
) {
    let mut buffer = String::new();
    buffer.push(c.to_lowercase().next().unwrap());

    // Провера двословних секвенци
    if doubles {
        if let Some(&(_, next_c)) = chars.peek() {
            buffer.push(next_c.to_lowercase().next().unwrap());

            if let Some(&cyr) = LAT_TO_CYR.get(&buffer[..]) {
                // Очувај величину слова
                output.push(if c.is_uppercase() {
                    cyr.to_uppercase().next().unwrap()
                } else {
                    cyr
                });
                chars.next(); // прескочи друго слово
                return;
            }
            buffer.pop(); // скини друго слово ако није препознато
        }
    }

    // Провера једнословних секвенци
    if let Some(&cyr) = LAT_TO_CYR.get(&buffer[..]) {
        // Очувај величину слова
        output.push(if c.is_uppercase() {
            cyr.to_uppercase().next().unwrap()
        } else {
            cyr
        });
    } else {
        output.push(c);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cir_to_lat() {
        assert_eq!(
            "Čiča Đura žvaće šljive, njegova ćerka Ljilja jede džem",
            cyr_to_lat("Чича Ђура жваће шљиве, његова ћерка Љиља једе џем")
        );
        assert_eq!(
            "Čokančićem ću te, čokančićem ćeš me!!",
            cyr_to_lat("Чоканчићем ћу те, чоканчићем ћеш ме!!")
        );
        assert_eq!("Njegoš", cyr_to_lat("Његош"));
        assert_eq!("škafiškafnjak", cyr_to_lat("шкафишкафњак"));

        // Провера конверзије двословних секвенци у контексту различите величине слова
        assert_eq!("Džak Ljubavi", cyr_to_lat("Џак Љубави"));
        assert_eq!("Džak LJUBAVI", cyr_to_lat("Џак ЉУБАВИ"));

        // Провера конверзије осталих карактера (без конверзије)
        assert_eq!("1 2 3 čokolada", cyr_to_lat("1 2 3 чоколада"));
    }

    #[test]
    fn test_lat_to_cir() {
        assert_eq!(
            "Чича Ђура жваће шљиве, његова ћерка Љиља једе џем",
            lat_to_cir("Čiča Đura žvaće šljive, njegova ćerka Ljilja jede džem")
        );
        assert_eq!(
            "Чоканчићем ћу те, чоканчићем ћеш ме!!",
            lat_to_cir("Čokančićem ću te, čokančićem ćeš me!!")
        );
        assert_eq!("Његош", lat_to_cir("Njegoš"));
        assert_eq!("шкафишкафњак", lat_to_cir("škafiškafnjak"));

        // Провера конверзије двословних секвенци у контексту различите величине слова
        assert_eq!("Џак Љубави", lat_to_cir("Džak Ljubavi"));
        assert_eq!("Џак ЉУБАВИ", lat_to_cir("Džak LJUBAVI"));

        // Провера конверзије осталих карактера (без конверзије)
        assert_eq!("1 2 3 чоколада", lat_to_cir("1 2 3 čokolada"));

        // Тестирање изузетака
        assert_eq!("Како Танјуг јавља, ја те волим!", lat_to_cir("Kako Tanjug javlja, ja te volim!"));
        assert_eq!("Оджубори овај поточић!", lat_to_cir("Odžubori ovaj potočić!"));
    }
}
