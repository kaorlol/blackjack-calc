mod strat_parser;

use std::{io, time::Instant};
use strat_parser::{get_action, Action, ActionError};

const CARD_ORDER: [&str; 13] = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
const FACE_CARDS: [&str; 3] = ["J", "Q", "K"];

struct Calculator {
	player_cards: Vec<String>,
	dealer_cards: Vec<String>,
}

impl Calculator {
	fn new(player_cards: Vec<String>, dealer_cards: Vec<String>) -> Self {
		Calculator { player_cards, dealer_cards }
	}

	fn convert_face_cards(&self, cards: &Vec<String>) -> Vec<String> {
		cards
			.iter()
			.map(|card| FACE_CARDS.contains(&card.as_str()).then(|| "10".to_string()).unwrap_or(card.to_string()))
			.collect()
	}

	fn order_hand(&self, cards: &Vec<String>) -> Vec<String> {
		cards
			.iter()
			.map(|card| {
				CARD_ORDER.iter().find(|c| c == &&card.as_str()).map(|c| c.to_string()).unwrap_or(card.to_string())
			})
			.collect()
	}

	fn is_ace(&self, card: &String) -> bool {
		card == "A"
	}

	fn is_pair(&self, cards: &Vec<String>) -> Option<String> {
		let ordered_cards = self.order_hand(&self.convert_face_cards(&cards));

		match ordered_cards.as_slice() {
			[first_card, second_card]
				if self.is_ace(first_card) && (self.is_ace(second_card) || is_number(second_card)) =>
			{
				Some(format!("{},{}", first_card, second_card))
			}
			[first_card, second_card]
				if is_number(first_card) && is_number(second_card) && first_card == second_card =>
			{
				Some(format!("{},{}", first_card, second_card))
			}
			_ => None,
		}
	}

	fn is_valid_hand(&self, cards: &Vec<String>) -> bool {
		cards
			.iter()
			.all(|card| is_number(card) || FACE_CARDS.contains(&card.as_str()) || self.is_ace(card))
	}

	fn is_bust(&self, cards: &Vec<String>) -> bool {
		let ordered_cards = self.order_hand(&self.convert_face_cards(&cards));
		let total = self.hand_total(&ordered_cards);

		!self.is_pair(&ordered_cards).is_some() && total > 21
	}

	fn hand_total(&self, cards: &Vec<String>) -> u8 {
		cards
			.iter()
			.map(|card| {
				if self.is_ace(card) {
					11
				} else if FACE_CARDS.contains(&card.as_str()) {
					10
				} else {
					card.parse::<u8>().unwrap()
				}
			})
			.sum()
	}

	fn suggest_action(&self) -> Result<Action, ActionError> {
		if !self.is_valid_hand(&self.player_cards) || !self.is_valid_hand(&self.dealer_cards) {
			return Err(ActionError::InvalidHand);
		}

		let player_total = self.hand_total(&self.player_cards);
		if self.is_bust(&self.player_cards) {
			return Err(ActionError::TooManyCards);
		} else if player_total == 21 {
			return Err(ActionError::Blackjack);
		}

		let dealer_total = self.hand_total(&self.dealer_cards);
		let dealer_card = self.convert_face_cards(&self.dealer_cards)[0].to_string();

		let pair = self.is_pair(&self.player_cards);

		println!("\nPlayer total: {}", player_total);
		println!("Dealer total: {}", dealer_total);

		Ok(get_action(player_total, dealer_card, pair).ok_or("Failed to get decision").unwrap())
	}
}

fn is_number(str: &str) -> bool {
	str.parse::<u8>().is_ok()
}

fn get_hand_from_input(hand_type: &str) -> Vec<String> {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input");

	input = input.trim().to_uppercase();

	match hand_type {
		"player" => input.split(",").map(|s| s.trim().to_string()).collect(),
		"dealer" => vec![input],
		_ => vec![],
	}
}

fn main() {
	println!("Enter your hand (comma-separated, e.g., '3,A'):");
	let player_hand = get_hand_from_input("player");

	println!("\nEnter dealer's card (e.g., 'Q'):");
	let dealer_hand = get_hand_from_input("dealer");

	let start = Instant::now();

	let calculator = Calculator::new(player_hand, dealer_hand);
	let action = calculator.suggest_action();
	println!("\nAction: {}\nTook {:?} to calculate action", action.unwrap().as_str(), start.elapsed());
}
