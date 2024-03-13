pub mod even_or_odd_mod{
    pub fn print_event_or_odd(number: i32) -> &'static str{
        //The match operator takes in a variable and compares its value with each case value. If there is a match, the corresponding block of code is executed.
        //es como el switch de javascript, el _ es el default o sea que no hay match con nada
        match number % 2 {
            0 => "Even",
            _ => "Odd"
        };
        if number % 2 == 0{
            "Even"
        }else{
            "Odd"
        }
    }
}