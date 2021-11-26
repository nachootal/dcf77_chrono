use std::io::Error;

/// Given weights to the bits in the dcf77 bit field. Not binary.
const BIT_WEIGHTS: [u8; 8] = [1, 2, 4, 8, 10, 20, 40, 80];

/// Checks the parity (even) of an input to the given parity
pub fn proof_parity(input: u64, parity: bool) -> bool {
    (input.count_ones()%2 > 0) && parity
}

/// Computes the parity of an input (even)
fn compute_parity(input: u64) -> bool {
    (input.count_ones()%2) > 0
}

/// Computes the value of an input with the BIT WEIGHTS of a DCF77 bitfield
pub fn compute_pulse(input: u8) -> u8 {
    let mut output: u8 = 0;
    for (index, value) in BIT_WEIGHTS.iter().rev().enumerate() {
        if input & (1 << index) > 0 {
            output += value;
        }
    }
    output
}

/// Creates a DCF77 bitfield out of the binary input given
fn create_pulse(input: u8) -> u8 {
    let mut aux: u8 = input;
    let mut output: u8 = 0;
    for (index, value) in BIT_WEIGHTS.iter().rev().enumerate() {
        if aux >= *value {
            output |=  1 << BIT_WEIGHTS.len() - 1 - index;
            aux -= value;
        }
    }
    output
}

/// Generic function to code an input value in a certain position of the bitfield
pub fn code_generic(input: u8, mask: u64, offset: u8, parity_mask: u64, max_value: u8) -> Result<u64, Error> {
    let mut coded_value:u64;
    if max_value > input {
        coded_value = create_pulse(input).into();
        coded_value = (coded_value << offset) & mask;
        if compute_parity(coded_value) {
            coded_value |= parity_mask
        }
        Ok(coded_value)
    } else {
        Err(Error::from_raw_os_error(input.into()))
    }
}
