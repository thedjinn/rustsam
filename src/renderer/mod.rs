use std::cmp::Ordering;

use crate::parser::Phoneme;

mod tests;

// Frequency data for each of the three formant waveforms
const FREQUENCY_DATA: (&[u8], &[u8], &[u8]) = (
    &[
        0x00, // ' *' 00
        0x13, // '.*' 01
        0x13, // '?*' 02
        0x13, // ',*' 03
        0x13, // '-*' 04
        0x0A, // 'IY' 05
        0x0E, // 'IH' 06
        0x13, // 'EH' 07
        0x18, // 'AE' 08
        0x1B, // 'AA' 09
        0x17, // 'AH' 10
        0x15, // 'AO' 11
        0x10, // 'UH' 12
        0x14, // 'AX' 13
        0x0E, // 'IX' 14
        0x12, // 'ER' 15
        0x0E, // 'UX' 16
        0x12, // 'OH' 17
        0x12, // 'RX' 18
        0x10, // 'LX' 19
        0x0D, // 'WX' 20
        0x0F, // 'YX' 21
        0x0B, // 'WH' 22
        0x12, // 'R*' 23
        0x0E, // 'L*' 24
        0x0B, // 'W*' 25
        0x09, // 'Y*' 26
        0x06, // 'M*' 27
        0x06, // 'N*' 28
        0x06, // 'NX' 29
        0x06, // 'DX' 30
        0x11, // 'Q*' 31
        0x06, // 'S*' 32
        0x06, // 'SH' 33
        0x06, // 'F*' 34
        0x06, // 'TH' 35
        0x0E, // '/H' 36
        0x10, // '/X' 37
        0x09, // 'Z*' 38
        0x0A, // 'ZH' 39
        0x08, // 'V*' 40
        0x0A, // 'DH' 41
        0x06, // 'CH' 42
        0x06, // '**' 43
        0x06, // 'J*' 44
        0x05, // '**' 45
        0x06, // '**' 46
        0x00, // '**' 47
        0x13, // 'EY' 48
        0x1B, // 'AY' 49
        0x15, // 'OY' 50
        0x1B, // 'AW' 51
        0x12, // 'OW' 52
        0x0D, // 'UW' 53
        0x06, // 'B*' 54
        0x06, // '**' 55
        0x06, // '**' 56
        0x06, // 'D*' 57
        0x06, // '**' 58
        0x06, // '**' 59
        0x06, // 'G*' 60
        0x06, // '**' 61
        0x06, // '**' 62
        0x06, // 'GX' 63
        0x06, // '**' 64
        0x06, // '**' 65
        0x06, // 'P*' 66
        0x06, // '**' 67
        0x06, // '**' 68
        0x06, // 'T*' 69
        0x06, // '**' 70
        0x06, // '**' 71
        0x06, // 'K*' 72
        0x0A, // '**' 73
        0x0A, // '**' 74
        0x06, // 'KX' 75
        0x06, // '**' 76
        0x06, // '**' 77
        0x2C, // 'UL' 78
        0x13  // 'UM' 79
    ],
    &[
        0x00, // ' *' 00
        0x43, // '.*' 01
        0x43, // '?*' 02
        0x43, // ',*' 03
        0x43, // '-*' 04
        0x54, // 'IY' 05
        0x49, // 'IH' 06
        0x43, // 'EH' 07
        0x3F, // 'AE' 08
        0x28, // 'AA' 09
        0x2C, // 'AH' 10
        0x1F, // 'AO' 11
        0x25, // 'UH' 12
        0x2D, // 'AX' 13
        0x49, // 'IX' 14
        0x31, // 'ER' 15
        0x24, // 'UX' 16
        0x1E, // 'OH' 17
        0x33, // 'RX' 18
        0x25, // 'LX' 19
        0x1D, // 'WX' 20
        0x45, // 'YX' 21
        0x18, // 'WH' 22
        0x32, // 'R*' 23
        0x1E, // 'L*' 24
        0x18, // 'W*' 25
        0x53, // 'Y*' 26
        0x2E, // 'M*' 27
        0x36, // 'N*' 28
        0x56, // 'NX' 29
        0x36, // 'DX' 30
        0x43, // 'Q*' 31
        0x49, // 'S*' 32
        0x4F, // 'SH' 33
        0x1A, // 'F*' 34
        0x42, // 'TH' 35
        0x49, // '/H' 36
        0x25, // '/X' 37
        0x33, // 'Z*' 38
        0x42, // 'ZH' 39
        0x28, // 'V*' 40
        0x2F, // 'DH' 41
        0x4F, // 'CH' 42
        0x4F, // '**' 43
        0x42, // 'J*' 44
        0x4F, // '**' 45
        0x6E, // '**' 46
        0x00, // '**' 47
        0x48, // 'EY' 48
        0x27, // 'AY' 49
        0x1F, // 'OY' 50
        0x2B, // 'AW' 51
        0x1E, // 'OW' 52
        0x22, // 'UW' 53
        0x1A, // 'B*' 54
        0x1A, // '**' 55
        0x1A, // '**' 56
        0x42, // 'D*' 57
        0x42, // '**' 58
        0x42, // '**' 59
        0x6E, // 'G*' 60
        0x6E, // '**' 61
        0x6E, // '**' 62
        0x54, // 'GX' 63
        0x54, // '**' 64
        0x54, // '**' 65
        0x1A, // 'P*' 66
        0x1A, // '**' 67
        0x1A, // '**' 68
        0x42, // 'T*' 69
        0x42, // '**' 70
        0x42, // '**' 71
        0x6D, // 'K*' 72
        0x56, // '**' 73
        0x6D, // '**' 74
        0x54, // 'KX' 75
        0x54, // '**' 76
        0x54, // '**' 77
        0x7F, // 'UL' 78
        0x7F  // 'UM' 79
    ],
    &[
        0x00, // ' *' 00
        0x5B, // '.*' 01
        0x5B, // '?*' 02
        0x5B, // ',*' 03
        0x5B, // '-*' 04
        0x6E, // 'IY' 05
        0x5D, // 'IH' 06
        0x5B, // 'EH' 07
        0x58, // 'AE' 08
        0x59, // 'AA' 09
        0x57, // 'AH' 10
        0x58, // 'AO' 11
        0x52, // 'UH' 12
        0x59, // 'AX' 13
        0x5D, // 'IX' 14
        0x3E, // 'ER' 15
        0x52, // 'UX' 16
        0x58, // 'OH' 17
        0x3E, // 'RX' 18
        0x6E, // 'LX' 19
        0x50, // 'WX' 20
        0x5D, // 'YX' 21
        0x5A, // 'WH' 22
        0x3C, // 'R*' 23
        0x6E, // 'L*' 24
        0x5A, // 'W*' 25
        0x6E, // 'Y*' 26
        0x51, // 'M*' 27
        0x79, // 'N*' 28
        0x65, // 'NX' 29
        0x79, // 'DX' 30
        0x5B, // 'Q*' 31
        0x63, // 'S*' 32
        0x6A, // 'SH' 33
        0x51, // 'F*' 34
        0x79, // 'TH' 35
        0x5D, // '/H' 36
        0x52, // '/X' 37
        0x5D, // 'Z*' 38
        0x67, // 'ZH' 39
        0x4C, // 'V*' 40
        0x5D, // 'DH' 41
        0x65, // 'CH' 42
        0x65, // '**' 43
        0x79, // 'J*' 44
        0x65, // '**' 45
        0x79, // '**' 46
        0x00, // '**' 47
        0x5A, // 'EY' 48
        0x58, // 'AY' 49
        0x58, // 'OY' 50
        0x58, // 'AW' 51
        0x58, // 'OW' 52
        0x52, // 'UW' 53
        0x51, // 'B*' 54
        0x51, // '**' 55
        0x51, // '**' 56
        0x79, // 'D*' 57
        0x79, // '**' 58
        0x79, // '**' 59
        0x70, // 'G*' 60
        0x6E, // '**' 61
        0x6E, // '**' 62
        0x5E, // 'GX' 63
        0x5E, // '**' 64
        0x5E, // '**' 65
        0x51, // 'P*' 66
        0x51, // '**' 67
        0x51, // '**' 68
        0x79, // 'T*' 69
        0x79, // '**' 70
        0x79, // '**' 71
        0x65, // 'K*' 72
        0x65, // '**' 73
        0x70, // '**' 74
        0x5E, // 'KX' 75
        0x5E, // '**' 76
        0x5E, // '**' 77
        0x08, // 'UL' 78
        0x01  // 'UM' 79
    ]
);

const AMPLITUDE_DATA: &[(u8, u8, u8)] = &[
    (0x00, 0x00, 0x00), // ' *' 00
    (0x00, 0x00, 0x00), // '.*' 01
    (0x00, 0x00, 0x00), // '?*' 02
    (0x00, 0x00, 0x00), // ',*' 03
    (0x00, 0x00, 0x00), // '-*' 04
    (0x0D, 0x0A, 0x08), // 'IY' 05
    (0x0D, 0x0B, 0x07), // 'IH' 06
    (0x0E, 0x0D, 0x08), // 'EH' 07
    (0x0F, 0x0E, 0x08), // 'AE' 08
    (0x0F, 0x0D, 0x01), // 'AA' 09
    (0x0F, 0x0C, 0x01), // 'AH' 10
    (0x0F, 0x0C, 0x00), // 'AO' 11
    (0x0F, 0x0B, 0x01), // 'UH' 12
    (0x0C, 0x09, 0x00), // 'AX' 13
    (0x0D, 0x0B, 0x07), // 'IX' 14
    (0x0C, 0x0B, 0x05), // 'ER' 15
    (0x0F, 0x0C, 0x01), // 'UX' 16
    (0x0F, 0x0C, 0x00), // 'OH' 17
    (0x0D, 0x0C, 0x06), // 'RX' 18
    (0x0D, 0x08, 0x01), // 'LX' 19
    (0x0D, 0x08, 0x00), // 'WX' 20
    (0x0E, 0x0C, 0x07), // 'YX' 21
    (0x0D, 0x08, 0x00), // 'WH' 22
    (0x0C, 0x0A, 0x05), // 'R*' 23
    (0x0D, 0x08, 0x01), // 'L*' 24
    (0x0D, 0x08, 0x00), // 'W*' 25
    (0x0D, 0x0A, 0x08), // 'Y*' 26
    (0x0C, 0x03, 0x00), // 'M*' 27
    (0x09, 0x09, 0x00), // 'N*' 28
    (0x09, 0x06, 0x03), // 'NX' 29
    (0x00, 0x00, 0x00), // 'DX' 30
    (0x00, 0x00, 0x00), // 'Q*' 31
    (0x00, 0x00, 0x00), // 'S*' 32
    (0x00, 0x00, 0x00), // 'SH' 33
    (0x00, 0x00, 0x00), // 'F*' 34
    (0x00, 0x00, 0x00), // 'TH' 35
    (0x00, 0x00, 0x00), // '/H' 36
    (0x00, 0x00, 0x00), // '/X' 37
    (0x0B, 0x03, 0x00), // 'Z*' 38
    (0x0B, 0x05, 0x01), // 'ZH' 39
    (0x0B, 0x03, 0x00), // 'V*' 40
    (0x0B, 0x04, 0x00), // 'DH' 41
    (0x00, 0x00, 0x00), // 'CH' 42
    (0x00, 0x00, 0x00), // '**' 43
    (0x01, 0x00, 0x00), // 'J*' 44
    (0x0B, 0x05, 0x01), // '**' 45
    (0x00, 0x0A, 0x0E), // '**' 46
    (0x02, 0x02, 0x01), // '**' 47
    (0x0E, 0x0E, 0x09), // 'EY' 48
    (0x0F, 0x0D, 0x01), // 'AY' 49
    (0x0F, 0x0C, 0x00), // 'OY' 50
    (0x0F, 0x0D, 0x01), // 'AW' 51
    (0x0F, 0x0C, 0x00), // 'OW' 52
    (0x0D, 0x08, 0x00), // 'UW' 53
    (0x02, 0x00, 0x00), // 'B*' 54
    (0x04, 0x01, 0x00), // '**' 55
    (0x00, 0x00, 0x00), // '**' 56
    (0x02, 0x00, 0x00), // 'D*' 57
    (0x04, 0x01, 0x00), // '**' 58
    (0x00, 0x00, 0x00), // '**' 59
    (0x01, 0x00, 0x00), // 'G*' 60
    (0x04, 0x01, 0x00), // '**' 61
    (0x00, 0x00, 0x00), // '**' 62
    (0x01, 0x00, 0x00), // 'GX' 63
    (0x04, 0x01, 0x00), // '**' 64
    (0x00, 0x00, 0x00), // '**' 65
    (0x00, 0x00, 0x00), // 'P*' 66
    (0x00, 0x00, 0x00), // '**' 67
    (0x00, 0x00, 0x00), // '**' 68
    (0x00, 0x00, 0x00), // 'T*' 69
    (0x00, 0x00, 0x00), // '**' 70
    (0x00, 0x00, 0x00), // '**' 71
    (0x00, 0x00, 0x00), // 'K*' 72
    (0x0C, 0x0A, 0x07), // '**' 73
    (0x00, 0x00, 0x00), // '**' 74
    (0x00, 0x00, 0x00), // 'KX' 75
    (0x00, 0x0A, 0x05), // '**' 76
    (0x00, 0x00, 0x00), // '**' 77
    (0x0F, 0x00, 0x13), // 'UL' 78
    (0x0F, 0x00, 0x10)  // 'UM' 79
];

const SAMPLED_CONSONANT_FLAGS: &[u8] = &[
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xF1, 0xE2, 0xD3, 0xBB, 0x7C, 0x95, 0x01, 0x02,
    0x03, 0x03, 0x00, 0x72, 0x00, 0x02, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x1B, 0x00, 0x00, 0x19, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];

#[derive(Debug, Eq, PartialEq)]
struct FrequencyData {
    f1: Vec<u8>,
    f2: Vec<u8>,
    f3: Vec<u8>
}

fn set_mouth_and_throat(mouth: u8, throat: u8) -> FrequencyData {
    // TODO: Convert to constructor?

    fn trans(factor: u8, frequency: u8) -> u8 {
        // Compute (((factor * frequency) / 256) % 256) * 2
        // Note: this assumes all of the frequencies are 7 bit values (to prevent overflowing).
        ((((factor as u16 * frequency as u16) >> 8) & 0xff) << 1) as u8
    }

    let mut frequency_data = FrequencyData {
        f1: FREQUENCY_DATA.0.into(),
        f2: FREQUENCY_DATA.1.into(),
        f3: FREQUENCY_DATA.2.into()
    };

    // recalculate formant frequencies 5..29 for the mouth (F1) and throat (F2)
    for index in 5..30 {
        // recalculate f1 (mouth formant)
        frequency_data.f1[index] = trans(mouth, frequency_data.f1[index]);

        // recalculate f2 (throat formant)
        frequency_data.f2[index] = trans(throat, frequency_data.f2[index]);
    }

    // recalculate formant frequencies 48..53
    for index in 48..54 {
        // recalculate f1 (mouth formant)
        frequency_data.f1[index] = trans(mouth, frequency_data.f1[index]);

        // recalculate f2 (throat formant)
        frequency_data.f2[index] = trans(throat, frequency_data.f2[index]);
    }

    frequency_data
}

#[derive(Debug, Eq, PartialEq)]
struct Frame {
    pitch: u8,

    // Frequencies
    f1: u8,
    f2: u8,
    f3: u8,

    // Amplitudes
    a1: u8,
    a2: u8,
    a3: u8,

    sampled_consonant_flag: u8
}

impl Frame {
    fn new() -> Self {
        Self {
            pitch: 0,
            f1: 0,
            f2: 0,
            f3: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            sampled_consonant_flag: 0
        }
    }
}

const RISING_INFLECTION: u8 = 255;
//const FALLING_INFLECTION: u8 = 1;

enum Inflection {
    Rising,
    Falling
}

const STRESS_PITCH_TABLE: &[u8] = &[
    0x00, 0xE0, 0xE6, 0xEC, 0xF3,
    0xF9, 0x00, 0x06, 0x0C, 0x06
];

/// Apply rising or falling inflection to the last 30 frames of the frame vec.
fn add_inflection(inflection: Inflection, frames: &mut [Frame]) {
    // store the location of the punctuation
    let end = frames.len();

    let mut position = end.saturating_sub(30);

    let mut a;

    // FIXME: Explain this fix better, it's not obvious
    // ML : A =, fixes a problem with invalid pitch with '.'
    loop {
        // TODO: Bounds checking
        a = frames[position].pitch;
        if a != 127 {
            break;
        }

        position += 1;
    }

    while position < end {
        // Add the inflection direction
        match inflection {
            Inflection::Falling => a += 1,
            Inflection::Rising => a -= 1
        }

        // Set the inflection
        frames[position].pitch = a;

        // Advance position to the next non-255 value, stopping at the end
        loop {
            position += 1;

            // TODO: Investigate why the un-equals check is here, possible bug?
            if position == end || frames[position].pitch != RISING_INFLECTION {
                break;
            }
        }
    }
}

// TODO: Figure out pitch range
fn create_frames(pitch: u8, phonemes: &[crate::parser::Phoneme], frequency_data: &FrequencyData) -> Vec<Frame> {
    let mut frames = Vec::new();

    for phoneme in phonemes {
        if phoneme.index == crate::parser::PHONEME_PERIOD {
            add_inflection(Inflection::Falling, &mut frames);
        } else if phoneme.index == crate::parser::PHONEME_QUESTION_MARK {
            add_inflection(Inflection::Rising, &mut frames);
        }

        // get the stress amount (more stress = higher pitch)
        let phase1 = STRESS_PITCH_TABLE[phoneme.stress as usize];

        // get number of frames to write
        // copy from the source to the frames list
        frames.extend((0..phoneme.length).map(|_| Frame {
            pitch: (pitch.wrapping_add(phase1)),

            f1: frequency_data.f1[phoneme.index],
            f2: frequency_data.f2[phoneme.index],
            f3: frequency_data.f3[phoneme.index],

            a1: AMPLITUDE_DATA[phoneme.index].0,
            a2: AMPLITUDE_DATA[phoneme.index].1,
            a3: AMPLITUDE_DATA[phoneme.index].2,

            sampled_consonant_flag: SAMPLED_CONSONANT_FLAGS[phoneme.index]
        }));
    }

    frames
}

const BLEND_RANK: &[u8] = &[
    0x00, 0x1F, 0x1F, 0x1F, 0x1F, 0x02, 0x02, 0x02,
    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x05, 0x05,
    0x02, 0x0A, 0x02, 0x08, 0x05, 0x05, 0x0B, 0x0A,
    0x09, 0x08, 0x08, 0xA0, 0x08, 0x08, 0x17, 0x1F,
    0x12, 0x12, 0x12, 0x12, 0x1E, 0x1E, 0x14, 0x14,
    0x14, 0x14, 0x17, 0x17, 0x1A, 0x1A, 0x1D, 0x1D,
    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x1A, 0x1D,
    0x1B, 0x1A, 0x1D, 0x1B, 0x1A, 0x1D, 0x1B, 0x1A,
    0x1D, 0x1B, 0x17, 0x1D, 0x17, 0x17, 0x1D, 0x17,
    0x17, 0x1D, 0x17, 0x17, 0x1D, 0x17, 0x17, 0x17
];

const OUT_BLEND_LENGTH: &[u8] = &[
    0x00, 0x02, 0x02, 0x02, 0x02, 0x04, 0x04, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x04, 0x03, 0x02, 0x04, 0x04, 0x02, 0x02,
    0x02, 0x02, 0x02, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x02,
    0x02, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x02, 0x00, 0x01, 0x02, 0x00, 0x01, 0x02,
    0x00, 0x01, 0x02, 0x00, 0x02, 0x02, 0x00, 0x01,
    0x03, 0x00, 0x02, 0x03, 0x00, 0x02, 0xA0, 0xA0
];

const IN_BLEND_LENGTH: &[u8] = &[
    0x00, 0x02, 0x02, 0x02, 0x02, 0x04, 0x04, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x04, 0x03, 0x03, 0x04, 0x04, 0x03, 0x03,
    0x03, 0x03, 0x03, 0x01, 0x02, 0x03, 0x02, 0x01,
    0x03, 0x03, 0x03, 0x03, 0x01, 0x01, 0x03, 0x03,
    0x03, 0x02, 0x02, 0x03, 0x02, 0x03, 0x00, 0x00,
    0x05, 0x05, 0x05, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x02, 0x02, 0x00, 0x03, 0x02, 0x00, 0x04, 0x02,
    0x00, 0x03, 0x02, 0x00, 0x02, 0x02, 0x00, 0x02,
    0x03, 0x00, 0x03, 0x03, 0x00, 0x03, 0xB0, 0xA0
];

/*
fn interpolate_buffer(buffer: &mut [u8], change: Option<i8>) {
    let width = buffer.len() - 1;

    let change = change.unwrap_or_else(|| buffer[width] as i8 - buffer[0] as i8);

    let sign = change < 0;
    let remainder = change.unsigned_abs() % width as u8;
    let div = change.checked_div(width as i8).unwrap_or(0);

    //println!("sign remainder div change width {} {} {} {} {}", sign, remainder, div, change, width);

    let mut error = 0;

    for position in 1..width {
        // Ensure value is 7 bits so i8 can be used
        assert!(buffer[position - 1] < 128);

        let mut value = buffer[position - 1] as i8 + div;

        error += remainder;

        if error >= width as u8 {
            // accumulated a whole integer error, so adjust output
            error -= width as u8;

            if sign {
                value -= 1;
            } else if value != 0 {
                // if value is 0, we always leave it alone
                value += 1;
            }
        }

        buffer[position] = value as u8;
    }

    //println!("iterated {} times, len = {}", width - 1, buffer.len());
}
*/

fn interpolate<F>(width: u8, mut table: F, frame: isize, change: i8)
where
    F: FnMut(usize, Option<u8>) -> u8
{
    let sign = change < 0;
    let remainder = change.unsigned_abs() % width;
    let div = change.checked_div(width as i8).unwrap_or(0);

    let mut error = 0;

    for position in (frame + 1)..(frame + width as isize) {
        // The reference implementation has a bug where the starting frame can sometimes be a
        // negative index. In JavaScript such an array lookup will result in a NaN value, causing
        // the interpolator to write a NaN value, and creating NaN feedback for the rest of the
        // interpolation sequence.
        if position < 1 {
            table(position as usize, Some(0));
            continue;
        }

        let value = table(position as usize - 1, None);

        // Ensure value is 7 bits so i8 can be used safely
        //assert!(value < 128);

        let mut value = value as i16 + div as i16;

        error += remainder;
        if error >= width {
            // Accumulated a whole integer error, so adjust output
            error -= width;

            if sign {
                value -= 1;
            } else if value != 0 {
                // If value is zero it should always be left alone
                value += 1;
            }
        }

        table(position as usize, Some(value as u8));
    }
}

fn create_transitions(frames: &mut Vec<Frame>, phonemes: &[Phoneme]) -> usize {
    let mut boundary: usize = 0;

    for position in 0..(phonemes.len() - 1) {
        let phoneme      = phonemes[position].index;
        let next_phoneme = phonemes[position + 1].index;

        // get the ranking of each phoneme
        let next_rank = BLEND_RANK[next_phoneme];
        let rank      = BLEND_RANK[phoneme];

        // compare the rank - lower rank value is stronger
        let (out_blend_frames, in_blend_frames) = match rank.cmp(&next_rank) {
            // Same rank, so use out blend lengths from each phoneme
            Ordering::Equal => (OUT_BLEND_LENGTH[phoneme], OUT_BLEND_LENGTH[next_phoneme]),

            // Next phoneme is stronger, so use its blend lengths
            Ordering::Less => (IN_BLEND_LENGTH[next_phoneme], OUT_BLEND_LENGTH[next_phoneme]),

            // Current phoneme is stronger, so use its blend lengths. Note: the out/in are swapped
            Ordering::Greater => (OUT_BLEND_LENGTH[phoneme], IN_BLEND_LENGTH[phoneme])
        };

        boundary = boundary.wrapping_add(phonemes[position].length as usize);

        let trans_end        = boundary + in_blend_frames as usize;
        let trans_start  = boundary as isize - out_blend_frames as isize;
        let trans_length = out_blend_frames + in_blend_frames; // total transition

        // TODO: What does the & 128 do? Check for positive numbers?
        if (trans_length.wrapping_sub(2)) & 128 == 0 {
            // unlike the other values, the pitches[] interpolates from
            // the middle of the current phoneme to the middle of the
            // next phoneme

            // half the width of the current and next phoneme
            let cur_width  = phonemes[position].length >> 1;
            let next_width = phonemes[position + 1].length >> 1;

            // Interpolate the pitch
            // TODO: The start position for pitch doesn't seem correct, needs verification
            let pitch = (frames[boundary + next_width as usize].pitch as i16 - frames[boundary - cur_width as usize].pitch as i16) as i8;

            interpolate(
                cur_width + next_width,
                |index, value| {
                    if let Some(value) = value {
                        // The reference implementation has a bug where it tries to interpolate off
                        // of the end of the frame list. This creates new frames that shouldn't
                        // actually exist. To prevent any indexing panics a new frame is added when
                        // writing off of the end. The new frame is populated with zeroes to mimic
                        // the behavior of "undefined" values in JavaScript in a safe way.
                        if index == frames.len() {
                            frames.push(Frame::new());
                        }

                        frames[index].pitch = value
                    }

                    // TODO: This check is only necessary for debugging
                    if index == frames.len() {
                        0
                    } else {
                        frames[index].pitch
                    }
                },
                trans_start,
                pitch
            );

            //let mut buffer = frames[range.clone()].iter().map(|frame| frame.pitch).collect::<Vec<_>>();
            //interpolate_buffer(&mut buffer, Some(pitch));
            //frames[range.clone()].iter_mut().zip(buffer.into_iter()).for_each(|(frame, value)| frame.pitch = value);

            // Interpolate the other values
            //let range = trans_start as usize..(trans_start as usize + trans_length as usize + 1);

            // The reference implementation has a bug here where it on some occasions tries to
            // interpolate one frame beyond the end of the frame list. This causes the change delta
            // to become NaN, causing the interpolator to leave the frames untouched (this might
            // not be the desired result). To prevent any panics the interpolation is skipped when
            // trans_end goes beyond the frame list.
            if trans_end >= frames.len() {
                continue;
            }

            let change = frames.get(trans_start as usize).map(|frame| frames[trans_end].f1 as i8 - frame.f1 as i8).unwrap_or(0);
            interpolate(
                trans_length,
                |index, value| {
                    if let Some(value) = value {
                        // The reference implementation has a bug where it tries to interpolate off
                        // of the end of the frame list. This creates new frames that shouldn't
                        // actually exist. To prevent any indexing panics a new frame is added when
                        // writing off of the end. The new frame is populated with zeroes to mimic
                        // the behavior of "undefined" values in JavaScript in a safe way.
                        //if index == frames.len() {
                            //frames.push(Frame::new());
                        //}

                        frames[index].f1 = value
                    }

                    frames[index].f1
                },
                trans_start,
                change
            );

            let change = frames.get(trans_start as usize).map(|frame| frames[trans_end].f2 as i8 - frame.f2 as i8).unwrap_or(0);
            interpolate(
                trans_length,
                |index, value| {
                    if let Some(value) = value {
                        frames[index].f2 = value
                    }

                    frames[index].f2
                },
                trans_start,
                change
            );

            let change = frames.get(trans_start as usize).map(|frame| frames[trans_end].f3 as i8 - frame.f3 as i8).unwrap_or(0);
            interpolate(
                trans_length,
                |index, value| {
                    if let Some(value) = value {
                        frames[index].f3 = value
                    }

                    frames[index].f3
                },
                trans_start,
                change
            );

            let change = frames.get(trans_start as usize).map(|frame| frames[trans_end].a1 as i8 - frame.a1 as i8).unwrap_or(0);
            interpolate(
                trans_length,
                |index, value| {
                    if let Some(value) = value {
                        frames[index].a1 = value
                    }

                    frames[index].a1
                },
                trans_start,
                change
            );

            let change = frames.get(trans_start as usize).map(|frame| frames[trans_end].a2 as i8 - frame.a2 as i8).unwrap_or(0);
            interpolate(
                trans_length,
                |index, value| {
                    if let Some(value) = value {
                        frames[index].a2 = value
                    }

                    frames[index].a2
                },
                trans_start,
                change
            );

            let change = frames.get(trans_start as usize).map(|frame| frames[trans_end].a3 as i8 - frame.a3 as i8).unwrap_or(0);
            interpolate(
                trans_length,
                |index, value| {
                    if let Some(value) = value {
                        frames[index].a3 = value
                    }

                    frames[index].a3
                },
                trans_start,
                change
            );
        }
    }

    // add the length of last phoneme
    boundary + phonemes[phonemes.len() - 1].length as usize
}

const AMPLITUDE_RESCALE_TABLE: &[u8] = &[
    0x00, 0x01, 0x02, 0x02, 0x02, 0x03, 0x03, 0x04,
    0x04, 0x05, 0x06, 0x08, 0x09, 0x0B, 0x0D, 0x0F
];

struct PreparedFrames {
    // TODO: How does this relate to frames.len()?
    frame_count: usize,
    frames: Vec<Frame>
}

fn prepare_frames(phonemes: &[Phoneme], pitch: u8, mouth: u8, throat: u8, sing_mode: bool) -> PreparedFrames {
    let frequency_data = set_mouth_and_throat(mouth, throat);
    let mut frames = create_frames(pitch, phonemes, &frequency_data);
    let t = create_transitions(&mut frames, phonemes);

    if !sing_mode {
        // Assing pitch contour
        // subtract half the frequency of the formant 1.
        // this adds variety to the voice
        for frame in frames.iter_mut() {
            frame.pitch = frame.pitch.saturating_sub(frame.f1 >> 1);
        }
    }

    // Rescale volume from decibels to the linear scale.
    for frame in frames.iter_mut() {
        frame.a1 = AMPLITUDE_RESCALE_TABLE[frame.a1 as usize];
        frame.a2 = AMPLITUDE_RESCALE_TABLE[frame.a2 as usize];
        frame.a3 = AMPLITUDE_RESCALE_TABLE[frame.a3 as usize];
    }

    PreparedFrames {
        frame_count: t,
        frames
    }
}

// Timetable for more accurate C64 simulation
const TIMETABLE: &[[u8; 5]] = &[
    [162, 167, 167, 127, 128], // formants synth
    [226,  60,  60,   0,   0], // unvoiced sample 0
    [225,  60,  59,   0,   0], // unvoiced sample 1
    [200,   0,   0,  54,  55], // voiced sample 0
    [199,   0,   0,  54,  54]  // voiced sample 1
];

struct OutputBuffer {
    buffer: Vec<u8>,
    position: usize,
    old_timetable_index: usize
}

impl OutputBuffer {
    fn new(size: usize) -> Self {
        Self {
            buffer: vec![0; size],
            position: 0,
            old_timetable_index: 0
        }
    }

    fn ary(&mut self, index: usize, array: [u8; 5]) {
        // TODO: index seems to be 0..=2, needs to be verified more on longer sentences
        self.position += TIMETABLE[self.old_timetable_index][index] as usize;

        self.old_timetable_index = index;

        // Write a little bit in advance
        for (index, sample) in array.into_iter().enumerate() {
            self.buffer[self.position / 50 + index] = sample;
        }
    }

    fn get(&self) -> &[u8] {
        &self.buffer[..(self.position / 50)]
    }

    fn write(&mut self, index: usize, a: u8) {
        // Scale by 16 and write 5 times
        // Note: renderer passes in values that are > 16, these are overflowing
        let scaled = (a & 15) * 16;

        self.ary(index, [scaled, scaled, scaled, scaled, scaled]);
    }
}

// Sampled data for consonants, consisting of five 256-byte sections
const SAMPLE_TABLE: &[u8] = &[
    //00  T', S, Z  (coronal)
    0x38, 0x84, 0x6B, 0x19, 0xC6, 0x63, 0x18, 0x86,
    0x73, 0x98, 0xC6, 0xB1, 0x1C, 0xCA, 0x31, 0x8C,
    0xC7, 0x31, 0x88, 0xC2, 0x30, 0x98, 0x46, 0x31,
    0x18, 0xC6, 0x35, 0x0C, 0xCA, 0x31, 0x0C, 0xC6,

    //20
    0x21, 0x10, 0x24, 0x69, 0x12, 0xC2, 0x31, 0x14,
    0xC4, 0x71, 0x08, 0x4A, 0x22, 0x49, 0xAB, 0x6A,
    0xA8, 0xAC, 0x49, 0x51, 0x32, 0xD5, 0x52, 0x88,
    0x93, 0x6C, 0x94, 0x22, 0x15, 0x54, 0xD2, 0x25,

    //40
    0x96, 0xD4, 0x50, 0xA5, 0x46, 0x21, 0x08, 0x85,
    0x6B, 0x18, 0xC4, 0x63, 0x10, 0xCE, 0x6B, 0x18,
    0x8C, 0x71, 0x19, 0x8C, 0x63, 0x35, 0x0C, 0xC6,
    0x33, 0x99, 0xCC, 0x6C, 0xB5, 0x4E, 0xA2, 0x99,

    //60
    0x46, 0x21, 0x28, 0x82, 0x95, 0x2E, 0xE3, 0x30,
    0x9C, 0xC5, 0x30, 0x9C, 0xA2, 0xB1, 0x9C, 0x67,
    0x31, 0x88, 0x66, 0x59, 0x2C, 0x53, 0x18, 0x84,
    0x67, 0x50, 0xCA, 0xE3, 0x0A, 0xAC, 0xAB, 0x30,

    //80
    0xAC, 0x62, 0x30, 0x8C, 0x63, 0x10, 0x94, 0x62,
    0xB1, 0x8C, 0x82, 0x28, 0x96, 0x33, 0x98, 0xD6,
    0xB5, 0x4C, 0x62, 0x29, 0xA5, 0x4A, 0xB5, 0x9C,
    0xC6, 0x31, 0x14, 0xD6, 0x38, 0x9C, 0x4B, 0xB4,

    //A0
    0x86, 0x65, 0x18, 0xAE, 0x67, 0x1C, 0xA6, 0x63,
    0x19, 0x96, 0x23, 0x19, 0x84, 0x13, 0x08, 0xA6,
    0x52, 0xAC, 0xCA, 0x22, 0x89, 0x6E, 0xAB, 0x19,
    0x8C, 0x62, 0x34, 0xC4, 0x62, 0x19, 0x86, 0x63,

    //C0
    0x18, 0xC4, 0x23, 0x58, 0xD6, 0xA3, 0x50, 0x42,
    0x54, 0x4A, 0xAD, 0x4A, 0x25, 0x11, 0x6B, 0x64,
    0x89, 0x4A, 0x63, 0x39, 0x8A, 0x23, 0x31, 0x2A,
    0xEA, 0xA2, 0xA9, 0x44, 0xC5, 0x12, 0xCD, 0x42,

    //E0
    0x34, 0x8C, 0x62, 0x18, 0x8C, 0x63, 0x11, 0x48,
    0x66, 0x31, 0x9D, 0x44, 0x33, 0x1D, 0x46, 0x31,
    0x9C, 0xC6, 0xB1, 0x0C, 0xCD, 0x32, 0x88, 0xC4,
    0x73, 0x18, 0x86, 0x73, 0x08, 0xD6, 0x63, 0x58,

    //100 CH', J', SH, ZH  (palato-alveolar)
    0x07, 0x81, 0xE0, 0xF0, 0x3C, 0x07, 0x87, 0x90,
    0x3C, 0x7C, 0x0F, 0xC7, 0xC0, 0xC0, 0xF0, 0x7C,
    0x1E, 0x07, 0x80, 0x80, 0x00, 0x1C, 0x78, 0x70,
    0xF1, 0xC7, 0x1F, 0xC0, 0x0C, 0xFE, 0x1C, 0x1F,

    //120
    0x1F, 0x0E, 0x0A, 0x7A, 0xC0, 0x71, 0xF2, 0x83,
    0x8F, 0x03, 0x0F, 0x0F, 0x0C, 0x00, 0x79, 0xF8,
    0x61, 0xE0, 0x43, 0x0F, 0x83, 0xE7, 0x18, 0xF9,
    0xC1, 0x13, 0xDA, 0xE9, 0x63, 0x8F, 0x0F, 0x83,

    //140
    0x83, 0x87, 0xC3, 0x1F, 0x3C, 0x70, 0xF0, 0xE1,
    0xE1, 0xE3, 0x87, 0xB8, 0x71, 0x0E, 0x20, 0xE3,
    0x8D, 0x48, 0x78, 0x1C, 0x93, 0x87, 0x30, 0xE1,
    0xC1, 0xC1, 0xE4, 0x78, 0x21, 0x83, 0x83, 0xC3,

    //160
    0x87, 0x06, 0x39, 0xE5, 0xC3, 0x87, 0x07, 0x0E,
    0x1C, 0x1C, 0x70, 0xF4, 0x71, 0x9C, 0x60, 0x36,
    0x32, 0xC3, 0x1E, 0x3C, 0xF3, 0x8F, 0x0E, 0x3C,
    0x70, 0xE3, 0xC7, 0x8F, 0x0F, 0x0F, 0x0E, 0x3C,

    //180
    0x78, 0xF0, 0xE3, 0x87, 0x06, 0xF0, 0xE3, 0x07,
    0xC1, 0x99, 0x87, 0x0F, 0x18, 0x78, 0x70, 0x70,
    0xFC, 0xF3, 0x10, 0xB1, 0x8C, 0x8C, 0x31, 0x7C,
    0x70, 0xE1, 0x86, 0x3C, 0x64, 0x6C, 0xB0, 0xE1,

    //1A0
    0xE3, 0x0F, 0x23, 0x8F, 0x0F, 0x1E, 0x3E, 0x38,
    0x3C, 0x38, 0x7B, 0x8F, 0x07, 0x0E, 0x3C, 0xF4,
    0x17, 0x1E, 0x3C, 0x78, 0xF2, 0x9E, 0x72, 0x49,
    0xE3, 0x25, 0x36, 0x38, 0x58, 0x39, 0xE2, 0xDE,

    //1C0
    0x3C, 0x78, 0x78, 0xE1, 0xC7, 0x61, 0xE1, 0xE1,
    0xB0, 0xF0, 0xF0, 0xC3, 0xC7, 0x0E, 0x38, 0xC0,
    0xF0, 0xCE, 0x73, 0x73, 0x18, 0x34, 0xB0, 0xE1,
    0xC7, 0x8E, 0x1C, 0x3C, 0xF8, 0x38, 0xF0, 0xE1,

    //1E0
    0xC1, 0x8B, 0x86, 0x8F, 0x1C, 0x78, 0x70, 0xF0,
    0x78, 0xAC, 0xB1, 0x8F, 0x39, 0x31, 0xDB, 0x38,
    0x61, 0xC3, 0x0E, 0x0E, 0x38, 0x78, 0x73, 0x17,
    0x1E, 0x39, 0x1E, 0x38, 0x64, 0xE1, 0xF1, 0xC1,

    //200 P', F, V, TH, DH  ([labio]dental)
    0x4E, 0x0F, 0x40, 0xA2, 0x02, 0xC5, 0x8F, 0x81,
    0xA1, 0xFC, 0x12, 0x08, 0x64, 0xE0, 0x3C, 0x22,
    0xE0, 0x45, 0x07, 0x8E, 0x0C, 0x32, 0x90, 0xF0,
    0x1F, 0x20, 0x49, 0xE0, 0xF8, 0x0C, 0x60, 0xF0,

    //220
    0x17, 0x1A, 0x41, 0xAA, 0xA4, 0xD0, 0x8D, 0x12,
    0x82, 0x1E, 0x1E, 0x03, 0xF8, 0x3E, 0x03, 0x0C,
    0x73, 0x80, 0x70, 0x44, 0x26, 0x03, 0x24, 0xE1,
    0x3E, 0x04, 0x4E, 0x04, 0x1C, 0xC1, 0x09, 0xCC,

    //240
    0x9E, 0x90, 0x21, 0x07, 0x90, 0x43, 0x64, 0xC0,
    0x0F, 0xC6, 0x90, 0x9C, 0xC1, 0x5B, 0x03, 0xE2,
    0x1D, 0x81, 0xE0, 0x5E, 0x1D, 0x03, 0x84, 0xB8,
    0x2C, 0x0F, 0x80, 0xB1, 0x83, 0xE0, 0x30, 0x41,

    //260
    0x1E, 0x43, 0x89, 0x83, 0x50, 0xFC, 0x24, 0x2E,
    0x13, 0x83, 0xF1, 0x7C, 0x4C, 0x2C, 0xC9, 0x0D,
    0x83, 0xB0, 0xB5, 0x82, 0xE4, 0xE8, 0x06, 0x9C,
    0x07, 0xA0, 0x99, 0x1D, 0x07, 0x3E, 0x82, 0x8F,

    //280
    0x70, 0x30, 0x74, 0x40, 0xCA, 0x10, 0xE4, 0xE8,
    0x0F, 0x92, 0x14, 0x3F, 0x06, 0xF8, 0x84, 0x88,
    0x43, 0x81, 0x0A, 0x34, 0x39, 0x41, 0xC6, 0xE3,
    0x1C, 0x47, 0x03, 0xB0, 0xB8, 0x13, 0x0A, 0xC2,

    //2A0
    0x64, 0xF8, 0x18, 0xF9, 0x60, 0xB3, 0xC0, 0x65,
    0x20, 0x60, 0xA6, 0x8C, 0xC3, 0x81, 0x20, 0x30,
    0x26, 0x1E, 0x1C, 0x38, 0xD3, 0x01, 0xB0, 0x26,
    0x40, 0xF4, 0x0B, 0xC3, 0x42, 0x1F, 0x85, 0x32,

    //2C0
    0x26, 0x60, 0x40, 0xC9, 0xCB, 0x01, 0xEC, 0x11,
    0x28, 0x40, 0xFA, 0x04, 0x34, 0xE0, 0x70, 0x4C,
    0x8C, 0x1D, 0x07, 0x69, 0x03, 0x16, 0xC8, 0x04,
    0x23, 0xE8, 0xC6, 0x9A, 0x0B, 0x1A, 0x03, 0xE0,

    //2E0
    0x76, 0x06, 0x05, 0xCF, 0x1E, 0xBC, 0x58, 0x31,
    0x71, 0x66, 0x00, 0xF8, 0x3F, 0x04, 0xFC, 0x0C,
    0x74, 0x27, 0x8A, 0x80, 0x71, 0xC2, 0x3A, 0x26,
    0x06, 0xC0, 0x1F, 0x05, 0x0F, 0x98, 0x40, 0xAE,

    //300 /H  (palatal)
    0x01, 0x7F, 0xC0, 0x07, 0xFF, 0x00, 0x0E, 0xFE,
    0x00, 0x03, 0xDF, 0x80, 0x03, 0xEF, 0x80, 0x1B,
    0xF1, 0xC2, 0x00, 0xE7, 0xE0, 0x18, 0xFC, 0xE0,
    0x21, 0xFC, 0x80, 0x3C, 0xFC, 0x40, 0x0E, 0x7E,

    //320
    0x00, 0x3F, 0x3E, 0x00, 0x0F, 0xFE, 0x00, 0x1F,
    0xFF, 0x00, 0x3E, 0xF0, 0x07, 0xFC, 0x00, 0x7E,
    0x10, 0x3F, 0xFF, 0x00, 0x3F, 0x38, 0x0E, 0x7C,
    0x01, 0x87, 0x0C, 0xFC, 0xC7, 0x00, 0x3E, 0x04,

    //340
    0x0F, 0x3E, 0x1F, 0x0F, 0x0F, 0x1F, 0x0F, 0x02,
    0x83, 0x87, 0xCF, 0x03, 0x87, 0x0F, 0x3F, 0xC0,
    0x07, 0x9E, 0x60, 0x3F, 0xC0, 0x03, 0xFE, 0x00,
    0x3F, 0xE0, 0x77, 0xE1, 0xC0, 0xFE, 0xE0, 0xC3,

    //360
    0xE0, 0x01, 0xDF, 0xF8, 0x03, 0x07, 0x00, 0x7E,
    0x70, 0x00, 0x7C, 0x38, 0x18, 0xFE, 0x0C, 0x1E,
    0x78, 0x1C, 0x7C, 0x3E, 0x0E, 0x1F, 0x1E, 0x1E,
    0x3E, 0x00, 0x7F, 0x83, 0x07, 0xDB, 0x87, 0x83,

    //380
    0x07, 0xC7, 0x07, 0x10, 0x71, 0xFF, 0x00, 0x3F,
    0xE2, 0x01, 0xE0, 0xC1, 0xC3, 0xE1, 0x00, 0x7F,
    0xC0, 0x05, 0xF0, 0x20, 0xF8, 0xF0, 0x70, 0xFE,
    0x78, 0x79, 0xF8, 0x02, 0x3F, 0x0C, 0x8F, 0x03,

    //3a0
    0x0F, 0x9F, 0xE0, 0xC1, 0xC7, 0x87, 0x03, 0xC3,
    0xC3, 0xB0, 0xE1, 0xE1, 0xC1, 0xE3, 0xE0, 0x71,
    0xF0, 0x00, 0xFC, 0x70, 0x7C, 0x0C, 0x3E, 0x38,
    0x0E, 0x1C, 0x70, 0xC3, 0xC7, 0x03, 0x81, 0xC1,

    //3c0
    0xC7, 0xE7, 0x00, 0x0F, 0xC7, 0x87, 0x19, 0x09,
    0xEF, 0xC4, 0x33, 0xE0, 0xC1, 0xFC, 0xF8, 0x70,
    0xF0, 0x78, 0xF8, 0xF0, 0x61, 0xC7, 0x00, 0x1F,
    0xF8, 0x01, 0x7C, 0xF8, 0xF0, 0x78, 0x70, 0x3C,

    //3e0
    0x7C, 0xCE, 0x0E, 0x21, 0x83, 0xCF, 0x08, 0x07,
    0x8F, 0x08, 0xC1, 0x87, 0x8F, 0x80, 0xC7, 0xE3,
    0x00, 0x07, 0xF8, 0xE0, 0xEF, 0x00, 0x39, 0xF7,
    0x80, 0x0E, 0xF8, 0xE1, 0xE3, 0xF8, 0x21, 0x9F,

    //400 /X  (glottal)
    0xC0, 0xFF, 0x03, 0xF8, 0x07, 0xC0, 0x1F, 0xF8,
    0xC4, 0x04, 0xFC, 0xC4, 0xC1, 0xBC, 0x87, 0xF0,
    0x0F, 0xC0, 0x7F, 0x05, 0xE0, 0x25, 0xEC, 0xC0,
    0x3E, 0x84, 0x47, 0xF0, 0x8E, 0x03, 0xF8, 0x03,

    //420
    0xFB, 0xC0, 0x19, 0xF8, 0x07, 0x9C, 0x0C, 0x17,
    0xF8, 0x07, 0xE0, 0x1F, 0xA1, 0xFC, 0x0F, 0xFC,
    0x01, 0xF0, 0x3F, 0x00, 0xFE, 0x03, 0xF0, 0x1F,
    0x00, 0xFD, 0x00, 0xFF, 0x88, 0x0D, 0xF9, 0x01,

    //440
    0xFF, 0x00, 0x70, 0x07, 0xC0, 0x3E, 0x42, 0xF3,
    0x0D, 0xC4, 0x7F, 0x80, 0xFC, 0x07, 0xF0, 0x5E,
    0xC0, 0x3F, 0x00, 0x78, 0x3F, 0x81, 0xFF, 0x01,
    0xF8, 0x01, 0xC3, 0xE8, 0x0C, 0xE4, 0x64, 0x8F,

    //460
    0xE4, 0x0F, 0xF0, 0x07, 0xF0, 0xC2, 0x1F, 0x00,
    0x7F, 0xC0, 0x6F, 0x80, 0x7E, 0x03, 0xF8, 0x07,
    0xF0, 0x3F, 0xC0, 0x78, 0x0F, 0x82, 0x07, 0xFE,
    0x22, 0x77, 0x70, 0x02, 0x76, 0x03, 0xFE, 0x00,

    //480
    0xFE, 0x67, 0x00, 0x7C, 0xC7, 0xF1, 0x8E, 0xC6,
    0x3B, 0xE0, 0x3F, 0x84, 0xF3, 0x19, 0xD8, 0x03,
    0x99, 0xFC, 0x09, 0xB8, 0x0F, 0xF8, 0x00, 0x9D,
    0x24, 0x61, 0xF9, 0x0D, 0x00, 0xFD, 0x03, 0xF0,

    //4a0
    0x1F, 0x90, 0x3F, 0x01, 0xF8, 0x1F, 0xD0, 0x0F,
    0xF8, 0x37, 0x01, 0xF8, 0x07, 0xF0, 0x0F, 0xC0,
    0x3F, 0x00, 0xFE, 0x03, 0xF8, 0x0F, 0xC0, 0x3F,
    0x00, 0xFA, 0x03, 0xF0, 0x0F, 0x80, 0xFF, 0x01,

    //4c0
    0xB8, 0x07, 0xF0, 0x01, 0xFC, 0x01, 0xBC, 0x80,
    0x13, 0x1E, 0x00, 0x7F, 0xE1, 0x40, 0x7F, 0xA0,
    0x7F, 0xB0, 0x00, 0x3F, 0xC0, 0x1F, 0xC0, 0x38,
    0x0F, 0xF0, 0x1F, 0x80, 0xFF, 0x01, 0xFC, 0x03,

    //4e0
    0xF1, 0x7E, 0x01, 0xFE, 0x01, 0xF0, 0xFF, 0x00,
    0x7F, 0xC0, 0x1D, 0x07, 0xF0, 0x0F, 0xC0, 0x7E,
    0x06, 0xE0, 0x07, 0xE0, 0x0F, 0xF8, 0x06, 0xC1,
    0xFE, 0x01, 0xFC, 0x03, 0xE0, 0x0F, 0x00, 0xFC
];

const SAMPLED_CONSONANT_VALUES_ZERO: &[u8] = &[
    0x18, 0x1A, 0x17, 0x17, 0x17
];

fn render_sample_inner(output: &mut OutputBuffer, sample_page: u16, off: u8, index1: u8, value1: u8, index0: u8, value0: u8) {
    let mut bit = 8;
    let mut sample = SAMPLE_TABLE[sample_page as usize + off as usize];

    loop {
        if sample & 128 != 0 {
            output.write(index1 as usize, value1);
        } else {
            output.write(index0 as usize, value0);
        }

        sample <<= 1;

        bit -= 1;
        if bit == 0 {
            break;
        }
    }
}

fn render_sample(output: &mut OutputBuffer, last_sample_offset: usize, consonant_flag: u8, pitch: u8) -> usize {
    // mask low three bits and subtract 1 get value to
    // convert 0 bits on unvoiced samples.
    let kind = (consonant_flag & 7) - 1;

    // determine which value to use from table { 0x18, 0x1A, 0x17, 0x17, 0x17 }
    // T', S, Z               0          0x18   coronal
    // CH', J', SH, ZH        1          0x1A   palato-alveolar
    // P', F, V, TH, DH       2          0x17   [labio]dental
    // /H                     3          0x17   palatal
    // /X                     4          0x17   glottal

    let sample_page: u16 = kind as u16 * 256; // unsigned short
    let mut off = consonant_flag & 248; // unsigned char

    if off == 0 {
        // voiced phoneme: Z*, ZH, V*, DH
        let mut phase1 = (pitch >> 4) ^ 255; // unsigned char

        off = (last_sample_offset & 0xFF) as u8; // unsigned char

        loop {
            render_sample_inner(output, sample_page, off, 3, 26, 4, 6);
            off = off.wrapping_add(1);

            let (new_phase1, overflowed) = phase1.overflowing_add(1);
            phase1 = new_phase1;
            if overflowed {
                break;
            }
        };

        return off as usize;
    }

    // unvoiced
    off ^= 255; // unsigned char

    let value0 = SAMPLED_CONSONANT_VALUES_ZERO[kind as usize]; // unsigned char

    loop {
        render_sample_inner(output, sample_page, off, 2, 5, 1, value0);

        off = off.wrapping_add(1);
        if off == 0 {
            break;
        }
    }

    last_sample_offset
}

fn sinus(x: u8) -> i8 {
    ((2.0 * std::f32::consts::PI * (x as f32 / 256.0)).sin() * 127.0) as i8
}

fn process_frames(output: &mut OutputBuffer, speed: u8, prepared_frames: &PreparedFrames) {
    let mut frame_count = prepared_frames.frame_count;
    let frames = &prepared_frames.frames;

    let mut speed_counter = speed;
    let mut phase1 = 0;
    let mut phase2 = 0;
    let mut phase3 = 0;
    let mut last_sample_offset = 0;
    let mut pos = 0;

    // These two variables are not supposed to underflow, however due to a bug in the reference
    // implementation glottal_pulse can be set to NaN, which will lock it to that value.
    let mut glottal_pulse = frames[0].pitch as isize;
    let mut mem38 = (glottal_pulse * 3) / 4;

    while frame_count > 0 {
        let flags = frames[pos].sampled_consonant_flag;

        // unvoiced sampled phoneme?
        if flags & 248 != 0 {
            last_sample_offset = render_sample(output, last_sample_offset, flags, frames[pos & 0xff].pitch);

            // skip ahead two in the phoneme buffer
            pos += 2;
            frame_count -= 2;
            speed_counter = speed;
        } else {
            {
                // Rectangle wave consisting of:
                //   0-128 = 0x90
                // 128-255 = 0x70

                // simulate the glottal pulse and formants
                let mut ary = [0_u8; 5];

                // TODO: Check if u16 is sufficient for these values
                let mut /* unsigned int */ p1: u32 = phase1 * 256; // Fixed point integers because we need to divide later on
                let mut /* unsigned int */ p2: u32 = phase2 * 256;
                let mut /* unsigned int */ p3: u32 = phase3 * 256;

                for sample in ary.iter_mut() {
                    // Sine oscillators
                    let /* signed char */ sp1 = sinus(((p1 >> 8) & 0xff) as u8);
                    let /* signed char */ sp2 = sinus(((p2 >> 8) & 0xff) as u8);

                    // Square oscillator
                    let /* signed char */ rp3: i8 = if 0xff & (p3 >> 8) < 129 {
                        -0x70
                    } else {
                        0x70
                    };

                    let /* signed int */ sin1: i32 = sp1 as i32 * (/* (unsigned char) */ frames[pos].a1 & 0x0F) as i32;
                    let /* signed int */ sin2: i32 = sp2 as i32 * (/* (unsigned char) */ frames[pos].a2 & 0x0F) as i32;
                    let /* signed int */ rect: i32 = rp3 as i32 * (/* (unsigned char) */ frames[pos].a3 & 0x0F) as i32;

                    // Sum the oscillators and convert to unsigned 8 bit audio
                    let mix = (sin1 + sin2 + rect + 4096) / 32;

                    *sample = mix as u8;

                    p1 += frames[pos].f1 as u32 * 256 / 4; // Compromise, this becomes a shift and works well
                    p2 += frames[pos].f2 as u32 * 256 / 4;
                    p3 += frames[pos].f3 as u32 * 256 / 4;
                }

                output.ary(0, ary);
            }

            speed_counter -= 1;

            if speed_counter == 0 {
                pos += 1; //go to next amplitude

                // decrement the frame count
                frame_count -= 1;

                if frame_count == 0 {
                    return;
                }

                speed_counter = speed;
            }

            glottal_pulse -= 1;

            if glottal_pulse != 0 {
                // not finished with a glottal pulse

                mem38 -= 1;

                // within the first 75% of the glottal pulse?
                // is the count non-zero and the sampled flag is zero?
                if mem38 != 0 || flags == 0 {
                    // update the phase of the formants
                    // TODO: we should have a switch to disable this, it causes a pretty nice voice without the masking!
                    phase1 += frames[pos].f1 as u32; // & 0xFF;
                    phase2 += frames[pos].f2 as u32; // & 0xFF;
                    phase3 += frames[pos].f3 as u32; // & 0xFF;

                    continue;
                }

                // voiced sampled phonemes interleave the sample with the
                // glottal pulse. The sample flag is non-zero, so render
                // the sample for the phoneme.
                last_sample_offset = render_sample(output, last_sample_offset, flags, frames[pos & 0xFF].pitch);
            }
        }

        // The reference implementation has a bug and tries to read beyond the end of the frame
        // list. In JavaScript this returns undefined, but in rust this results in a panic.
        if frame_count == 0 {
            break;
        }

        glottal_pulse = frames[pos].pitch as isize;
        if glottal_pulse > 0 {
            mem38 = (glottal_pulse * 3) / 4;
        }

        // reset the formant wave generators to keep them in
        // sync with the glottal pulse
        phase1 = 0;
        phase2 = 0;
        phase3 = 0;
    }
}

pub fn render(phonemes: &[Phoneme], pitch: u8, mouth: u8, throat: u8, speed: u8, sing_mode: bool) -> Vec<u8> {
    let prepared_frames = prepare_frames(phonemes, pitch, mouth, throat, sing_mode);

    // Create output buffer
    let mut output = OutputBuffer::new(
        (
            176.4_f32 * // 22050 / 125
            phonemes.iter().fold(0, |length, phoneme| length + phoneme.length as usize) as f32 * // Combined phoneme length in frames.
            speed as f32
        ).ceil() as usize
    );

    process_frames(&mut output, speed, &prepared_frames);

    output.get().to_vec()
}
