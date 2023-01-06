#[derive(Debug)]
pub enum ParseError {
    // TODO: Cases
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parse error")
    }
}

#[derive(Debug)]
pub struct ParseResult {
    pub phonemes: Vec<Phoneme>
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Phoneme {
    pub length: u8,
    pub index: usize,
    pub stress: u8
}

impl Phoneme {
    fn has_flag(&self, flag: u16) -> bool {
        PHONEME_FLAGS[self.index] & flag != 0
    }
}

impl ParseResult {
    fn new() -> Self {
        Self {
            phonemes: Vec::new()
        }
    }
}

const STRESS_TABLE: &[char] = &[
    '*', '1', '2', '3', '4', '5', '6', '7', '8'
];

const PHONEME_NAME_TABLE: &[(char, char)] = &[
    (' ', '*'), // 00
    ('.', '*'), // 01
    ('?', '*'), // 02
    (',', '*'), // 03
    ('-', '*'), // 04
    ('I', 'Y'), // 05
    ('I', 'H'), // 06
    ('E', 'H'), // 07
    ('A', 'E'), // 08
    ('A', 'A'), // 09
    ('A', 'H'), // 10
    ('A', 'O'), // 11
    ('U', 'H'), // 12
    ('A', 'X'), // 13
    ('I', 'X'), // 14
    ('E', 'R'), // 15
    ('U', 'X'), // 16
    ('O', 'H'), // 17
    ('R', 'X'), // 18
    ('L', 'X'), // 19
    ('W', 'X'), // 20
    ('Y', 'X'), // 21
    ('W', 'H'), // 22
    ('R', '*'), // 23
    ('L', '*'), // 24
    ('W', '*'), // 25
    ('Y', '*'), // 26
    ('M', '*'), // 27
    ('N', '*'), // 28
    ('N', 'X'), // 29
    ('D', 'X'), // 30
    ('Q', '*'), // 31
    ('S', '*'), // 32
    ('S', 'H'), // 33
    ('F', '*'), // 34
    ('T', 'H'), // 35
    ('/', 'H'), // 36
    ('/', 'X'), // 37
    ('Z', '*'), // 38
    ('Z', 'H'), // 39
    ('V', '*'), // 40
    ('D', 'H'), // 41
    ('C', 'H'), // 42
    ('*', '*'), // 43
    ('J', '*'), // 44
    ('*', '*'), // 45
    ('*', '*'), // 46
    ('*', '*'), // 47
    ('E', 'Y'), // 48
    ('A', 'Y'), // 49
    ('O', 'Y'), // 50
    ('A', 'W'), // 51
    ('O', 'W'), // 52
    ('U', 'W'), // 53
    ('B', '*'), // 54
    ('*', '*'), // 55
    ('*', '*'), // 56
    ('D', '*'), // 57
    ('*', '*'), // 58
    ('*', '*'), // 59
    ('G', '*'), // 60
    ('*', '*'), // 61
    ('*', '*'), // 62
    ('G', 'X'), // 63
    ('*', '*'), // 64
    ('*', '*'), // 65
    ('P', '*'), // 66
    ('*', '*'), // 67
    ('*', '*'), // 68
    ('T', '*'), // 69
    ('*', '*'), // 70
    ('*', '*'), // 71
    ('K', '*'), // 72
    ('*', '*'), // 73
    ('*', '*'), // 74
    ('K', 'X'), // 75
    ('*', '*'), // 76
    ('*', '*'), // 77
    ('U', 'L'), // 78
    ('U', 'M'), // 79
    ('U', 'N')  // 80
];

const PHONEME_LENGTH_TABLE: &[(u8, u8)] = &[
    (0x00, 0x00), // ' *' 00
    (0x12, 0x12), // '.*' 01
    (0x12, 0x12), // '?*' 02
    (0x12, 0x12), // ',*' 03
    (0x08, 0x08), // '-*' 04
    (0x08, 0x0B), // 'IY' 05
    (0x08, 0x09), // 'IH' 06
    (0x08, 0x0B), // 'EH' 07
    (0x08, 0x0E), // 'AE' 08
    (0x0B, 0x0F), // 'AA' 09
    (0x06, 0x0B), // 'AH' 10
    (0x0C, 0x10), // 'AO' 11
    (0x0A, 0x0C), // 'UH' 12
    (0x05, 0x06), // 'AX' 13
    (0x05, 0x06), // 'IX' 14
    (0x0B, 0x0E), // 'ER' 15
    (0x0A, 0x0C), // 'UX' 16
    (0x0A, 0x0E), // 'OH' 17
    (0x0A, 0x0C), // 'RX' 18
    (0x09, 0x0B), // 'LX' 19
    (0x08, 0x08), // 'WX' 20
    (0x07, 0x08), // 'YX' 21
    (0x09, 0x0B), // 'WH' 22
    (0x07, 0x0A), // 'R*' 23
    (0x06, 0x09), // 'L*' 24
    (0x08, 0x08), // 'W*' 25
    (0x06, 0x08), // 'Y*' 26
    (0x07, 0x08), // 'M*' 27
    (0x07, 0x08), // 'N*' 28
    (0x07, 0x08), // 'NX' 29
    (0x02, 0x03), // 'DX' 30
    (0x05, 0x05), // 'Q*' 31
    (0x02, 0x02), // 'S*' 32
    (0x02, 0x02), // 'SH' 33
    (0x02, 0x02), // 'F*' 34
    (0x02, 0x02), // 'TH' 35
    (0x02, 0x02), // '/H' 36
    (0x02, 0x02), // '/X' 37
    (0x06, 0x06), // 'Z*' 38
    (0x06, 0x06), // 'ZH' 39
    (0x07, 0x08), // 'V*' 40
    (0x06, 0x06), // 'DH' 41
    (0x06, 0x06), // 'CH' 42
    (0x02, 0x02), // '**' 43
    (0x08, 0x09), // 'J*' 44
    (0x03, 0x04), // '**' 45
    (0x01, 0x02), // '**' 46
    (0x1E, 0x01), // '**' 47
    (0x0D, 0x0E), // 'EY' 48
    (0x0C, 0x0F), // 'AY' 49
    (0x0C, 0x0F), // 'OY' 50
    (0x0C, 0x0F), // 'AW' 51
    (0x0E, 0x0E), // 'OW' 52
    (0x09, 0x0E), // 'UW' 53
    (0x06, 0x08), // 'B*' 54
    (0x01, 0x02), // '**' 55
    (0x02, 0x02), // '**' 56
    (0x05, 0x07), // 'D*' 57
    (0x01, 0x02), // '**' 58
    (0x01, 0x01), // '**' 59
    (0x06, 0x07), // 'G*' 60
    (0x01, 0x02), // '**' 61
    (0x02, 0x02), // '**' 62
    (0x06, 0x07), // 'GX' 63
    (0x01, 0x02), // '**' 64
    (0x02, 0x02), // '**' 65
    (0x08, 0x08), // 'P*' 66
    (0x02, 0x02), // '**' 67
    (0x02, 0x02), // '**' 68
    (0x04, 0x06), // 'T*' 69
    (0x02, 0x02), // '**' 70
    (0x02, 0x02), // '**' 71
    (0x06, 0x07), // 'K*' 72
    (0x01, 0x02), // '**' 73
    (0x04, 0x04), // '**' 74
    (0x06, 0x07), // 'KX' 75
    (0x01, 0x01), // '**' 76
    (0x04, 0x04), // '**' 77
    (0xC7, 0x05), // 'UL' 78
    (0xFF, 0x05)  // 'UM' 79
    // FIXME: Phoneme 80 (UN) is missing
];

mod flag {
    // Unused constants
    pub const _OX8000: u16          = 0x8000; // Unknown: ' *', '.*', '?*', ',*', '-*'
    pub const _OX4000: u16          = 0x4000; // Unknown: '.*', '?*', ',*', '-*', 'Q*'

    // Consonant articulations
    pub const FRICATIVE: u16        = 0x2000;
    pub const LIQUID: u16           = 0x1000;
    pub const NASAL: u16            = 0x0800;
    pub const ALVEOLAR: u16         = 0x0400;

    // 0x0200 is unused
    pub const _OX0200: u16          = 0x0200;

    pub const PUNCTUATION: u16      = 0x0100;
    pub const VOWEL: u16            = 0x0080;
    pub const CONSONANT: u16        = 0x0040; // Note that UM and UN are marked as both vowels and consonants

    pub const DIPHTHONG_YX: u16     = 0x0020; // Diphthong ending with YX, front vowels?
    pub const DIPHTHONG: u16        = 0x0010;

    // Unknown:
    // 'M*', 'N*', 'NX', 'DX', 'Q*', 'CH', 'J*', 'B*', '**', '**', 'D*',
    // '**', '**', 'G*', '**', '**', 'GX', '**', '**', 'P*', '**', '**',
    // 'T*', '**', '**', 'K*', '**', '**', 'KX', '**', '**'
    pub const OX0008: u16           = 0x0008;

    pub const VOICED: u16           = 0x0004; // Applied to vowels and consonants

    // Plosives
    pub const PLOSIVE: u16          = 0x0002; // Both voiced and unvoiced
    pub const UNVOICED_PLOSIVE: u16 = 0x0001;
}

const PHONEME_FLAGS: &[u16] = &[
    0x8000, // ' *' 00
    0xc100, // '.*' 01
    0xc100, // '?*' 02
    0xc100, // ',*' 03
    0xc100, // '-*' 04
    0x00a4, // 'IY' 05
    0x00a4, // 'IH' 06
    0x00a4, // 'EH' 07
    0x00a4, // 'AE' 08
    0x00a4, // 'AA' 09
    0x00a4, // 'AH' 10
    0x0084, // 'AO' 11
    0x0084, // 'UH' 12
    0x00a4, // 'AX' 13
    0x00a4, // 'IX' 14
    0x0084, // 'ER' 15
    0x0084, // 'UX' 16
    0x0084, // 'OH' 17
    0x0084, // 'RX' 18
    0x0084, // 'LX' 19
    0x0084, // 'WX' 20
    0x0084, // 'YX' 21
    0x0044, // 'WH' 22
    0x1044, // 'R*' 23
    0x1044, // 'L*' 24
    0x1044, // 'W*' 25
    0x1044, // 'Y*' 26
    0x084c, // 'M*' 27
    0x0c4c, // 'N*' 28
    0x084c, // 'NX' 29
    0x0448, // 'DX' 30
    0x404c, // 'Q*' 31
    0x2440, // 'S*' 32
    0x2040, // 'SH' 33
    0x2040, // 'F*' 34
    0x2440, // 'TH' 35
    0x0040, // '/H' 36
    0x0040, // '/X' 37
    0x2444, // 'Z*' 38
    0x2044, // 'ZH' 39
    0x2044, // 'V*' 40
    0x2444, // 'DH' 41
    0x2048, // 'CH' 42
    0x2040, // '**' 43
    0x004c, // 'J*' 44
    0x2044, // '**' 45
    0x0000, // '**' 46
    0x0000, // '**' 47
    0x00b4, // 'EY' 48
    0x00b4, // 'AY' 49
    0x00b4, // 'OY' 50
    0x0094, // 'AW' 51
    0x0094, // 'OW' 52
    0x0094, // 'UW' 53
    0x004e, // 'B*' 54
    0x004e, // '**' 55
    0x004e, // '**' 56
    0x044e, // 'D*' 57
    0x044e, // '**' 58
    0x044e, // '**' 59
    0x004e, // 'G*' 60
    0x004e, // '**' 61
    0x004e, // '**' 62
    0x004e, // 'GX' 63
    0x004e, // '**' 64
    0x004e, // '**' 65
    0x004b, // 'P*' 66
    0x004b, // '**' 67
    0x004b, // '**' 68
    0x044b, // 'T*' 69
    0x044b, // '**' 70
    0x044b, // '**' 71
    0x004b, // 'K*' 72
    0x004b, // '**' 73
    0x004b, // '**' 74
    0x004b, // 'KX' 75
    0x004b, // '**' 76
    0x004b, // '**' 77
    0x0080, // 'UL' 78
    0x00c1, // 'UM' 79
    0x00c1  // 'UN' 80
];

/// Match both characters, but not with wildcards.
fn full_match(sign1: char, sign2: char) -> Option<usize> {
    // TODO: Investigate if sign2 is ever an asterisk
    PHONEME_NAME_TABLE.iter().position(|(first, second)|
        *second != '*' && *first == sign1 && *second == sign2
    )
}

/// Match character plus a wildcard.
fn wildcard_match(sign1: char) -> Option<usize> {
    PHONEME_NAME_TABLE.iter().position(|(first, second)|
        *first == sign1 && *second == '*'
    )
}

// TODO: Emit Result instead of panicking
fn parser1(text: &str) -> ParseResult {
    let mut result = ParseResult::new();
    let mut iter = text.chars().peekable();

    while let Some(sign1) = iter.next() {
        if let Some(sign2) = iter.peek() {
            if let Some(phoneme_index) = full_match(sign1, *sign2) {
                // Matched both characters (no wildcards)

                // Skip the second character of the input as we've matched it
                iter.next();

                // add_phoneme
                result.phonemes.push(Phoneme {
                    index: phoneme_index,
                    length: 0,
                    stress: 0
                });

                continue;
            }
        }

        if let Some(phoneme_index) = wildcard_match(sign1) {
            // Matched just the first character (with second character matching '*'

            // add_phoneme
            result.phonemes.push(Phoneme {
                index: phoneme_index,
                length: 0,
                stress: 0
            });

            continue;
        }

        // Note: the first index ("*") is not matched in the original implementation. The original
        // implementation searches backwards, but this does not make sense on a modern CPU.
        // TODO: Can be replaced with ascii math instead of iteration?
        if let Some(index) = STRESS_TABLE[1..].iter().position(|candidate| *candidate == sign1) {
            // add_stress
            // Compensate for the skipped "*" in the iterator index
            let index = index + 1;

            // FIXME: This can never happen here?
            //if index & 128 != 0 {
                //throw new Error('Got the flag 0x80, see CopyStress() and SetPhonemeLength() comments!');
            //}

            // Set stress for prior phoneme
            result.phonemes.last_mut().expect("Tried adding stress without adding a phoneme first").stress = index as u8;
        } else {
            panic!("Could not parse character {:?}", sign1);
        }
    }

    result
}

pub const PHONEME_PAUSE: usize         = 0;
pub const PHONEME_PERIOD: usize        = 1;
pub const PHONEME_QUESTION_MARK: usize = 2;
pub const PHONEME_AX: usize            = 13;
pub const PHONEME_UX: usize            = 16;
pub const PHONEME_RX: usize            = 18;
pub const PHONEME_LX: usize            = 19;
pub const PHONEME_WX: usize            = 20;
pub const PHONEME_YX: usize            = 21;
pub const PHONEME_R_STAR: usize        = 23;
pub const PHONEME_L_STAR: usize        = 24;
pub const PHONEME_M_STAR: usize        = 27;
pub const PHONEME_N_STAR: usize        = 28;
pub const PHONEME_DX: usize            = 30;
pub const PHONEME_Q_STAR: usize        = 31;
pub const PHONEME_S_STAR: usize        = 32;
pub const PHONEME_SLASH_H: usize       = 36;
pub const PHONEME_SLASH_X: usize       = 37;
pub const PHONEME_Z_STAR: usize        = 38;
pub const PHONEME_CH: usize            = 42;
pub const PHONEME_STAR_STAR_43: usize  = 43;
pub const PHONEME_J_STAR: usize        = 44;
pub const PHONEME_STAR_STAR_45: usize  = 45;
pub const PHONEME_UW: usize            = 53;
pub const PHONEME_D_STAR: usize        = 57;
pub const PHONEME_G_STAR: usize        = 60;
pub const PHONEME_GX: usize            = 63;
pub const PHONEME_T_STAR: usize        = 69;
pub const PHONEME_K_STAR: usize        = 72;
pub const PHONEME_KX: usize            = 75;
pub const PHONEME_UL: usize            = 78;
pub const PHONEME_UM: usize            = 79;
pub const PHONEME_UN: usize            = 80;

fn handle_uw_ch_j(phonemes: &mut Vec<Phoneme>, position: usize) {
    let phoneme = &phonemes[position];

    match phoneme.index {
        // 'UW' Example: NEW, DEW, SUE, ZOO, THOO, TOO
        // Check for UW with alveolar flag set on previous phoneme
        PHONEME_UW if phonemes.get(position - 1).unwrap().has_flag(flag::ALVEOLAR) => {
            phonemes[position].index = PHONEME_UX;
        },

        // 'CH' Example: CHEW
        PHONEME_CH => {
            phonemes.insert(position + 1, Phoneme {
                length: 0,
                index: PHONEME_STAR_STAR_43,
                stress: phoneme.stress
            });
        },

        // 'J*' Example: JAY
        PHONEME_J_STAR => {
            phonemes.insert(position + 1, Phoneme {
                length: 0,
                index: PHONEME_STAR_STAR_45,
                stress: phoneme.stress
            });
        },

        _ => ()
    }
}

fn parser2(result: &mut ParseResult) -> Result<(), ParseError> {
    let mut position: isize = -1;

    loop {
        position += 1;
        let position = position as usize;

        if position >= result.phonemes.len() {
            break;
        }

        // Is phoneme pause?
        if result.phonemes[position].index == PHONEME_PAUSE {
            continue;
        }

        if result.phonemes[position].has_flag(flag::DIPHTHONG) {
            // <DIPHTHONG ENDING WITH WX> -> <DIPHTHONG ENDING WITH WX> WX
            // <DIPHTHONG NOT ENDING WITH WX> -> <DIPHTHONG NOT ENDING WITH WX> YX
            // Example: OIL, COW
            // If ends with IY, use YX, else use WX
            // Insert at WX or YX following, copying the stress
            // 'WX' = 20 'YX' = 21
            result.phonemes.insert(position + 1, Phoneme {
                length: 0,
                index: if result.phonemes[position].has_flag(flag::DIPHTHONG_YX) {
                    PHONEME_YX
                } else {
                    PHONEME_WX
                },
                stress: result.phonemes[position].stress
            });

            handle_uw_ch_j(&mut result.phonemes, position);
            continue;
        }

        if result.phonemes[position].index == PHONEME_UL {
            // 'UL' => 'AX' 'L*'
            // Example: MEDDLE
            result.phonemes[position].index = PHONEME_AX;
            result.phonemes.insert(position + 1, Phoneme {
                length: 0,
                index: PHONEME_L_STAR,
                stress: result.phonemes[position].stress
            });

            continue;
        }

        if result.phonemes[position].index == PHONEME_UM {
            // 'UM' => 'AX' 'M*'
            // Example: ASTRONOMY
            result.phonemes[position].index = PHONEME_AX;
            result.phonemes.insert(position + 1, Phoneme {
                length: 0,
                index: PHONEME_M_STAR,
                stress: result.phonemes[position].stress
            });

            continue;
        }

        if result.phonemes[position].index == PHONEME_UN {
            // 'UN' => 'AX' 'N*'
            result.phonemes[position].index = PHONEME_AX;
            result.phonemes.insert(position + 1, Phoneme {
                length: 0,
                index: PHONEME_N_STAR,
                stress: result.phonemes[position].stress
            });

            continue;
        }

        if result.phonemes[position].has_flag(flag::VOWEL) && result.phonemes[position].stress != 0 {
            // Example: FUNCTION
            // RULE:
            //       <STRESSED VOWEL> <SILENCE> <STRESSED VOWEL> -> <STRESSED VOWEL> <SILENCE> Q <VOWEL>
            // EXAMPLE: AWAY EIGHT
            if result.phonemes.get(position + 1).map_or(false, |phoneme| phoneme.index == PHONEME_PAUSE) { // If following phoneme is a pause, get next
                if let Some(phoneme) = result.phonemes.get(position + 2) {
                    if phoneme.has_flag(flag::VOWEL) && phoneme.stress != 0 {
                        // Insert glottal stop between two stressed vowels with space between them
                        result.phonemes.insert(position + 2, Phoneme {
                            length: 0,
                            index: PHONEME_Q_STAR,
                            stress: 0
                        });
                    }
                }
            }

            continue;
        }

        ////let priorPhoneme = (pos === 0) ? null : getPhoneme(pos - 1);
        let prior_phoneme = if position > 0 {
            result.phonemes.get(position - 1)
        } else {
            None
        };

        if result.phonemes[position].index == PHONEME_R_STAR {
            if let Some(prior_phoneme) = prior_phoneme {
                // position - 1 is guaranteed to be valid inside this block
                // Rules for phonemes before R
                match prior_phoneme.index {
                    // Example: TRACK
                    // T* R* -> CH R*
                    PHONEME_T_STAR => {
                        result.phonemes[position - 1].index = PHONEME_CH;
                    },

                    // Example: DRY
                    // D* R* -> J* R*
                    PHONEME_D_STAR => {
                        result.phonemes[position - 1].index = PHONEME_J_STAR;
                    }

                    // Example: ART
                    // <VOWEL> R* -> <VOWEL> RX
                    _ => if prior_phoneme.has_flag(flag::VOWEL) {
                        result.phonemes[position].index = PHONEME_RX;
                    }
                }
            }

            continue;
        }

        // 'L*'
        if result.phonemes[position].index == PHONEME_L_STAR && prior_phoneme.map_or(false, |phoneme| phoneme.has_flag(flag::VOWEL)) {
            // Example: ALL
            // <VOWEL> L* -> <VOWEL> LX
            result.phonemes[position].index = PHONEME_LX;
            continue;
        }

        // 'G*' 'S*'
        if result.phonemes[position].index == PHONEME_S_STAR && prior_phoneme.map_or(false, |phoneme| phoneme.index == PHONEME_G_STAR) {
            // G S -> G Z
            // Can't get to fire -
            //       1. The G -> GX rule intervenes
            //       2. Reciter already replaces GS -> GZ
            result.phonemes[position].index = PHONEME_Z_STAR;
            continue;
        }

        // 'G*'
        if result.phonemes[position].index == PHONEME_G_STAR {
            // G <VOWEL OR DIPHTHONG NOT ENDING WITH IY> -> GX <VOWEL OR DIPHTHONG NOT ENDING WITH IY>
            // Example: GO
            if let Some(phoneme) = result.phonemes.get(position + 1) {
                // If diphthong ending with YX, move continue processing next phoneme
                if !phoneme.has_flag(flag::DIPHTHONG_YX) {
                    // replace G with GX and continue processing next phoneme
                    // G <VOWEL OR DIPHTHONG NOT ENDING WITH IY> -> GX <VOWEL OR DIPHTHONG NOT ENDING WITH IY>
                    result.phonemes[position].index = PHONEME_GX;
                }
            }

            continue;
        }

        // 'K*'
        if result.phonemes[position].index == PHONEME_K_STAR {
            // K <VOWEL OR DIPHTHONG NOT ENDING WITH IY> -> KX <VOWEL OR DIPHTHONG NOT ENDING WITH IY>
            // Example: COW
            // If at end, replace current phoneme with KX
            // Note: also applies when next phoneme is not DIPHTHONG_YX
            if result.phonemes.get(position + 1).map_or(true, |phoneme| !phoneme.has_flag(flag::DIPHTHONG_YX)) {
                // VOWELS AND DIPHTHONGS ENDING WITH IY SOUND flag set?
                result.phonemes[position].index = PHONEME_KX;

                // TODO: Figure out what the impact of this change is and if it can ever match
                // any rules below
                // TODO: Can be removed safely after switching to using array indices?
                //phoneme = PHONEME_KX;
            }
        }

        // Replace with softer version?
        if result.phonemes[position].has_flag(flag::UNVOICED_PLOSIVE) && result.phonemes.get(position - 1).map_or(false, |phoneme| phoneme.index == PHONEME_S_STAR) {
            // 'S*'
            // RULE:
            //   'S*' 'P*' -> 'S*' 'B*'
            //   'S*' 'T*' -> 'S*' 'D*'
            //   'S*' 'K*' -> 'S*' 'G*'
            //   'S*' 'KX' -> 'S*' 'GX'
            //   'S*' 'UM' -> 'S*' '**'
            //   'S*' 'UN' -> 'S*' '**'
            // Examples: SPY, STY, SKY, SCOWL
            result.phonemes[position].index -= 12;
        } else if !result.phonemes[position].has_flag(flag::UNVOICED_PLOSIVE) {
            handle_uw_ch_j(&mut result.phonemes, position);
        }

        // 'T*', 'D*'
        if result.phonemes[position].index == PHONEME_T_STAR || result.phonemes[position].index == PHONEME_D_STAR {
            // RULE: Soften T following vowel
            // NOTE: This rule fails for cases such as "ODD"
            //       <UNSTRESSED VOWEL> T <PAUSE> -> <UNSTRESSED VOWEL> DX <PAUSE>
            //       <UNSTRESSED VOWEL> D <PAUSE>  -> <UNSTRESSED VOWEL> DX <PAUSE>
            // Example: PARTY, TARDY
            if let Some(prior_phoneme) = result.phonemes.get(position - 1) {
                if prior_phoneme.has_flag(flag::VOWEL) {
                    let mut phoneme = result.phonemes.get(position + 1);
                    let next_phoneme = phoneme;

                    if next_phoneme.is_some() && next_phoneme.unwrap().index == PHONEME_PAUSE {
                        phoneme = result.phonemes.get(position + 2);
                    }

                    if let Some(phoneme) = phoneme {
                        if phoneme.has_flag(flag::VOWEL) && next_phoneme.map_or(false, |phoneme| phoneme.stress == 0) {
                            // Soften T or D following vowel or ER and preceding a pause -> DX
                            result.phonemes[position].index = PHONEME_DX;
                        }
                    }
                }
            }

            continue;
        }
    }

    Ok(())
}

fn copy_stress(phonemes: &mut [Phoneme]) {
    let mut iter = phonemes.iter_mut().peekable();

    while let Some(phoneme) = iter.next() {
        // if CONSONANT_FLAG set, skip - only vowels get stress
        if !phoneme.has_flag(flag::CONSONANT) {
            continue;
        }

        if let Some(next_phoneme) = iter.peek() {
            // if the following phoneme is the end, or a vowel, skip
            if next_phoneme.has_flag(flag::VOWEL) {
                // get the stress value at the next position
                let stress = next_phoneme.stress;

                // TODO: Why the check for <0x80? Value never seems to be set that high
                if stress != 0 && stress < 0x80 {
                    // if next phoneme is stressed, and a VOWEL OR ER
                    // copy stress from next phoneme to this one
                    phoneme.stress = stress + 1;
                }
            }
        }
    }
}

fn set_phoneme_length(phonemes: &mut [Phoneme]) {
    for phoneme in phonemes.iter_mut() {
        let stress = phoneme.stress;

        if stress == 0 || stress > 0x7F {
            phoneme.length = PHONEME_LENGTH_TABLE[phoneme.index].0;
        } else {
            phoneme.length = PHONEME_LENGTH_TABLE[phoneme.index].1;
        }
    }
}

fn adjust_lengths(phonemes: &mut Vec<Phoneme>) {
    // LENGTHEN VOWELS PRECEDING PUNCTUATION
    //
    // Search for punctuation. If found, back up to the first vowel, then
    // process all phonemes between there and up to (but not including) the punctuation.
    // If any phoneme is found that is a either a fricative or voiced, the duration is
    // increased by (length * 1.5) + 1

    for position in 0..phonemes.len() {
        // not punctuation?
        if !phonemes[position].has_flag(flag::PUNCTUATION) {
            continue;
        }

        // Back up while not a vowel
        let mut vowel_position = position;
        while vowel_position > 0 && !phonemes[vowel_position - 1].has_flag(flag::VOWEL) {
            vowel_position -= 1;
        }

        // Vowel position now points to the last non-vowel. Decrement again to make it point to the
        // vowel itself.
        vowel_position = vowel_position.saturating_sub(1);

        // Now handle everything between the vowel up to the punctuation
        for phoneme in &mut phonemes[vowel_position..position] {
            // test for not fricative/unvoiced or not voiced
            if !phoneme.has_flag(flag::FRICATIVE) || phoneme.has_flag(flag::VOICED) {
                // change phoneme length to (length * 1.5) + 1
                phoneme.length += (phoneme.length >> 1) + 1;
            }
        }
    }

    // Similar to the above routine, but shorten vowels under some circumstances
    // Loop through all phonemes
    for loop_position in 0..phonemes.len() {
        let mut position = loop_position;

        // vowel?
        if phonemes[loop_position].has_flag(flag::VOWEL) {
            // get next phoneme
            position += 1;

            // The reference implementation does not check for bounds here, causing the phoneme to
            // be null. This will fail all has_flag checks, effectively marking the position as a
            // vowel and doing nothing (because the phoneme index checks fail).
            if position >= phonemes.len() {
                continue;
            }

            let vowel_phoneme_position = Some(position);

            // not a consonant
            if !phonemes[position].has_flag(flag::CONSONANT) {
                // 'RX' or 'LX'?
                if phonemes[position].index == PHONEME_RX || phonemes[position].index == PHONEME_LX {
                    position += 1;

                    if phonemes[position].has_flag(flag::CONSONANT) {
                        // followed by consonant?

                        // decrease length of vowel by 1 frame
                        phonemes[loop_position].length -= 1;
                    }
                }

                continue;
            }

            // Got here if not <VOWEL>
            // FIXME: the case when phoneme === END is taken over by !phonemeHasFlag(phoneme, FLAG_CONSONANT)
            let flags = vowel_phoneme_position.map_or(
                flag::CONSONANT | flag::UNVOICED_PLOSIVE,
                |position| PHONEME_FLAGS[phonemes[position].index]
            );

            // Unvoiced
            if flags & flag::VOICED == 0 {
                // *, .*, ?*, ,*, -*, DX, S*, SH, F*, TH, /H, /X, CH, P*, T*, K*, KX

                // unvoiced plosive
                if flags & flag::UNVOICED_PLOSIVE != 0 {
                    // RULE: <VOWEL> <UNVOICED PLOSIVE>
                    // <VOWEL> <P*, T*, K*, KX>
                    phonemes[loop_position].length -= phonemes[loop_position].length >> 3;
                }

                continue;
            }

            // RULE: <VOWEL> <VOWEL or VOICED CONSONANT>
            // <VOWEL> <IY, IH, EH, AE, AA, AH, AO, UH, AX, IX, ER, UX, OH, RX, LX, WX, YX, WH, R*, L*, W*,
            //          Y*, M*, N*, NX, Q*, Z*, ZH, V*, DH, J*, EY, AY, OY, AW, OW, UW, B*, D*, G*, GX>
            // increase length
            phonemes[loop_position].length += (phonemes[loop_position].length >> 2) + 1;

            continue;
        }

        //  *, .*, ?*, ,*, -*, WH, R*, L*, W*, Y*, M*, N*, NX, DX, Q*, S*, SH, F*,
        // TH, /H, /X, Z*, ZH, V*, DH, CH, J*, B*, D*, G*, GX, P*, T*, K*, KX

        // nasal?
        if phonemes[loop_position].has_flag(flag::NASAL) {
            // RULE: <NASAL> <STOP CONSONANT>
            //       Set punctuation length to 6
            //       Set stop consonant length to 5
            // M*, N*, NX,

            // is next phoneme a stop consonant?
            if let Some(phoneme) = phonemes.get_mut(loop_position + 1) {
                if phoneme.has_flag(flag::PLOSIVE) {
                    // B*, D*, G*, GX, P*, T*, K*, KX
                    phoneme.length = 6;
                    phonemes[loop_position].length = 5;
                }
            }

            continue;
        }

        //  *, .*, ?*, ,*, -*, WH, R*, L*, W*, Y*, DX, Q*, S*, SH, F*, TH,
        // /H, /X, Z*, ZH, V*, DH, CH, J*, B*, D*, G*, GX, P*, T*, K*, KX

        // stop consonant?
        if phonemes[loop_position].has_flag(flag::PLOSIVE) {
            // B*, D*, G*, GX

            // RULE: <STOP CONSONANT> {optional silence} <STOP CONSONANT>
            //       Shorten both to (length/2 + 1)

            // Move past silence
            let mut position = loop_position + 1;
            while position < phonemes.len() && phonemes[position].index == PHONEME_PAUSE {
                position += 1;
            }

            // if another stop consonant, process.
            if let Some(phoneme) = phonemes.get_mut(position) {
                if phoneme.has_flag(flag::PLOSIVE) {
                    // RULE: <STOP CONSONANT> {optional silence} <STOP CONSONANT>
                    phoneme.length = (phoneme.length >> 1) + 1;
                    phonemes[loop_position].length = (phonemes[loop_position].length >> 1) + 1;
                }
            }

            continue;
        }

        //  *, .*, ?*, ,*, -*, WH, R*, L*, W*, Y*, DX, Q*, S*, SH, F*, TH,
        // /H, /X, Z*, ZH, V*, DH, CH, J*

        // liquid consonant following a plosive
        if loop_position > 0 && phonemes[loop_position].has_flag(flag::LIQUID) && phonemes[loop_position - 1].has_flag(flag::PLOSIVE) {
            // R*, L*, W*, Y*
            // RULE: <STOP CONSONANT> <LIQUID>
            //       Decrease <LIQUID> by 2
            // prior phoneme is a stop consonant
            // decrease the phoneme length by 2 frames
            phonemes[loop_position].length -= 2;
        }
    }
}

fn prolong_plosives(phonemes: &mut Vec<Phoneme>) {
    let mut position = 0;

    while position < phonemes.len() {
        // Not a stop consonant, move to next one.
        if !phonemes[position].has_flag(flag::PLOSIVE) {
            position += 1;
            continue;
        }

        // If plosive, move to next non-empty phoneme and validate the flags.
        if phonemes[position].has_flag(flag::UNVOICED_PLOSIVE) {
            let mut next_non_empty = position + 1;
            while phonemes.get(next_non_empty).map_or(false, |phoneme| phoneme.index == PHONEME_PAUSE) {
                next_non_empty += 1;
            }

            // If not END and either flag 0x0008 or '/H' or '/X'
            if let Some(phoneme) = phonemes.get(next_non_empty) {
                if phoneme.has_flag(flag::OX0008) || phoneme.index == PHONEME_SLASH_H || phoneme.index == PHONEME_SLASH_X {
                    position += 1;
                    continue;
                }
            }
        }

        phonemes.insert(position + 1, Phoneme {
            index: phonemes[position].index + 1,
            stress: phonemes[position].stress,
            length: PHONEME_LENGTH_TABLE[phonemes[position].index + 1].0
        });

        phonemes.insert(position + 2, Phoneme {
            index: phonemes[position].index + 2,
            stress: phonemes[position].stress,
            length: PHONEME_LENGTH_TABLE[phonemes[position].index + 2].0
        });

        position += 3;
    }
}

pub fn parse_phonemes(text: &str) -> Result<Vec<Phoneme>, ParseError> {
    // TODO: Find a better name for this

    // Parser1
    let mut result = parser1(text);

    // Parser2
    parser2(&mut result)?;

    // CopyStress
    copy_stress(&mut result.phonemes);

    // SetPhonemeLength
    set_phoneme_length(&mut result.phonemes);

    // AdjustLengths
    adjust_lengths(&mut result.phonemes);

    // ProlongPlosiveStopConsonantsCode41240
    prolong_plosives(&mut result.phonemes);

    // Filter pauses
    result.phonemes.retain(|phoneme| phoneme.index != PHONEME_PAUSE);

    Ok(result.phonemes)
}
