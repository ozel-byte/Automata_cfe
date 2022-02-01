pub mod automata{
    const NUMBERS: &str = "123456789";
    const NUMBERS2: &str = "0123456789";
    const VALIDATE: &str = "012";

    pub struct ValidarCFE{
        pub  valid: String,
        pub  text_entry: String,
        pub  slide_entry: String,
        pub  date_paid: String,
        pub  amount_paid: String,
        pub  service_number: String
    }

    impl ValidarCFE {
        pub fn estado_0(&mut self){
            self.valid = String::from("2");
            if self.text_entry.len() == 30 && !self.text_entry.contains(" ") {
                if self.slide_entry[0..1].trim() == "0"{
                    self.slide_entry = self.slide_entry[1..].to_string();
                    self.estado_1();
                }
            } else if self.slide_entry.len() == 0 {
                self.valid = String::from("0");
            }
        }
        
    }

    trait E1{
        fn estado_1(&mut self);
    }

    impl E1 for ValidarCFE{
        fn estado_1(&mut self){
            if self.slide_entry[0..1].trim() == "1" {
                self.slide_entry = self.slide_entry[1..].to_string();
                self.estado_2();
            }
        }
    }

    trait E2{
        fn estado_2(&mut self);
    }

    impl E2 for ValidarCFE{
        fn estado_2(&mut self) {
            let self_number: Result<i64,_> = self.slide_entry[..12].trim().parse();
            match self_number {
                Ok(_v) => {
                    if  self.slide_entry[..12].to_string() != "000000000000" {
                        self.slide_entry = self.slide_entry[12..].to_string();
                        self.estado_3();
                    }
                },
                Err(_e) => {
                    println!("No es un numero valido lo siento :(")
                }
            }
 
        }
    }

    trait E3{
        fn estado_3(&mut self);
    }

    impl E3 for ValidarCFE{
        fn estado_3(&mut self) {
            if self.slide_entry[..2].to_string() == "00" || ((self.slide_entry[..2].parse::<i32>().unwrap()) >=1 && self.slide_entry[..2].parse::<i32>().unwrap()<=22 ){
                self.slide_entry = self.slide_entry[2..].to_string();
                self.estado_4();
            }
        }
    }

    trait E4{
        fn estado_4(&mut self);
    }

    impl E4 for ValidarCFE{
        fn estado_4(&mut self) {
            if self.slide_entry[..2].to_string() == "02"{
                self.slide_entry = self.slide_entry[2..].to_string();
                self.estado_5(true);
            } else if (self.slide_entry[..1].to_string() == "0" && NUMBERS.contains(self.slide_entry[1..2].trim())) || (self.slide_entry[..1].to_string() == "1" && VALIDATE.contains(self.slide_entry[1..2].trim())){ 
                self.slide_entry = self.slide_entry[2..].to_string();
                self.estado_5(false);
            }
        }
    }

    trait E5{
        fn estado_5(&mut self, feb: bool);
    }

    impl E5 for ValidarCFE{
        fn estado_5(&mut self, feb: bool) {

            if !feb && self.slide_entry[..2].parse::<i32>().unwrap() >=1 && self.slide_entry[..2].parse::<i32>().unwrap() <= 31{
                println!("Estado 5: {}", self.slide_entry); 
                self.slide_entry = self.slide_entry[2..].to_string();
                self.estado_6();
            } else if self.slide_entry[..2].parse::<i32>().unwrap() >=1 && self.slide_entry[..2].parse::<i32>().unwrap() <= 29 {
                println!("Estado 5Feb: {}", self.slide_entry); 
                self.slide_entry = self.slide_entry[2..].to_string();
                self.estado_6();
            }
            
        }
    }

    
    trait E6{
        fn estado_6(&mut self);
    }

    impl E6 for ValidarCFE{
        fn estado_6(&mut self) {
            let mut bandera = 0;
            for item in 0..9{
                if NUMBERS2.contains(self.slide_entry[item..item+1].trim()){
                    bandera += 1;
                }
            }
            if bandera == 9{
                self.slide_entry = self.slide_entry[9..].to_string();
                self.estado_7();
            }
        }
    }

    trait E7{
        fn estado_7(&mut self);
    }

    impl E7 for ValidarCFE{
        fn estado_7(&mut self) {
            if NUMBERS2.contains(self.slide_entry[..1].trim()){
                self.estado_8();
            }
        }
    }

    trait E8{
        fn estado_8(&mut self);
    }

    impl E8 for ValidarCFE{
        fn estado_8(&mut self) {

            self.service_number = self.text_entry[2..14].to_string();
            //self.date_paid = self.text_entry[14..20].to_string();
            self.date_paid = self.text_entry[14..16].to_string() + "-" + &self.text_entry[16..18].to_string() + "-" + &self.text_entry[18..20].to_string();
            self.amount_paid = "$".to_string() + &(self.text_entry[20..29].parse::<i32>().unwrap()).to_string();

            self.valid = String::from("1");
            println!("numero de recibo: {}", self.text_entry);
            println!("Numero de servicio: {}", self.service_number);
            println!("Fecha: {}", self.date_paid);
            println!("Importe: {}", self.amount_paid);
        }
    }
}