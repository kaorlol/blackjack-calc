use std::cmp::Ordering;

pub mod checks;

pub enum Card {
	Numbered(u8),
	Face,
	Ace,
}

impl Card {
	pub fn to_string(&self) -> String {
		match self {
			Card::Numbered(value) => value.to_string(),
			Card::Face => "10".to_string(),
			Card::Ace => "A".to_string(),
		}
	}

	pub fn convert_cards(cards: &Vec<String>) -> Vec<Card> {
		cards
			.iter()
			.map(|card| match card.as_str() {
				"A" => Card::Ace,
				"J" | "Q" | "K" => Card::Face,
				_ => Card::Numbered(card.parse::<u8>().unwrap()),
			})
			.collect()
	}

	pub fn is_valid_hand(cards: &Vec<String>) -> bool {
		let hand = Card::convert_cards(cards);
		hand.iter().all(|card| match card {
			Card::Numbered(value) => *value >= 2 && *value <= 10,
			Card::Face | Card::Ace => true,
		})
	}

	pub fn organize_hand(cards: &Vec<String>) -> Vec<String> {
		let mut hand = Card::convert_cards(cards);
		hand.sort_by(|a, b| match (a, b) {
			(Card::Numbered(a), Card::Numbered(b)) => a.cmp(b),
			(Card::Numbered(_), _) => Ordering::Less,
			(_, Card::Numbered(_)) => Ordering::Greater,
			_ => Ordering::Equal,
		});
		hand.iter().map(|card| card.to_string()).collect()
	}
}

// pub fn covert_hand_to_cards(hand: &Vec<String>) -> Vec<Card> {
// 	hand.iter()
// 		.map(|card| match card.as_str() {
// 			"A" => Card::Ace,
// 			"J" | "Q" | "K" => Card::Face,
// 			_ => Card::Numbered(card.parse::<u8>().unwrap()),
// 		})
// 		.collect()
// }

pub fn hand_total(hand: &Vec<String>) -> u8 {
	let hand = Card::convert_cards(hand);
	let mut total = 0;
	let mut ace_count = 0;

	for card in hand {
		match card {
			Card::Numbered(value) => total += value,
			Card::Face | Card::Ace => {
				total += 10;
				if let Card::Ace = card {
					ace_count += 1;
				}
			}
		}
	}

	// Adjust for Aces
	while ace_count > 0 && total > 21 {
		total -= 10;
		ace_count -= 1;
	}

	total
}
