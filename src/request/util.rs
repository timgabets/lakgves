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

/// Generate RRN (Retrieval Referense Number)
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
    fn test_dummy_gen_rrn() {
        let rrn1 = gen_rrn();
        let rrn2 = gen_rrn();

        assert_eq!(rrn1.len(), 12);
        assert_eq!(rrn2.len(), 12);
        assert_ne!(rrn1, rrn2);
    }
}
