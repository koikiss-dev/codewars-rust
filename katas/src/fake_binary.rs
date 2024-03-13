pub mod fake_binary_mod{
    pub fn fake_bin(s: &str) -> String{
        let mut binary = String::new();
        
        for j in s.chars(){
            if j.to_digit(10).unwrap() /*esto convierte a un entero */ < 5 {
                binary.push('0');
            }else if j.to_digit(10).unwrap() >= 5{
                binary.push('1')
            }
        }
        binary
    }
}