mod ticket {

	pub struct  item {
		name:		String,
		operator:	String,
		EAN: u64,
	}

	impl	item {
		pub	fn	new_ticket(name: String, operator: String, EAN: u64) -> item {

		}
		// GETTERS FUNCTIONS
		pub	fn	get_name(&self) -> &str {
			&self.name
		}
		pub	fn	get_operator(&self) -> &str {
			&self.name
		}
		pub	fn	get_EAN(&self) -> u64 {
			&self.EAN
		}
		// SETTERS FUNCTIONS TO COMBINE WITH NEW
		pub	fn	set_name(<user input>) -> <update_struct> {
			//update data in struct
			//implement for each field
		}
	}
}

//create modules for positions in warehouse
//	use a real-life method, investigate a modern one

//create code ti generate a 1000x900px ticket
//	contain EAN
//	date and time
//	item name and quick specs

//maybe think in a friendly approach for the use of tickets

/*mod helper_functions {
	use	super::etiqueta;

	fn	validate_op(user: String, PDT: String) -> bool {
		!user.is_empty() {
			return "User Not valid"
		}
		PDT.is_empty() {
		PDT.is_empty()
		}
	}
}*/

fn	main()
