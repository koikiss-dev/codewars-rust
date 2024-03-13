pub mod calculate_bmi_mod{
    pub fn bmi(w: u32, h:f32) -> &'static str {
        let form_bmi = w as f32 / h.powf(2.0);

        /* if form_bmi <= 18.5 {
            "Underweight"
        }else if form_bmi <= 25.0 {
            "Normal"
        }else if form_bmi <= 30.0 {
            "Overweight"
        }else{
            "Obese"
        } */

        //tambien con match
        match form_bmi {
            i if i <= 18.5 => "Underweight",
            i if i <= 25.0 => "Normal",
            i if i <= 30.0 => "Overweight",
            _ => "Obese"
        }
        

        //println!("{}", form_bmi.round());
    }
}