use rand::prelude::SliceRandom;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter, Clone, Copy)]
enum Suit {
    Spade,
    Diamond,
    Club,
    Heart,
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

enum BlackjackResult {
    Winners(Vec<Player>),
}

struct Card {
    suit: Suit,
    value: Value,
    face_up: bool,
}
impl Card {
    fn flip(&mut self) {
        self.face_up = !self.face_up;
    }
}
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.face_up {
            write!(f, "Face Down")
        } else {
            write!(f, "{:?} of {:?}s", self.value, self.suit)
        }
    }
}
impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.face_up {
            write!(f, "Face Down")
        } else {
            write!(f, "{:?} of {:?}s", self.value, self.suit)
        }
    }
}

struct Deck {
    cards: Vec<Card>,
}
impl Deck {
    fn new() -> Self {
        let mut cards = Vec::new();

        // Create cards in deck in standard order
        for suit in Suit::iter() {
            match suit {
                Suit::Spade | Suit::Diamond => {
                    for value in Value::iter() {
                        cards.push(Card {
                            suit,
                            value,
                            face_up: false,
                        });
                    }
                }
                Suit::Club | Suit::Heart => {
                    for value in Value::iter().rev() {
                        cards.push(Card {
                            suit,
                            value,
                            face_up: false,
                        });
                    }
                }
            }
        }

        Self { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    cards: Vec<Card>,
}
impl Player {
    fn new(name: String) -> Self {
        let cards = Vec::new();

        Self { name, cards }
    }

    fn draw_from(&mut self, deck: &mut Deck, flip: bool) {
        let mut card = deck.draw().unwrap();
        if flip {
            card.flip();
        }
        self.cards.push(card);
    }

    fn get_sum(&self) -> i32 {
        let mut score = 0;
        let mut aces = 0;

        for card in &self.cards {
            if !card.face_up {
                continue;
            }

            match card.value {
                Value::Ace => {
                    score += 11;
                    aces += 1;
                }
                Value::Two => score += 2,
                Value::Three => score += 3,
                Value::Four => score += 4,
                Value::Five => score += 5,
                Value::Six => score += 6,
                Value::Seven => score += 7,
                Value::Eight => score += 8,
                Value::Nine => score += 9,
                Value::Ten | Value::Jack | Value::Queen | Value::King => score += 10,
            }
        }

        while score > 21 && aces > 0 {
            score -= 10;
            aces -= 1;
        }

        score
    }

    fn make_all_face_up(&mut self) {
        for card in &mut self.cards {
            if !card.face_up {
                card.flip();
            }
        }
    }
}
impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:?}, Sum: {}",
            self.name,
            self.cards,
            self.get_sum()
        )
    }
}

fn blackjack(num_players: i32) -> BlackjackResult {
    if num_players < 1 {
        panic!("Must have at least 1 player");
    } else if num_players > 4 {
        panic!("Cannot have more than 4 players");
    }

    // Initialise Deck and Players
    let mut deck = Deck::new();
    deck.shuffle();

    let mut dealer = Player::new(String::from("Dealer"));
    let mut players = Vec::new();
    for player_num in 0..num_players {
        let mut name = String::from("Player ");
        name.push_str(&(player_num + 1).to_string());
        players.push(Player::new(name));
    }

    // Draw initial cards
    for player in &mut players {
        player.draw_from(&mut deck, true);
    }
    dealer.draw_from(&mut deck, false);

    for player in &mut players {
        player.draw_from(&mut deck, true);
    }
    dealer.draw_from(&mut deck, true);

    // Player draw card logic
    println!("{}", dealer);

    let term = console::Term::stdout();

    for player in &mut players {
        loop {
            println!("{}", player);
            println!("[H]it or [S]tand");
            let c = term.read_char().unwrap();

            match c {
                'H' | 'h' => {
                    player.draw_from(&mut deck, true);

                    match player.get_sum().cmp(&21) {
                        std::cmp::Ordering::Greater => {
                            println!("{}", player);
                            println!("Bust");
                            break;
                        }

                        std::cmp::Ordering::Equal => {
                            println!("{}", player);
                            break;
                        }

                        _ => {}
                    }
                }
                'S' | 's' => break,
                _ => {}
            }
        }
    }

    // Dealer draw card logic
    println!("Dealer is playing");

    dealer.make_all_face_up();
    println!("{}", dealer);
    while dealer.get_sum() < 17 {
        println!("Dealer will hit");
        dealer.draw_from(&mut deck, true);
        println!("{}", dealer);
    }

    if dealer.get_sum() > 21 {
        println!("Dealer went bust");
    }

    let winners = players
        .into_iter()
        .filter(|p: &Player| {
            (p.get_sum() == 21 && p.cards.len() == 2)
                || (p.get_sum() <= 21 && (p.get_sum() > dealer.get_sum() || dealer.get_sum() > 21))
        })
        .collect();

    BlackjackResult::Winners(winners)
}

fn main() {
    loop {
        let term = console::Term::stdout();
        term.clear_screen().unwrap();

        match blackjack(1) {
            BlackjackResult::Winners(winners) => {
                if winners.is_empty() {
                    println!("No one won")
                } else {
                    println!("Winners: {:#?}", winners)
                }
            }
        }

        println!("Any key to start again, [Q]uit");
        let c = term.read_char().unwrap();

        match c {
            'Q' | 'q' => break,
            _ => {}
        }
    }
}
