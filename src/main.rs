mod strat_parser;

use std::{io, time::Instant};
use strat_parser::{suggest_action, BlackjackAction, CARD_ORDER};

struct BlackjackCalculator {
	player_cards: Vec<String>,
	dealer_cards: Vec<String>,
}

impl BlackjackCalculator {
	fn new(player_cards: Vec<String>, dealer_cards: Vec<String>) -> Self {
		BlackjackCalculator { player_cards, dealer_cards }
	}

	fn order_hand(&self, cards: Vec<String>) -> Vec<String> {
		let mut ordered_cards = vec![];

		for card in CARD_ORDER.iter() {
			for c in cards.iter() {
				if c == card {
					ordered_cards.push(c.to_string());
				}
			}
		}

		ordered_cards
	}

	fn is_ace(&self, card: &String) -> bool {
		card == "A"
	}

	fn is_pair(&self, cards: Vec<String>) -> Option<String> {
		let ordered_cards = self.order_hand(cards);

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

	fn hand_total(&self, cards: Vec<String>) -> u8 {
		let mut total = 0;
		let mut aces = 0;

		for card in cards {
			if card == "J" || card == "Q" || card == "K" {
				total += 10;
			} else if card == "A" {
				aces += 1;
			} else {
				total += card.parse::<u8>().unwrap();
			}
		}

		for _ in 0..aces {
			if total + 11 <= 21 {
				total += 11;
			} else {
				total += 1;
			}
		}

		total
	}

	fn get_action(&self) -> BlackjackAction {
		let player_total = self.hand_total(self.player_cards.clone());

		let dealer_total = self.hand_total(self.dealer_cards.clone());
		let dealer_card = self.dealer_cards[0].clone();

		let pair = self.is_pair(self.player_cards.clone());

		println!("\nPlayer total: {}", player_total);
		println!("Dealer total: {}", dealer_total);

		suggest_action(player_total, dealer_card, pair).ok_or("Failed to get decision").unwrap()
	}
}

fn is_number(str: &str) -> bool {
	str.parse::<u8>().is_ok()
}

fn get_hand_from_input(hand_type: &str) -> Vec<String> {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input");

	// input.trim().split(",").map(|s| s.trim().to_string()).collect()
	match hand_type {
		"player" => input.trim().split(",").map(|s| s.trim().to_string()).collect(),
		"dealer" => vec![input.trim().to_string()],
		_ => vec![],
	}
}

fn main() {
	println!("Enter your hand (comma-separated, e.g., '3,A'):");
	let player_hand = get_hand_from_input("player");

	println!("\nEnter dealer's card (e.g., 'Q'):");
	let dealer_hand = get_hand_from_input("dealer");

	let start = Instant::now();

	let calculator = BlackjackCalculator::new(player_hand, dealer_hand);
	let action = calculator.get_action();

	println!("\nAction: {}\nTook {:?} to calculate action", action.as_str(), start.elapsed());
}
