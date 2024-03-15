pub mod personalized_message_mod{
    pub fn greet(name: &str, owner: &str) -> String{
        //let g = owner.contains("Daniel");

        match name.to_ascii_uppercase() == owner.to_uppercase() {
            true => String::from("Hello boss"),
            _ => String::from("Hello guest")
        }
    }
}