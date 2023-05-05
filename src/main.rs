mod ledger;

use crate::ledger::Ledger;
use std::error::Error;

macro_rules! input {
    ($input_buf:ident << $prompt:expr) => {
        let mut $input_buf = String::new();
        {
            use std::io::Write;
            print!("{}", $prompt);
            std::io::stdout().flush().unwrap();

            let stdin = std::io::stdin();
            stdin.read_line(&mut $input_buf).unwrap();
        }
        let $input_buf = $input_buf.trim();
    }
}

struct LedgerUI {
    ledger: Ledger,
}

impl LedgerUI {
    const HELP: [(&'static str, &'static str); 5] = [
        ("/<search>", "search for people"),
        ("a", "show all totals"),
        ("A", "Add entry"),
        ("?", "Help"),
        ("q", "Quit"),
    ];

    fn show_help(&self) {
        println!("Choices:");
        for (cmd, help) in LedgerUI::HELP {
            println!("{:#30}:{:}", cmd, help);
        }
    }
    
    pub fn from_path(path: impl AsRef<std::path::Path>)
        -> Result<Self, Box<dyn Error>> {
        let ledger = Ledger::from_path(path)?;
        Ok(Self { ledger })
    }

    pub fn show_all(&self) {
        self.show_net_credits(self.ledger.totals_iter());
    }

    pub fn add_entry(&mut self) {
        input!(name << "Name: ");
        input!(credit << "Credit: ");
        self.ledger.add_entry(name, credit);
    }

    pub fn show_net_credits<'a>(
        &self,
        total_hash: impl Iterator<Item = (&'a String, &'a i32)>) {
        println!("{:=>53}", "");
        println!("Sn.\t{:#30} \tTotal Credits", "Name");

        for (serial_no, (person, total_credits)) in total_hash.enumerate() {
            println!("{}:\t{:#30}:\t{}", serial_no, person, total_credits);
        };
        
        println!("{:=>53}", "");
    }

    pub fn search(&self, query: &str) -> Result<(), Box<dyn Error>> {
        let results = self.ledger.search_totals(query);

        self.show_net_credits(results.clone());

        /*
        input!(choice << "Choose: ");
        let choice = choice.parse::<u32>()?;

        self.show_net_credits(results.skip(choice as usize).take(1));
        */
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = LedgerUI::from_path("ledger.csv")?;

    ui.show_help();

    loop {
        input!(cmd << "> ");

        match cmd {
            "?" => ui.show_help(),
            "q" => break,
            "a" => ui.show_all(),
            "A" => ui.add_entry(),
            _ if cmd.starts_with("/") => ui.search(&cmd[1..])?,
            _ => eprintln!("Unknown command, try again!"),
        };

        println!("");
    }

    Ok(())
}

