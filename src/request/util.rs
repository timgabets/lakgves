extern crate rand;

use rand::Rng;

pub fn gen_message_id() -> i64 {
    // TODO: random generating
    430173293629234065
}

pub fn get_system_id() -> String {
    // TODO: using from config file??
    String::from("PROUST")
}

/// Generate STAN (System Trace Audit Number) - N 6
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

/// Generate RRN (Retrieval Referense Number) - AN 12
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
    fn gen_message_id_dummy() {
        assert_eq!(gen_message_id(), 430173293629234065);
    }
    #[test]
    fn gen_system_id_dummy() {
        assert_eq!(get_system_id(), "PROUST");
    }

    #[test]
    fn test_dummy_gen_stan() {
        let stan1 = gen_stan();
        let stan2 = gen_stan();

        assert_eq!(stan1.len(), 6);
        assert_eq!(stan2.len(), 6);
        assert_ne!(stan1, stan2);
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
