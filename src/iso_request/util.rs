extern crate chrono;
extern crate rand;

use chrono::prelude::{DateTime, Local};
use rand::Rng;

/// Generate STAN (System Trace Audit Number) - ISO 011 (N 6)
pub fn gen_stan() -> String {
    let mut rng = rand::thread_rng();
    let stan: u32 = rng.gen();
    let mut stan = stan.to_string();
    // STAN is 6 characters length, padded with zeros from the left
    if stan.len() > 6 {
        stan.truncate(6);
    } else {
        stan = format!("{:06}", stan);
    }
    stan
}

/// Get local transaction date and time represented as mmddHHMMSS - ISO 007 (N 10)
pub fn get_mmddhhmmss() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%d%m%H%M%S").to_string()
}

/// Get local transaction time represented as HHMMSS - ISO 012 (N 6)
pub fn get_hhmmss() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%H%M%S").to_string()
}

/// Get local transaction date represented as MMDD - ISO 013 (N 4)
pub fn get_mmdd() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%m%d").to_string()
}

/// Generate RRN (Retrieval Referense Number) - ISO 037 (AN 12)
pub fn gen_rrn() -> String {
    let mut rng = rand::thread_rng();
    let rrn: u64 = rng.gen();
    let mut rrn = rrn.to_string();
    // RRN is 12 characters length, padded with zeros from the left
    if rrn.len() > 12 {
        rrn.truncate(12);
    } else {
        rrn = format!("{:012}", rrn);
    }
    rrn
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_gen_stan() {
        let stan1 = gen_stan();
        let stan2 = gen_stan();

        assert_eq!(stan1.len(), 6);
        assert_eq!(stan2.len(), 6);
        assert_ne!(stan1, stan2);
    }

    #[test]
    fn test_get_hhmmss() {
        let hhmmss = get_hhmmss();
        assert_eq!(hhmmss.len(), 6);
    }

    #[test]
    fn test_get_mmdd() {
        let mmdd = get_mmdd();
        assert_eq!(mmdd.len(), 4);
    }

    #[test]
    fn test_dummy_gen_rrn() {
        let rrn1 = gen_rrn();
        let rrn2 = gen_rrn();

        assert_eq!(rrn1.len(), 12);
        assert_eq!(rrn2.len(), 12);
        assert_ne!(rrn1, rrn2);
    }
}
