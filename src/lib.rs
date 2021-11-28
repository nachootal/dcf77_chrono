use std::io::Error;
use crate::dcf77::*;
mod dcf77;

/// Decodes the hour and minutes out of a dcf77 bit field
pub fn from_dcf77(input: u64) -> Result<DateTimeNacho, Error> {
    let processed_hour = dcf77::hour::process_hour(input)?;
    let processed_minutes = dcf77::hour::process_minutes(input)?;
    let processed_day = dcf77::date::process_day(input);
    let processed_day_of_week = dcf77::date::process_day_of_week(input);
    let processed_month = dcf77::date::process_month(input);
    let processed_year = dcf77::date::process_year(input);
    Ok(DateTimeNacho{hour: processed_hour, minutes: processed_minutes, day: processed_day, month: processed_month, year: processed_year, day_of_week: processed_day_of_week})
}

/// Produces a bit field with the given hour and minutes
pub fn to_dcf77(hour: u8, minutes: u8, day: u8, day_of_week: u8, month: u8, year: u16) -> Result<u64, Error> {
    let coded_hour = dcf77::hour::code_hour(hour)?;
    let coded_minutes = dcf77::hour::code_minutes(minutes)?;
    let coded_day = dcf77::date::code_day(day)?;
    let coded_day_of_week = dcf77::date::code_day_of_the_week(day_of_week)?;
    let coded_month = dcf77::date::code_month(month)?;
    let coded_year = dcf77::date::code_year(year)?;
    Ok(coded_hour | coded_minutes | coded_day | coded_day_of_week | coded_month | coded_year)
}
