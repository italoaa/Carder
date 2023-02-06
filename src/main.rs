use carder::{self, Card, Hand, Suit};

fn main() {
    let test_card: Card = Card::new(2, Suit::Diamonds);
    let mut test_hand: Hand = Hand::new(2, Suit::Spades);
    test_hand.add_card(test_card);
    // println!("{}", test_card);
    println!("{}", test_hand);
}
