// As documented: https://www.cyber-sciences.com/wp-content/uploads/2019/01/TN-103_DCF77.pdf
/// Mask for the Antenna bit (R) in the DCF77 bit field
const ANTENNA_BIT_MASK: u64 = 1<<44;

/// Mask for the Announce bit for daylight saving time switching bit (A1) in the DCF77 bit field
const ANNOUNCE_DAYLIGHT_SAVING_SWITCHING: u64 = 1<<43;

/// Mask for the daylight saving time bit (Z1) in the DCF77 bit field
const ANNOUNCE_DAYLIGHT_SAVING: u64 = 1<<42;

/// Mask for the standard time bit (Z2) in the DCF77 bit field
const ANNOUNCE_STANDARD_TIME: u64 = 1<<42;

/// Mask for the leap second bit (A2) in the DCF77 bit field
const ANNOUNCE_BIT_LEAP_FOR_SECOND: u64 = 1<<41;

/// Codes a given antenna bit into the DCF77 bit field
pub fn code_antenna(input: bool) -> u64 {
    if input {
        1<<ANTENNA_BIT_MASK as u64
    } else {
        0 as u64
    }
}

/// Codes a given announce bit for daylight saving time switching bit into the DCF77 bit field
pub fn code_announce_daylight_saving_switching(input: bool) -> u64 {
    if input {
        1<<ANNOUNCE_DAYLIGHT_SAVING_SWITCHING as u64
    } else {
        0 as u64
    }
}

/// Codes a given announce daylight saving bit into the DCF77 bit field
pub fn code_daylight_saving(input: bool) -> u64 {
    if input {
    1<<ANNOUNCE_DAYLIGHT_SAVING as u64
    } else {
        0 as u64
    }
}

/// Codes a given standard time bit into the DCF77 bit field
pub fn code_standard_time(input: bool) -> u64 {
    if input {
    1<<ANNOUNCE_STANDARD_TIME as u64
    } else {
        0 as u64
    }
}

/// Codes a given bit leap second bit into the DCF77 bit field
pub fn code_bit_leap_second(input: bool) -> u64 {
    if input {
        1<<ANNOUNCE_BIT_LEAP_FOR_SECOND as u64
    } else {
        0 as u64
    }
}

/// Extracts the antenna bit out of a dcf77 bitfield
pub fn process_antenna(input: u64) -> bool {
    0 < input & (1<<ANTENNA_BIT_MASK)
}

/// Extracts the daylight saving switch bit out of a dcf77 bitfield
pub fn process_daylight_saving_switch(input: u64) -> bool {
    0 < input & (1<<ANNOUNCE_DAYLIGHT_SAVING_SWITCHING)
}

/// Extracts the daylight saving bit out of a dcf77 bitfield
pub fn process_daylight_saving(input: u64) -> bool {
    0 < input & (1<<ANNOUNCE_DAYLIGHT_SAVING)
}

/// Extracts the standard time bit out of a dcf77 bitfield
pub fn process_standard_time(input: u64) -> bool {
    0 < input & (1<<ANNOUNCE_STANDARD_TIME)
}

/// Extracts the bit leap second bit out of a dcf77 bitfield
pub fn process_bit_leap_second(input: u64) -> bool {
    0 < input & (1<<ANNOUNCE_BIT_LEAP_FOR_SECOND)
}
