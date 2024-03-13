pub mod string_repeat_mod{
    pub fn repeat_str(src: &str, count: usize) -> String {
        //creamos un string vacio, nuevo y muteable
        let mut r = String::new();

        //con el for recorremos hasta llegar al count y por cada iteracion hacemos un push de strings a nuestra variable r agregando el src
        for _ in 0..count{
            r.push_str(src)
        }

        //tambien esta esta opcion 
        //src.repeat(count);
        r
    }
}