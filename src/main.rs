mod hand_utils;
mod strat_parser;

use hand_utils::{
	checks::hand_checks::{is_bust, is_pair, is_valid_hand},
	convert_face_cards, hand_total,
};
use std::{io, time::Instant};
use strat_parser::{get_action, Action, ActionError};

struct ActionCalculator {
	player_cards: Vec<String>,
	dealer_cards: Vec<String>,
}

impl ActionCalculator {
	fn new(player_cards: Vec<String>, dealer_cards: Vec<String>) -> Self {
		ActionCalculator { player_cards, dealer_cards }
	}

	fn suggest_action(&self) -> Result<Action, ActionError> {
		if !is_valid_hand(&self.player_cards) || !is_valid_hand(&self.dealer_cards) {
			return Err(ActionError::InvalidHand);
		}

		let player_total = hand_total(&self.player_cards);
		if is_bust(&self.player_cards) {
			return Err(ActionError::TooManyCards);
		} else if player_total == 21 {
			return Err(ActionError::Blackjack);
		}

		let dealer_total = hand_total(&self.dealer_cards);
		let dealer_card = convert_face_cards(&self.dealer_cards)[0].to_string();

		let pair = is_pair(&self.player_cards);

		println!("\nPlayer total: {}", player_total);
		println!("Dealer total: {}", dealer_total);

		Ok(get_action(player_total, dealer_card, pair).expect(ActionError::InvalidAction.into()))
	}
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

	let calculator = ActionCalculator::new(player_hand, dealer_hand);
	let action = calculator.suggest_action();
	println!("\nAction: {}\nTook {:?} to calculate action", action.unwrap().as_str(), start.elapsed());
}
