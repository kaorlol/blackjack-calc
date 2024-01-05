mod strat_parser;

use std::{env, time::Instant};
use strat_parser::{suggest_action, BlackjackAction};

struct BlackjackCalculator {
	player_cards: Vec<String>,
	dealer_cards: Vec<String>,
}

impl BlackjackCalculator {
	fn new(player_cards: Vec<String>, dealer_cards: Vec<String>) -> Self {
		BlackjackCalculator { player_cards, dealer_cards }
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

		println!("Player total: {}", player_total);
		println!("Dealer total: {}", dealer_total);

		suggest_action(player_total, dealer_total).ok_or("Failed to get decision").unwrap()
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 3 {
		println!("Usage: blackjack-calc <player_hand> <dealer_hand>");
		return;
	}

	let start = Instant::now();

	let player_hand = args[1].split(",").map(|s| s.to_string()).collect::<Vec<String>>();
	let dealer_hand = args[2].split(",").map(|s| s.to_string()).collect::<Vec<String>>();

	let calculator = BlackjackCalculator::new(player_hand, dealer_hand);
	let action = calculator.get_action();

	println!("Action: {}\nTook {:?} to ejaculate action", action.as_str(), start.elapsed());
}
