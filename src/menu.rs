pub struct Menu {
    options: Vec<String>,
}

impl Menu {
    pub fn new(o: Vec<String>) -> Menu {
        Menu {
            options: o,
        }
    }

    pub fn print_menu(&self) {
        println!("____________________________________________________________");
        let mut num: u8 = 0;
        for o in &self.options {
            num += 1;
            println!("|\t{}\t{}\t|", num, o);
        }
        println!("|\t0\t退出\t|");
        println!("____________________________________________________________");
    }
}