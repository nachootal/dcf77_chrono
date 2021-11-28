use std::io::Error;
pub mod hour;
pub mod date;

/// Struct that contains the fields of information in a DCF77 bit field
pub struct DateTimeNacho {
    /// Hour in format [0..24)
    pub hour: u8,
    /// Minutes in format [0..60)
    pub minutes: u8,
    /// Day of the month in format [0..7)
    pub day: u8,
    /// Month in format [0..12]
    pub month: u8,
    /// Year in format [0..)
    pub year: u16,
    /// Day of the week in format [0..7)
    pub day_of_week: u8
}

/// Given weights to the bits in the dcf77 bit field. Not binary.
const BIT_WEIGHTS: [u16; 8] = [1, 2, 4, 8, 10, 20, 40, 80];

/// Checks the parity (even) of an input to the given parity
pub fn proof_parity(input: u64, parity: bool) -> bool {
    (input.count_ones()%2 > 0) && parity
}

/// Computes the parity of an input (even)
fn compute_parity(input: u64) -> bool {
    (input.count_ones()%2) > 0
}

/// Computes the value of an input with the BIT WEIGHTS of a DCF77 bitfield
pub fn compute_pulse(input: u16) -> u16 {
    let mut output: u16 = 0;
    for (index, value) in BIT_WEIGHTS.iter().rev().enumerate() {
        if input & (1 << index) > 0 {
            output += value;
        }
    }
    output
}

/// Creates a DCF77 bitfield out of the binary input given
fn create_pulse(input: u16) -> u16 {
    let mut aux: u16 = input;
    let mut output: u16 = 0;
    for (index, value) in BIT_WEIGHTS.iter().rev().enumerate() {
        if aux >= *value {
            output |=  1 << BIT_WEIGHTS.len() - 1 - index;
            aux -= value;
        }
    }
    output
}

/// Generic function to code an input value in a certain position of the bitfield
pub fn code_generic(input: u16, mask: u64, offset: u8, parity_mask: u64, max_value: u16) -> Result<u64, Error> {
    let mut coded_value:u64;
    if max_value > input {
        coded_value = create_pulse(input).into();
        coded_value = (coded_value << offset) & mask;
        if 0 < parity_mask && compute_parity(coded_value) {
            coded_value |= parity_mask
        }
        Ok(coded_value)
    } else {
        Err(Error::from_raw_os_error(input.into()))
    }
}

/// Generic function to decode an input value in a certain position of the bitfield
pub fn decode_generic(input: u64, mask: u64, offset: u8) -> u16 {
    let output: u16 = (input & mask >> offset).try_into().unwrap();
    compute_pulse(output)
}

/// Generic function to decode an input value in a certain position in the bitfield that has a
/// parity check
pub fn decode_generic_parity(input: u64, mask: u64, offset: u8, parity_mask:u64) -> Result<u8, Error> {
    if proof_parity(input&mask, input&parity_mask > 0) {
        let output: u8 = (input >> offset).try_into().unwrap();
        Ok(compute_pulse(output.try_into().unwrap()).try_into().unwrap())
    } else {
        // if parity not correct
        Err(Error::from_raw_os_error(0))
    }
}
