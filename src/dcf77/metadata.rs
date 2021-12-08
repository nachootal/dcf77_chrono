// As documented: https://www.cyber-sciences.com/wp-content/uploads/2019/01/TN-103_DCF77.pdf
/// Mask for the Antenna bit (R) in the DCF77 bit field
const ANTENNA_BIT_MASK: u64 = 1<<44;

/// Codes a given antenna bit into the DCF77 bit field
pub fn code_antenna(input: bool) -> u64 {
    if input {
        ANTENNA_BIT_MASK
    } else {
        0 as u64
    }
}

/// Extracts the antenna bit out of a dcf77 bitfield
pub fn process_antenna(input: u64) -> bool {
    0 < input & ANTENNA_BIT_MASK
}

/// Mask for the Announce bit for daylight saving time switching bit (A1) in the DCF77 bit field
const ANNOUNCE_DAYLIGHT_SAVING_SWITCHING: u64 = 1<<43;

/// Codes a given announce bit for daylight saving time switching bit into the DCF77 bit field
pub fn code_announce_daylight_saving_switching(input: bool) -> u64 {
    if input {
        ANNOUNCE_DAYLIGHT_SAVING_SWITCHING
    } else {
        0 as u64
    }
}

/// Extracts the daylight saving switch bit out of a dcf77 bitfield
pub fn process_daylight_saving_switch(input: u64) -> bool {
    0 < input & ANNOUNCE_DAYLIGHT_SAVING_SWITCHING
}

/// Mask for the daylight saving time bit (Z1) in the DCF77 bit field
const ANNOUNCE_DAYLIGHT_SAVING: u64 = 1<<42;

/// Codes a given announce daylight saving bit into the DCF77 bit field
pub fn code_daylight_saving(input: bool) -> u64 {
    if input {
        ANNOUNCE_DAYLIGHT_SAVING
    } else {
        0 as u64
    }
}

/// Extracts the daylight saving bit out of a dcf77 bitfield
pub fn process_daylight_saving(input: u64) -> bool {
    0 < input & ANNOUNCE_DAYLIGHT_SAVING
}

/// Mask for the standard time bit (Z2) in the DCF77 bit field
const ANNOUNCE_STANDARD_TIME: u64 = 1<<42;

/// Codes a given standard time bit into the DCF77 bit field
pub fn code_standard_time(input: bool) -> u64 {
    if input {
        ANNOUNCE_STANDARD_TIME
    } else {
        0 as u64
    }
}

/// Extracts the standard time bit out of a dcf77 bitfield
pub fn process_standard_time(input: u64) -> bool {
    0 < input & ANNOUNCE_STANDARD_TIME
}

/// Mask for the leap second bit (A2) in the DCF77 bit field
const ANNOUNCE_BIT_LEAP_FOR_SECOND: u64 = 1<<41;

/// Codes a given bit leap second bit into the DCF77 bit field
pub fn code_bit_leap_second(input: bool) -> u64 {
    if input {
        ANNOUNCE_BIT_LEAP_FOR_SECOND
    } else {
        0 as u64
    }
}

/// Extracts the bit leap second bit out of a dcf77 bitfield
pub fn process_bit_leap_second(input: u64) -> bool {
    0 < input & ANNOUNCE_BIT_LEAP_FOR_SECOND
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_antenna() {
        let mut dcf77_bitfield = 0x0;
        dcf77_bitfield |= code_antenna(false);
        assert!(false == process_antenna(dcf77_bitfield));
        dcf77_bitfield |= code_antenna(true);
        assert!(true == process_antenna(dcf77_bitfield));
    }
    #[test]
    fn test_announce_daylight_saving_switching() {
        let mut dcf77_bitfield = 0x0;
        dcf77_bitfield |= code_announce_daylight_saving_switching(false);
        assert!(false == process_daylight_saving_switch(dcf77_bitfield));
        dcf77_bitfield |= code_announce_daylight_saving_switching(true);
        assert!(true == process_daylight_saving_switch(dcf77_bitfield));
    }
    #[test]
    fn test_process_daylight_saving() {
        let mut dcf77_bitfield = 0x0;
        dcf77_bitfield |= code_daylight_saving(false);
        assert!(false == process_daylight_saving(dcf77_bitfield));
        dcf77_bitfield |= code_daylight_saving(true);
        assert!(true == process_daylight_saving(dcf77_bitfield));
    }
    #[test]
    fn test_standard_time() {
        let mut dcf77_bitfield = 0x0;
        dcf77_bitfield |= code_standard_time(false);
        assert!(false == process_standard_time(dcf77_bitfield));
        dcf77_bitfield |= code_standard_time(true);
        assert!(true == process_standard_time(dcf77_bitfield));
    }
    #[test]
    fn test_bit_leap_second() {
        let mut dcf77_bitfield = 0x0;
        dcf77_bitfield |= code_bit_leap_second(false);
        assert!(false == process_bit_leap_second(dcf77_bitfield));
        dcf77_bitfield |= code_bit_leap_second(true);
        assert!(true == process_bit_leap_second(dcf77_bitfield));
    }
}
