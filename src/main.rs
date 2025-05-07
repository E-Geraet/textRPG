use std::ffi::CString;
use std::io::{self, Write};
use rand::Rng;
use rand::seq::SliceRandom;


//main loop
fn main() {
    println!("Willkommen im Roguelike-Spiel!");

    let räume = neue_welt();
    let mut aktueller_raum_index = 0;
    let mut eingabe = String::new();

    loop {
        // Raum beschreiben
        beschreibe_raum(&räume, aktueller_raum_index);

        // Eingabe holen
        eingabe.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut eingabe).unwrap();
        let eingabe = eingabe.trim().to_lowercase();

        // Eingabe verarbeiten
        match eingabe.as_str() {
            "nord" | "n" => {
                if let Some(neuer_raum) = räume[aktueller_raum_index].nord {
                    aktueller_raum_index = neuer_raum;
                    println!("Du gehst nach Norden.");
                } else {
                    println!("Du kannst nicht nach Norden gehen.");
                }
            },
            "süd" | "s" => {
                if let Some(neuer_raum) = räume[aktueller_raum_index].süd {
                    aktueller_raum_index = neuer_raum;
                    println!("Du gehst nach Süden.");
                } else {
                    println!("Du kannst nicht nach Süden gehen.");
                }
            },
            // Weitere Richtungen...
            "exit" => {
                println!("Das Spiel wird beendet.");
                break;
            },
            _ => println!("Unbekannter Befehl."),
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
    Stark,
    Intelligent,
    Charismatisch,
    Flink,
    Ausdauernd,
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
// Raumstruktur
struct Room {
    name: String,
    inhalt: Vec<String>,
    nord: Option<usize>,
    west: Option<usize>,
    ost: Option<usize>,
    süd: Option<usize>,
}

// Implementierung für Room
impl Room {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            inhalt: Vec::new(),
            nord: None,
            west: None,
            ost: None,
            süd: None,
        }
    }
}

// Eigenständige Funktionen (außerhalb des impl Room-Blocks)
fn neue_welt() -> Vec<Room> {
    let mut räume = Vec::new();

    // Random Räume
    räume.push(Room::new("Anfang"));
    räume.push(Room::new("Schloss"));
    räume.push(Room::new("Geheimraum"));

    // Räume verbinden
    verbinde_räume(&mut räume, 1, 2, "ost");
    verbinde_räume(&mut räume, 0, 1, "nord");

    räume
}

fn verbinde_räume(räume: &mut Vec<Room>, raum1_index: usize, raum2_index: usize, richtung: &str) {
    match richtung.to_lowercase().as_str() {
        "nord" => {
            räume[raum1_index].nord = Some(raum2_index);
            räume[raum2_index].süd = Some(raum1_index);
        },
        "süd" => {
            räume[raum1_index].süd = Some(raum2_index);
            räume[raum2_index].nord = Some(raum1_index);
        },
        "ost" => {
            räume[raum1_index].ost = Some(raum2_index);
            räume[raum2_index].west = Some(raum1_index);
        },
        "west" => {
            räume[raum1_index].west = Some(raum2_index);
            räume[raum2_index].ost = Some(raum1_index);
        },
        _ => println!("Richtung ungültig"),
    }
}

fn beschreibe_raum(räume: &Vec<Room>, raum_index: usize) {
    let raum = &räume[raum_index];

    println!("===== {} =====", raum.name);
    println!("Ausgänge:");
    if raum.nord.is_some() { println!("- Nach Norden"); }
    if raum.süd.is_some() { println!("- Nach Süden"); }
    if raum.ost.is_some() { println!("- Nach Osten"); }
    if raum.west.is_some() { println!("- Nach Westen"); }

    println!("");
}