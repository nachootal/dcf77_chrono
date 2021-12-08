use std::io::Error;
use std::io::ErrorKind;
pub mod hour;
pub mod date;
pub mod metadata;

/// Given weights to the bits in the dcf77 bit field. Not binary.
const BIT_WEIGHTS: [u32; 8] = [1, 2, 4, 8, 10, 20, 40, 80];

/// Struct that contains the information of the section in the DCF77 bitfield
#[derive(Copy, Clone)]
pub struct SectionInBitfield {
    data_bit_mask: u64,
    data_position: u8,
    parity_mask: u64,
    max_data: u32
}

/// Checks the parity (even) of an input to the given parity
fn proof_parity(input: u64) -> bool {
    //println!("Proof parity: 0x{:X} :: {:}", input, input.count_ones()%2 == 0);
    input.count_ones()%2 == 0
}

/// Computes the value of an input with the BIT WEIGHTS of a DCF77 bitfield
fn compute_pulse(input: u64, mask: u64) -> u32 {
    let mut output: u32 = 0;
    let used_bit_weights = &BIT_WEIGHTS[0..mask.count_ones().try_into().unwrap()];
    for (index, value) in used_bit_weights.iter().rev().enumerate() {
        if input & (1 << index) > 0 {
            output += value;
        }
    }
    println!("Computed pulse: 0x{:X} - {:}", input, output);
    output
}

/// Creates a DCF77 bitfield out of the binary input given
fn create_pulse(input: u32, mask: u64) -> u32 {
    let mut aux: u32 = input;
    let mut output: u32 = 0;
    let used_bit_weights = &BIT_WEIGHTS[0..mask.count_ones().try_into().unwrap()];
    for (index, value) in used_bit_weights.iter().rev().enumerate() {
        //println!("Input to pulse: {:}={:}=0x{:X}={:}={:}", input, aux, output, index, value);
        if aux >= *value {
            output |=  1 << index;
            aux -= value;
        }
    }
    println!("Create pulse: {:}={:}=0x{:X}", input, aux, output);
    output
}

/// Generic function to code an input value in a certain position of the bitfield
pub fn code_dcf77(input: u32, section: SectionInBitfield) -> Result<u64, Error> {
    let mut coded_value:u64;
    if section.max_data >= input {
        coded_value = create_pulse(input, section.data_bit_mask).into();
        println!("Locating pulse: 0x{:X} -  {:} - 0x{:}", coded_value, section.data_position, section.data_bit_mask.count_ones());
        coded_value = coded_value << u32::from(section.data_position);
        println!("Pre Coded Value: {:} -> {:} -> 0x{:X}", input, section.data_position, coded_value);
        if 0 < section.parity_mask && !proof_parity(coded_value) {
            coded_value |= section.parity_mask;
        }
        println!("Coded Value: {:} -> {:} -> 0x{:X}", input, section.data_position, coded_value);
        Ok(coded_value)
    } else {
        let error_payload = format!("Above max! : 0x{:X} -> 0x{:X}", input, section.max_data);
        Err(Error::new(ErrorKind::InvalidData, error_payload))
    }
}

/// Generic function to decode an input value in a certain position in the bitfield that has a
/// parity check
pub fn decode_dcf77(input: u64, section: SectionInBitfield) -> Result<u32, Error> {
    let data: u64 = input & (section.data_bit_mask << section.data_position);
    let parity: u64 = input & section.parity_mask;
    println!("Decode 0x{:X} - 0x{:X} - {:} - 0x{:X} - {:}", input, section.data_bit_mask, section.data_position, section.parity_mask, 0 < parity && !proof_parity(data | parity));
    if 0 < parity && !proof_parity(data | parity) {
        let error_payload = format!("Invalid data - Input: 0x{:X} Mask: 0x{:X} Offset: {:} Parity Mask: 0x{:X}", input, section.data_bit_mask, section.data_position, section.parity_mask);
        Err(Error::new(ErrorKind::InvalidData, error_payload))
    } else {
        Ok(compute_pulse(data >> section.data_position, section.data_bit_mask))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pulses() {
        let mut mask: u64 = 0x0;
        for number_bits in 0..BIT_WEIGHTS.len() {
            mask |= 1 << number_bits;
            for value in 0..=BIT_WEIGHTS[0..=number_bits].iter().sum() {
                let output = compute_pulse(create_pulse(value, mask).into(), mask);
                assert!(value == output)
            }
        }
    }
    #[test]
    fn test_coder_decoder() {
        let mut section = SectionInBitfield {data_bit_mask: 0,
                                             data_position: 0,
                                             parity_mask: 0,
                                             max_data: 0};
        for number_bits in 0..BIT_WEIGHTS.len() {
            section.data_bit_mask |= 1 << number_bits;
            section.max_data = BIT_WEIGHTS[0..=number_bits].iter().sum();
            for value in 0..=section.max_data {
                match code_dcf77(value.into(), section) {
                    Ok(coded_value) => {
                        match decode_dcf77(coded_value, section) {
                            Ok(output) => {
                                assert!(u32::from(value) == output)
                            }
                            Err(data) => {
                                println!("{:?}", data)
                            }
                        }
                    }
                    Err(data) => {
                        println!("{:?}", data)
                    }
                }
            }
        }

    }
}
