use crate::hand_utils::{hand_total, Card};

pub fn is_pair(cards: &Vec<String>) -> Option<String> {
	let hand = Card::convert_cards(cards);
	if hand.len() != 2 {
		return None;
	}

	let first = &hand[0];
	let second = &hand[1];

	match (first, second) {
		(Card::Ace, Card::Ace) => Some("A,A".to_string()),
		(Card::Ace, Card::Numbered(_)) => Some(format!("A,{}", second.to_string())),
		(Card::Numbered(_), Card::Ace) => Some(format!("{},A", first.to_string())),
		(Card::Numbered(a), Card::Numbered(b)) if a == b => Some(format!("{}{}", a, b)),
		_ => None,
	}
}

pub fn is_bust(cards: &Vec<String>) -> bool {
	let ordered_cards = Card::organize_hand(cards);
	let total = hand_total(&ordered_cards);

	!is_pair(&ordered_cards).is_some() && total > 21
}
