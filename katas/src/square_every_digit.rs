pub mod square_every_digit_mod{
    pub fn square_digits(num: u64) -> u64{
        //creamos un vector donde guardar los numeros elevados a la 2
        let mut vector_nums: Vec<u64> = Vec::new();

        //ejecutamos un bucle for donde pasamos num a string y a char para iterar por cada unos de ellos
        for j in num.to_string().chars(){
            //convertimos cada valor a un numero
            let h:u64 = j.to_digit(10).unwrap() as u64;

            //mandamos a vector_nums el valor de h.pow
            vector_nums.push(h.pow(2));
        }

        //convertimos cada dato a string y una collecion
        let chars_numbers:String = vector_nums.iter().map(|&g| g.to_string()).collect();

        //lo pasamos a numeros
        let pow_result:u64 = chars_numbers.parse().unwrap();

        pow_result
    }
}