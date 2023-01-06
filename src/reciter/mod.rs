use std::collections::HashMap;
use once_cell::sync::Lazy;

mod rules;

#[derive(Debug)]
pub enum ReciterError {
    BadPunctuation,
    MissingOpenParenthesis,
    MissingCloseParenthesis,
    NoRulesForCharacter(char),
    NoMatchingRuleFoundAtIndex(usize),
    NoMatchingCharacterRuleFoundAtIndex(usize)
}

impl std::error::Error for ReciterError {}

impl std::fmt::Display for ReciterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReciterError::BadPunctuation => write!(f, "Bad punctuation"),
            ReciterError::MissingOpenParenthesis => write!(f, "Missing open parenthesis"),
            ReciterError::MissingCloseParenthesis => write!(f, "Missing close parenthesis"),
            ReciterError::NoRulesForCharacter(character) => write!(f, "No rules found for character {:?}", character),
            ReciterError::NoMatchingRuleFoundAtIndex(index) => write!(f, "No matching rule found at index {}", index),
            ReciterError::NoMatchingCharacterRuleFoundAtIndex(index) => write!(f, "No matching character rule found at index {}", index)
        }
    }
}

type CharacterFlag = u8;

mod flag {
    use super::CharacterFlag;

    pub const NUMERIC: CharacterFlag        = 0x01; // numeric
    pub const RULESET_2: CharacterFlag      = 0x02; // use rule set 2
    pub const VOICED: CharacterFlag         = 0x04; // D J L N R S T Z FIXME: is this correct?
    pub const OXO8: CharacterFlag           = 0x08; // B D G J L M N R V W Z unknown
    pub const DIPTHONG: CharacterFlag       = 0x10; // C G J S X Z FIXME: is this correct?
    pub const CONSONANT: CharacterFlag      = 0x20; // B C D F G H J K L M N P Q R S T V W X Y Z ` FIXME: is this correct?
    pub const VOWEL_OR_Y: CharacterFlag     = 0x40; // Is a vowel or Y
    pub const ALPHA_OR_QUOTE: CharacterFlag = 0x80; // Is alpha or '
}

// charFlags
fn flags_for_character(character: char) -> CharacterFlag {
    match character {
        '!' => 0x02,
        '"' => 0x02,
        '#' => 0x02,
        '$' => 0x02,
        '%' => 0x02,
        '&' => 0x02,
        '\''=> 0x82,
        '*' => 0x02,
        '+' => 0x02,
        ',' => 0x02,
        '-' => 0x02,
        '.' => 0x02,
        '/' => 0x02,
        '0' => 0x03,
        '1' => 0x03,
        '2' => 0x03,
        '3' => 0x03,
        '4' => 0x03,
        '5' => 0x03,
        '6' => 0x03,
        '7' => 0x03,
        '8' => 0x03,
        '9' => 0x03,
        ':' => 0x02,
        ';' => 0x02,
        '<' => 0x02,
        '=' => 0x02,
        '>' => 0x02,
        '?' => 0x02,
        '@' => 0x02,
        'A' => 0xc0,
        'B' => 0xa8,
        'C' => 0xb0,
        'D' => 0xac,
        'E' => 0xc0,
        'F' => 0xa0,
        'G' => 0xb8,
        'H' => 0xa0,
        'I' => 0xc0,
        'J' => 0xbc,
        'K' => 0xa0,
        'L' => 0xac,
        'M' => 0xa8,
        'N' => 0xac,
        'O' => 0xc0,
        'P' => 0xa0,
        'Q' => 0xa0,
        'R' => 0xac,
        'S' => 0xb4,
        'T' => 0xa4,
        'U' => 0xc0,
        'V' => 0xa8,
        'W' => 0xa8,
        'X' => 0xb0,
        'Y' => 0xc0,
        'Z' => 0xbc,
        '^' => 0x02,
        '`' => 0x20,
        _   => 0x00
    }
}

static CHARACTER_RULES: Lazy<Vec<ReciterRule>> = Lazy::new(||
    rules::CHARACTER_RULES.iter().map(|(pattern, replacement)|
        ReciterRule::new(pattern, replacement).unwrap_or_else(|err| {
            panic!("Could not instantiate reciter rule for {:?} -> {:?} ({:?})", pattern, replacement, err)
        })
    ).collect()
);

static RULES: Lazy<HashMap<char, Vec<ReciterRule>>> = Lazy::new(|| {
    let mut rules_per_character: HashMap<char, Vec<ReciterRule>> = HashMap::new();

    for (pattern, replacement) in rules::RULES {
        let rule = ReciterRule::new(pattern, replacement).unwrap_or_else(|err| {
            panic!("Could not instantiate reciter rule for {:?} -> {:?} ({:?})", pattern, replacement, err)
        });

        rules_per_character.entry(rule.source[0]).or_default().push(rule);
    }

    rules_per_character
});

struct ReciterRule<'a> {
    prefix: Vec<char>,
    source: Vec<char>,
    suffix: Vec<char>,
    target: &'a str
}

impl<'a> ReciterRule<'a> {
    fn new(pattern: &'a str, replacement: &'a str) -> Result<Self, ReciterError> {
        let (prefix, rest) = pattern.split_once('(').ok_or(ReciterError::MissingOpenParenthesis)?;
        let (source, suffix) = rest.split_once(')').ok_or(ReciterError::MissingCloseParenthesis)?;

        Ok(Self {
            prefix: prefix.chars().collect(),
            source: source.chars().collect(),
            suffix: suffix.chars().collect(),
            target: replacement
        })
    }

    fn check_prefix(&self, text: &[char], position: usize) -> bool {
        let mut position = position;

        for rule_character in self.prefix.iter().rev() {
            if has_flags(*rule_character, flag::ALPHA_OR_QUOTE) {
                if position == 0 || text[position - 1] != *rule_character {
                    // Rule char does not match.
                    return false;
                }

                position -= 1;
                continue;
            }

            match rule_character {
                // '' - previous char must not be alpha or quotation mark.
                ' ' => {
                    if position >= 1 && !has_flags_at(text, position - 1, flag::ALPHA_OR_QUOTE) {
                        position -= 1;
                    } else {
                        return false;
                    }
                },

                // '#' - previous char must be a vowel or Y.
                '#' => {
                    if position >= 1 && has_flags_at(text, position - 1, flag::VOWEL_OR_Y) {
                        position -= 1;
                    } else {
                        return false;
                    }
                },

                // '.' - unknown?
                '.' => {
                    // Contrary to the use of 0x08 in the suffix, this one is actually used in
                    // the test set.
                    if position >= 1 && has_flags_at(text, position - 1, flag::OXO8) {
                        position -= 1;
                    } else {
                        return false;
                    }
                },

                // '&' - previous char must be a dipthong or previous chars must be 'CH' or 'SH'
                #[allow(clippy::if_same_then_else)]
                '&' => {
                    if position >= 1 && has_flags_at(text, position - 1, flag::DIPTHONG) {
                        // Dipthong
                        position -= 1;
                    } else if position >= 2 && text[position - 2] == 'C' && text[position - 1] == 'H' {
                        // CH
                        position -= 2;
                    } else if position >= 2 && text[position - 2] == 'S' && text[position - 1] == 'H' {
                        // SH
                        position -= 2;
                    } else {
                        return false;
                    }
                },

                // '@' - previous char must be voiced and not 'H'.
                '@' => {
                    if position >= 1 && has_flags_at(text, position - 1, flag::VOICED) {
                        // Voiced
                        position -= 1;
                    } else {
                        // Note: logic errors in the original implementation make the "H" check
                        // irrelevant. There is also a check for "C", "S", or "T", that is
                        // never true.
                        return false;
                    }
                },

                // '^' - previous char must be a consonant.
                '^' => {
                    if position >= 1 && has_flags_at(text, position - 1, flag::CONSONANT) {
                        position -= 1;
                    } else {
                        return false;
                    }
                },

                // '+' - previous char must be either 'E', 'I' or 'Y'.
                '+' => {
                    if position >= 1 && (text[position - 1] == 'E' || text[position - 1] == 'I' || text[position - 1] == 'Y') {
                        position -= 1;
                    } else {
                        return false;
                    }
                },

                // ':' - walk left in input position until we hit a non consonant or begin of string.
                ':' => {
                    while position >= 1 && has_flags_at(text, position - 1, flag::CONSONANT) {
                        position -= 1;
                    }
                },

                _ => panic!("Unrecognized rule prefix character {:?}", rule_character)
            };
        }

        true
    }

    fn check_suffix(&self, text: &[char], position: usize) -> bool {
        let mut position = position;

        for rule_character in &self.suffix {
            // do we have to handle the byte specially?
            if has_flags(*rule_character, flag::ALPHA_OR_QUOTE) {
                if position + 1 >= text.len() || text[position + 1] != *rule_character {
                    return false;
                }

                position += 1;
                continue;
            }

            // pos37226:
            match rule_character {
                // ' ' - next char must not be alpha or quotation mark.
                ' ' => {
                    if position + 1 < text.len() && !has_flags_at(text, position + 1, flag::ALPHA_OR_QUOTE) {
                        position += 1;
                    } else {
                        return false;
                    }
                },

                // '#' - next char must be a vowel or Y.
                '#' => {
                    if position + 1 < text.len() && has_flags_at(text, position + 1, flag::VOWEL_OR_Y) {
                        position += 1;
                    } else {
                        return false;
                    }
                },

                // '.' - unknown?
                '.' => {
                    // Note: this code is never triggered in the test suite. Does it have a
                    // function?
                    if position + 1 < text.len() && has_flags_at(text, position, flag::OXO8) {
                        position += 1;
                    } else {
                        return false;
                    }
                },

                // '&' - next char must be a dipthong or next chars must be 'HC' or 'HS'
                #[allow(clippy::if_same_then_else)]
                '&' => {
                    if position + 1 < text.len() && has_flags_at(text, position + 1, flag::DIPTHONG) {
                        // Character is dipthong
                        position += 1;
                    } else if position + 2 < text.len() && text[position + 1] == 'H' && text[position + 2] == 'C' {
                        // HC
                        position += 2;
                    } else if position + 2 < text.len() && text[position + 1] == 'H' && text[position + 2] == 'S' {
                        // HS
                        position += 2;
                    } else {
                        return false;
                    }
                },

                // '@' - next char must be voiced and not 'H'.
                #[allow(clippy::if_same_then_else)]
                '@' => {
                    // Note, in the original source, the comment for this character says
                    // "voiced and not H", but the original code implements "voiced or H".
                    if position + 1 < text.len() && has_flags_at(text, position + 1, flag::VOICED) {
                        // Voiced character
                        position += 1;
                    } else if position + 1 < text.len() && text[position + 1] == 'H' {
                        // H
                        position += 1;
                    } else if position + 1 < text.len() && (text[position + 1] == 'C' || text[position + 1] == 'S' || text[position + 1] == 'T') {
                        // C, S, or T
                        // FIXME: This is illogical and can never be reached. Bug in original code? reciter.c:489 (pos37367)
                        unreachable!("Should not be reachable for input character {:?}, bug in original code?", text[position + 1]);
                    } else {
                        return false;
                    }
                },

                // '^' - next char must be a consonant.
                '^' => {
                    if position + 1 < text.len() && has_flags_at(text, position + 1, flag::CONSONANT) {
                        position += 1;
                    } else {
                        return false;
                    }
                },

                // '+' - next char must be either 'E', 'I' or 'Y'.
                '+' => {
                    if position + 1 < text.len() && (text[position + 1] == 'E' || text[position + 1] == 'I' || text[position + 1] == 'Y') {
                        position += 1;
                    } else {
                        return false;
                    }
                },

                // ':' - walk right in input position until we hit a non consonant.
                ':' => {
                    while position + 1 < text.len() && has_flags_at(text, position + 1, flag::CONSONANT) {
                        position += 1;
                    }
                },

                /* '%' - check if we have:
                   - 'ING'
                   - 'E' not followed by alpha or quot
                   - 'ER' 'ES' or 'ED'
                   - 'EFUL'
                   - 'ELY'
                   */
                '%' => {
                    if position + 3 < text.len() && text[position + 1] == 'I' && text[position + 2] == 'N' && text[position + 3] == 'G' {
                        // ING
                        position += 3;
                    } else if position + 1 < text.len() && text[position + 1] == 'E' && (position + 2 >= text.len() || !has_flags_at(text, position + 2, flag::ALPHA_OR_QUOTE)) {
                        // E not followed by alpha or quote
                        position += 1;
                    } else if position + 2 < text.len() && text[position + 1] == 'E' && (text[position + 2] == 'R' || text[position + 2] == 'S' || text[position + 2] == 'D') {
                        // ER, ES, or ED
                        position += 2;
                    } else if position + 3 < text.len() && text[position + 1] == 'E' && text[position + 2] == 'L' && text[position + 3] == 'Y' {
                        // ELY
                        position += 3;
                    } else if position + 4 < text.len() && text[position + 1] == 'E' && text[position + 2] == 'F' && text[position + 3] == 'U' && text[position + 4] == 'L' {
                        // EFUL
                        position += 4;
                    } else {
                        return false;
                    }
                },

                _ => panic!("Unrecognized rule suffix character {:?}", rule_character)
            };
        }

        true
    }

    fn matches(&self, text: &[char], position: usize) -> bool {
        // Check if the source matches
        if !text[position..].starts_with(&self.source) {
            return false;
        }

        // Check if the prefix matches
        if !self.check_prefix(text, position) {
            return false;
        }

        // Check if the suffix matches
        if !self.check_suffix(text, position + (self.source.len() - 1)) {
            return false;
        }

        true
    }
}

fn has_flags(character: char, flag: CharacterFlag) -> bool {
    flags_for_character(character) & flag != 0
}

fn has_flags_at(text: &[char], position: usize, flag: CharacterFlag) -> bool {
    flags_for_character(text[position]) & flag != 0
}

/// Convert the input text to a representation using phonemes.
pub fn text_to_phonemes(text: &str) -> Result<String, ReciterError> {
    let mut output = String::new();

    // Pad the input string with spaces so the ends have word boundaries
    let input: Vec<char> = std::iter::once(' ').chain(text.to_ascii_uppercase().chars()).chain(std::iter::once(' ')).collect();

    let mut index = 0;

    // Note: the original implementation bounds this by a maximum of 10000 iterations due to a lack
    // of error checking.
    while index < input.len() {
        let character = input[index];

        // Check for "." not followed by a number
        if character == '.' && (index + 1 >= input.len() || !has_flags_at(&input, index + 1, flag::NUMERIC)) {
            output += ".";
            index += 1;
            continue;
        }

        // Replace characters without flags with spaces
        if flags_for_character(character) == 0 {
            output += " ";
            index += 1;
            continue;
        }

        // Apply character rules if the rule set 2 flag is set
        if has_flags(character, flag::RULESET_2) {
            if let Some(rule) = CHARACTER_RULES.iter().find(|rule| rule.matches(&input, index)) {
                index += rule.source.len();
                output += rule.target;
            } else {
                return Err(ReciterError::NoMatchingCharacterRuleFoundAtIndex(index));
            }

            continue;
        }

        // Non-alpha or quote characters should be covered by rule set 2
        if !has_flags(character, flag::ALPHA_OR_QUOTE) {
            return Err(ReciterError::BadPunctuation);
        }

        // Find and apply the first matching rule that has the character as its starting character
        if let Some(rules) = RULES.get(&character) {
            if let Some(rule) = rules.iter().find(|rule| rule.matches(&input, index)) {
                index += rule.source.len();
                output += rule.target;
            } else {
                return Err(ReciterError::NoMatchingRuleFoundAtIndex(index));
            }
        } else {
            return Err(ReciterError::NoRulesForCharacter(character));
        }
    }

    //Ok(output.trim().to_owned())
    Ok(output[..output.len() - 1].to_owned())
}

#[cfg(test)]
mod tests {
    use super::text_to_phonemes;

    use std::fs::File;
    use std::path::PathBuf;

    use serde::Deserialize;

    #[derive(Deserialize)]
    struct TestCase {
        text: String,
        phonemes: String
    }

    #[test]
    fn sanity() {
        assert_eq!(text_to_phonemes("").unwrap(), "");
    }

    #[test]
    fn from_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/reciter.json");

        println!("{}", path.display());

        let file = File::open(path).expect("Could not open reciter test file");
        let mmap = unsafe { memmap::Mmap::map(&file) }.expect("Could not mmap reciter test file");
        let contents = std::str::from_utf8(&mmap).expect("Could not read reciter test file");

        let testcases: Vec<TestCase> = serde_json::from_str(contents).expect("Could not deserialize reciter test file");

        let max_line_length: usize = 30;

        for (index, testcase) in testcases.iter().enumerate() {
            print!("\x1b[90m{:3} \x1b[94m{:max_line_length$}\x1b[90m -> \x1b[0m", index, testcase.text);

            let start = std::time::Instant::now();
            let result = text_to_phonemes(&testcase.text);
            let duration = start.elapsed();

            // Pretty printing
            match &result {
                Ok(phonemes) if *phonemes == testcase.phonemes => {
                    println!("\x1b[1;92mPASS\x1b[0;32m in \x1b[92m{:?}\x1b[0m", duration);
                },
                Ok(phonemes) => {
                    println!("\x1b[1;91mFAIL\x1b[0;31m (\x1b[91m{:?}\x1b[31m instead of \x1b[91m{:?}\x1b[31m) in \x1b[91m{:?}\x1b[0m", phonemes, testcase.phonemes, duration);
                    assert_eq!(*phonemes, testcase.phonemes);
                },
                Err(err) => {
                    println!("\x1b[1;91mFAIL\x1b[0;31m ({:?}) in \x1b[91m{:?}\x1b[0m", err, duration);
                }
            }

            assert!(result.is_ok());
        }
    }
}
