pub fn gen_message_id() -> i64 {
	// TODO: random generating
	430173293629234065
}

pub fn get_system_id() -> String {
	// TODO: using from config file??
	String::from("PROUST")
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
}