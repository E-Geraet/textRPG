use std::io::{self, Write};
use rand::Rng;
use rand::seq::SliceRandom;


//main loop
fn main() {
    println!("Willkommen im Roguelike-Spiel!");

    let räume = neue_welt();
    let mut aktueller_raum_index = 0;
    let mut eingabe = String::new();
    let mut user = Charakter::create_charakter("Spieler"); // Korrektur des Funktionsnamens

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
            "nord" | "n" | "norden" => {
                if let Some(neuer_raum) = räume[aktueller_raum_index].nord {
                    aktueller_raum_index = neuer_raum;
                    println!("Du gehst nach Norden.");

                    // Zufallschance für ein Ereignis (20%)
                    let mut rng = rand::thread_rng();
                    if rng.gen_range(1..=100) <= 20 {
                        let _event = random_events(&mut user);
                    }
                } else {
                    println!("Du kannst nicht nach Norden gehen.");
                }
            },
            "süd" | "s" | "süden" => {
                if let Some(neuer_raum) = räume[aktueller_raum_index].süd {
                    aktueller_raum_index = neuer_raum;
                    println!("Du gehst nach Süden.");

                    // Zufallschance für ein Ereignis
                    let mut rng = rand::thread_rng();
                    if rng.gen_range(1..=100) <= 20 {
                        let _event = random_events(&mut user);
                    }
                } else {
                    println!("Du kannst nicht nach Süden gehen.");
                }
            },
            "ost" | "o" | "osten" => {
                if let Some(neuer_raum) = räume[aktueller_raum_index].ost {
                    aktueller_raum_index = neuer_raum;
                    println!("Du gehst nach Osten.");

                    // Zufallschance für ein Ereignis
                    let mut rng = rand::thread_rng();
                    if rng.gen_range(1..=100) <= 20 {
                        let _event = random_events(&mut user);
                    }
                } else {
                    println!("Du kannst nicht nach Osten gehen.");
                }
            },
            "west" | "w" | "westen" => {
                if let Some(neuer_raum) = räume[aktueller_raum_index].west {
                    aktueller_raum_index = neuer_raum;
                    println!("Du gehst nach Westen.");

                    // Zufallschance für ein Ereignis
                    let mut rng = rand::thread_rng();
                    if rng.gen_range(1..=100) <= 20 {
                        let _event = random_events(&mut user);
                    }
                } else {
                    println!("Du kannst nicht nach Westen gehen.");
                }
            },
            "status" => {
                println!("===== Charakter-Status =====");
                println!("Name: {}", user.name);
                println!("Gesundheit: {}", user.gesundheit);
                println!("Hunger: {}", user.hunger);
                println!("Aggression: {}", user.aggression);
                println!("Intelligenz: {}", user.intelligenz);
                println!("Besonderheiten: {:?}", user.besonderheiten);
                println!("==========================");
            },
            "hilfe" => {
                println!("Verfügbare Befehle:");
                println!("  nord/n/norden - Nach Norden gehen");
                println!("  süd/s/süden - Nach Süden gehen");
                println!("  ost/o/osten - Nach Osten gehen");
                println!("  west/w/westen - Nach Westen gehen");
                println!("  status - Charakterstatus anzeigen");
                println!("  hilfe - Diese Hilfe anzeigen");
                println!("  exit/beenden/verlassen - Spiel beenden");
            },
            "exit" | "beenden" | "verlassen" => {
                println!("Das Spiel wird beendet.");
                break;
            },
            _ => println!("Unbekannter Befehl."),
        }
    }
}


//Hauptschleife zu ende


struct Charakter {
    name: String,
    gesundheit: i32,
    aggression: i32,
    hunger: i32,
    intelligenz: i32,
    besonderheiten: Vec<Trait>,
}


#[derive(Clone, Debug)]
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

struct Events {
    name: String,
    beschreibung: String,
    effekt_typ: Effekte,
    effekt_wert: i32,
}


enum Effekte {
    Gesundheit,
    Hunger,
    Aggression,
    Intelligenz,
    // weitere effekte
}


fn random_events(charakter: &mut Charakter) -> Events {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..5);

    let event = match random_number {
        1 => Events {
            name: "Heilende Quelle".to_string(),
            beschreibung: "Du findest eine heilende Quelle und trinkst daraus.".to_string(),
            effekt_typ: Effekte::Gesundheit,
            effekt_wert: 10,
        },
        2 => Events {
            name: "Hungriger Wolf".to_string(),
            beschreibung: "Ein hungriger Wolf greift dich an!".to_string(),
            effekt_typ: Effekte::Gesundheit,
            effekt_wert: -10,
        },
        3 => Events {
            name: "Alte Frucht".to_string(),
            beschreibung: "Du findest eine alte Frucht und isst sie.".to_string(),
            effekt_typ: Effekte::Hunger,
            effekt_wert: -5,
        },
        _ => Events {
            name: "Seltsames Geräusch".to_string(),
            beschreibung: "Du hörst ein seltsames Geräusch, aber nichts passiert.".to_string(),
            effekt_typ: Effekte::Gesundheit,
            effekt_wert: 0,
        },
    };

    println!("{}", event.beschreibung);

    event
}
