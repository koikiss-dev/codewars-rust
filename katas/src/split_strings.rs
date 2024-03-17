pub mod split_string_mod{
    pub fn solution(s: &str) -> Vec<String> {
        /* let mut g = Vec::new();
        let mut chars = s.chars(); */
        s.chars().collect::<Vec<_>>().chunks(2).map(|f| {
            if f.len() == 1{
                format!("{}_", f[0])

            }else{
                f.into_iter().collect()
            }
        }).collect()
       /*  while let Some(i) = chars.next(){
            let second = match chars.next() {
                Some(c) => c,
                _ => '_'
            };
            g.push(format!("{}{}", i, second));
        }
        g */
    }
}