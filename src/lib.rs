use core::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub pile: Vec<Card>,
    pub style: DeckType,
}

pub enum DeckType {
    Poker,
    Italian,
}

impl Deck {
    pub fn new(style: DeckType) -> Deck {
        Deck {
            pile: Deck::init_cards(&style),
            style,
        }
    }
    fn init_cards(style: &DeckType) -> Vec<Card> {
        let exclude: Vec<u32> = match style {
            DeckType::Italian => {
                vec![8, 9, 10]
            }
            DeckType::Poker => {
                vec![]
            }
        };

        let mut pile: Vec<Card> = vec![];
        for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for rank in 1..14 {
                if !exclude.contains(&rank) {
                    pile.push(Card::new(rank, suit.clone()))
                }
            }
        }
        pile
    }
    pub fn shuffle(&mut self) {
        self.pile.shuffle(&mut thread_rng())
    }
    pub fn draw(&mut self) -> Card {
        self.pile.pop().unwrap()
    }
}

pub struct Card {
    pub rank: u32,
    pub suit: Suit,
}

#[derive(Clone, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

pub struct Hand {
    cards: Vec<Card>,
    count: u32,
}

impl Hand {
    // This returns a Hand with at least one card
    pub fn new() -> Hand {
        Hand {
            cards: vec![],
            count: 1,
        }
    }
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
        self.count += 1;
    }
}

impl Card {
    pub fn new(rank: u32, suit: Suit) -> Card {
        Card { rank, suit }
    }

    pub fn single_card_list(&self) -> [String; 9] {
        let mut balance = " ".to_string();
        if self.rank >= 10 {
            balance = "".to_string();
        }
        [
            "┌─────────┐".to_string(),
            format!("│{}{}       │", self.rank, balance).to_string(),
            "│         │".to_string(),
            "│         │".to_string(),
            format!("│    {}    │", self.suit).to_string(),
            "│         │".to_string(),
            "│         │".to_string(),
            format!("│       {}{}│", balance, self.rank).to_string(),
            "└─────────┘".to_string(),
        ]
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Clubs => {
                write!(f, "♧")
            }
            Self::Diamonds => {
                write!(f, "♢")
            }
            Self::Hearts => {
                write!(f, "♡")
            }
            Self::Spades => {
                write!(f, "♤")
            }
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let single_card_array: [String; 9] = self.single_card_list();

        let mut card_string: String = "".to_string();

        for line in single_card_array {
            card_string.push_str(&format!("{}\n", line))
        }
        write!(f, "{}", card_string)
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This makes a vector of single card lists
        let cards_array: Vec<_> = self
            .cards
            .iter()
            .map(|card| card.single_card_list())
            .collect();

        let mut hand_string: String = "".to_string();

        // looping over cards
        for index in 0..9 {
            let padding = "  ".to_string();

            for card in &cards_array {
                hand_string.push_str(&card[index]);
                hand_string.push_str(&padding);
            }
            hand_string.push_str("\n");
        }

        write!(f, "{}", hand_string)
    }
}
