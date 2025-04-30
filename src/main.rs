use std::ffi::CString;
use std::io::{self, Write};
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    println!("Hallo, User!");
    let mut eingabe = String::new();

    loop {
        eingabe.clear();

        print!("Gehst du in den Norden oder in den Süden Nord/Süd: ");
        // WICHTIG: stdout flushen, damit die Ausgabe direkt erscheint
        io::stdout().flush().unwrap();


        io::stdin().read_line(&mut eingabe).unwrap();
        let eingabe = eingabe.trim(); // entfernt Zeilenumbruch etc.

        if eingabe == "Süd" {
            println!("Du hast Süd eingegeben");
        } else if eingabe == "Nord" {
            println!("Du hast Nord eingegeben");
        } else if eingabe == "exit" {
            println!("Das Programm wird beendet.");
            break;
        } else {
            println!("Unbekannte Eingabe. Versuche Süd, Nord oder exit.");
        }
    }
}

struct Charakter {
    name: String,
    gesundheit: i32,
    aggression: i32,
    hunger: i32,
    intelligenz: i32,
    besonderheiten: Vec<Trait>,
}


enum Trait {
    stark,
    intelligent,
    charismatisch,
    flink,
    ausdauernd,
    // und so weiter, alles platzhalter bisher
}
impl Charakter {
    fn create_charakter(name :&str) -> Charakter {
        let mut rng = rand::thread_rng();

        let gesundheit = rng.gen_range(1..100);
        let aggression = rng.gen_range(1..10);
        let hunger = rng.gen_range(1..10);
        let intelligenz = rng.gen_range(1..10);

        let mut traits = vec![Trait::Stark, Trait::Intelligent, Trait::Charismatisch,
                              Trait::Flink, Trait::Ausdauernd];

        traits.shuffle(&mut rng);

        let besonderheiten = traits[0..2].to_vec();


        Charakter {
            name: name.to_string(),
            gesundheit,
            aggression,
            hunger,
            intelligenz,
            besonderheiten,
        }
    }


}


//Worldbulding
struct Room {
        name: String,
        inhalt, Vec<string>//keine ahnung welcher daten typ hier sinn macht
        nord: Option<usize>,
        west: Option<usize>,
        ost: Option<usize>,
        süd: Option<usize>,

}

impl Room {
    fn new(name: &str) -> Self {}
    name: name.to_string(),
    inhalt: Vec::new(),
    nord: None,
    west: None,
    ost: None,
    süd: None,
}