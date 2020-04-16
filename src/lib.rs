pub fn gen_message_id() -> i64 {
	430173293629234065
}

pub fn gen_system_id() -> String {
	String::from("PROUST")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn dummy_gen_message_id() {
		assert_eq!(gen_message_id(), 430173293629234065);
	}

	#[test]
	fn dummy_gen_system_id() {
		assert_eq!(gen_system_id(), "PROUST");
	}
}
