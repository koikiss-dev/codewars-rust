pub mod vowel_count_mod {
    pub fn get_count(string: &str) -> usize {
        let mut vowel_count: usize = 0;
        //nota: podia usar filter, matches como esto
        // string.matches(|x| match x {'a'|'e'|'i'|'o'|'u' => true, _ => false}).count()
        for k in string.to_string().chars() {
            //vector de vocales para saber si en el contiene el valor de k actual

            if vec!['a', 'e', 'i', 'o', 'u'].contains(&k) {
                vowel_count += 1
            }
        }
        vowel_count
    }
}
