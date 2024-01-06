mod hand_utils;
mod strat_parser;

use hand_utils::{
	checks::hand_checks::{is_bust, is_pair, is_valid_hand},
	convert_face_cards, hand_total,
};
use std::{io, time::Instant};
use strat_parser::{get_action, Action, ActionError};

fn suggest_action(player_hand: Vec<String>, dealer_hand: Vec<String>) -> Result<Action, ActionError> {
	if !is_valid_hand(&player_hand) || !is_valid_hand(&dealer_hand) {
		return Err(ActionError::InvalidHand);
	}

	let player_total = hand_total(&player_hand);
	if is_bust(&player_hand) {
		return Err(ActionError::TooManyCards);
	} else if player_total == 21 {
		return Err(ActionError::Blackjack);
	}

	let dealer_total = hand_total(&player_hand);
	let dealer_card = convert_face_cards(&player_hand)[0].to_string();

	let pair = is_pair(&player_hand);

	println!("\nPlayer total: {}", player_total);
	println!("Dealer total: {}", dealer_total);

	Ok(get_action(player_total, dealer_card, pair).expect(ActionError::InvalidAction.into()))
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

	let action = suggest_action(player_hand, dealer_hand).expect("Failed to get action");
	println!("\nAction: {}\nTook {:?} to calculate action", action.as_str(), start.elapsed());
}
