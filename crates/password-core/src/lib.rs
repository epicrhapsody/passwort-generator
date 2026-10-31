use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Options {
    pub length: usize,
    pub lowercase: bool,
    pub uppercase: bool,
    pub digits: bool,
    pub symbols: bool,
    pub exclude_similar: bool,
}

fn index(size: usize) -> Result<usize, String> {
    let limit = 256 - (256 % size);
    loop {
        let mut byte = [0u8];
        getrandom::getrandom(&mut byte)
            .map_err(|_| "Sicherer Systemzufall ist nicht verfügbar.".to_string())?;
        if (byte[0] as usize) < limit {
            return Ok(byte[0] as usize % size);
        }
    }
}

/// Uniform sampling with rejection of passwords missing an enabled character class.
/// No password is logged or persisted. RNG failure never falls back to weak randomness.
pub fn generate(options: &Options) -> Result<String, String> {
    if !(8..=128).contains(&options.length) {
        return Err("Die Länge muss zwischen 8 und 128 liegen.".into());
    }
    let groups: Vec<Vec<u8>> = [
        (options.lowercase, "abcdefghijklmnopqrstuvwxyz"),
        (options.uppercase, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        (options.digits, "0123456789"),
        (options.symbols, "!@#$%^&*()-_=+[]{};:,.?/~|"),
    ]
    .into_iter()
    .filter(|(enabled, _)| *enabled)
    .map(|(_, chars)| {
        chars
            .bytes()
            .filter(|c| !options.exclude_similar || !b"0Oo1Il|".contains(c))
            .collect()
    })
    .collect();
    if groups.is_empty() {
        return Err("Bitte mindestens eine Zeichenart auswählen.".into());
    }
    let alphabet: Vec<u8> = groups.iter().flatten().copied().collect();
    loop {
        let bytes: Vec<u8> = (0..options.length)
            .map(|_| index(alphabet.len()).map(|i| alphabet[i]))
            .collect::<Result<_, _>>()?;
        if groups
            .iter()
            .all(|group| bytes.iter().any(|c| group.contains(c)))
        {
            return String::from_utf8(bytes).map_err(|_| "Ungültiger Zeichensatz.".into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn options() -> Options {
        Options {
            length: 20,
            lowercase: true,
            uppercase: true,
            digits: true,
            symbols: true,
            exclude_similar: false,
        }
    }
    #[test]
    fn validates_bounds_and_empty_selection() {
        for length in [0, 7, 129, usize::MAX] {
            assert!(generate(&Options {
                length,
                ..options()
            })
            .is_err());
        }
        assert!(generate(&Options {
            lowercase: false,
            uppercase: false,
            digits: false,
            symbols: false,
            ..options()
        })
        .is_err());
    }
    #[test]
    fn all_group_combinations_and_boundaries() {
        for mask in 1..16 {
            for length in [8, 20, 128] {
                for exclude_similar in [false, true] {
                    let o = Options {
                        length,
                        lowercase: mask & 1 != 0,
                        uppercase: mask & 2 != 0,
                        digits: mask & 4 != 0,
                        symbols: mask & 8 != 0,
                        exclude_similar,
                    };
                    let password = generate(&o).unwrap();
                    assert_eq!(password.len(), length);
                    assert_eq!(
                        password.bytes().any(|c| c.is_ascii_lowercase()),
                        o.lowercase
                    );
                    assert_eq!(
                        password.bytes().any(|c| c.is_ascii_uppercase()),
                        o.uppercase
                    );
                    assert_eq!(password.bytes().any(|c| c.is_ascii_digit()), o.digits);
                    assert_eq!(
                        password.bytes().any(|c| c.is_ascii_punctuation()),
                        o.symbols
                    );
                    if exclude_similar {
                        assert!(!password.bytes().any(|c| b"0Oo1Il|".contains(&c)));
                    }
                }
            }
        }
    }
}
