/*
	Key:

	-----------------------------------------------------------------
	| H = | Hit								       					|
	| S = | Stand 								   					|
	| Dh = | Double if allowed, otherwise hit      					|
	| Ds = | Double if allowed, otherwise stand    					|
	| P = | Split 								   					|
	| Ph = | Split if double after split is allowed, otherwise hit  |
	| Rh = | Surrender if allowed, otherwise hit   					|
	| Rs = | Surrender if allowed, otherwise stand 					|
	| Rp = | Surrender if allowed and double after split            |
	|		 not allowed, otherwise split                           |
	-----------------------------------------------------------------

	Strategy:
	https://wizardofodds.com/games/blackjack/strategy/2-decks/

								   Dealers Cards:
	Your  --------------------------------------------------------------
	hand  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9  |  10  |  A  |
		  --------------------------------------------------------------
	4-8   |  H  |  H  |  H  |  H  |  H  |  H  |  H  |  H  |  H  |   H  |
	9     |  Dh |  Dh |  Dh |  Dh |  Dh |  H  |  H  |  H  |  H  |   H  |
	10    |  Dh |  Dh |  Dh |  Dh |  Dh |  Dh |  Dh |  Dh |  H  |   H  |
	11    |  Dh |  Dh |  Dh |  Dh |  Dh |  Dh |  Dh |  Dh |  Dh |   Dh |
	12    |  H  |  H  |  S  |  S  |  S  |  H  |  H  |  H  |  H  |   H  |
	13    |  S  |  S  |  S  |  S  |  S  |  H  |  H  |  H  |  H  |   H  |
	14    |  S  |  S  |  S  |  S  |  S  |  H  |  H  |  H  |  H  |   H  |
	15    |  S  |  S  |  S  |  S  |  S  |  H  |  H  |  H  |  Rh  |  Rh |
	16    |  S  |  S  |  S  |  S  |  S  |  H  |  H  |  H  |  Rh  |  Rh |
	17    |  S  |  S  |  S  |  S  |  S  |  S  |  S  |  S  |  S  |   Rs |
	18-21 |  S  |  S  |  S  |  S  |  S  |  S  |  S  |  S  |  S  |   S  |
	A,2   |  H  |  H  |  H  |  Dh |  Dh |  H  |  H  |  H  |  H  |   H  |
	A,3   |  H  |  H  |  Dh |  Dh |  Dh |  H  |  H  |  H  |  H  |   H  |
	A,4   |  H  |  H  |  Dh |  Dh |  Dh |  H  |  H  |  H  |  H  |   H  |
	A,5   |  H  |  H  |  Dh |  Dh |  Dh |  H  |  H  |  H  |  H  |   H  |
	A,6   |  H  |  Dh |  Dh |  Dh |  Dh |  H  |  H  |  H  |  H  |   H  |
	A,7   |  Ds |  Ds |  Ds |  Ds |  Ds |  S  |  S  |  H  |  H  |   H  |
	A,8   |  S  |  S  |  S  |  S  |  Ds |  S  |  S  |  S  |  S  |   S  |
	A,9-10|  S  |  S  |  S  |  S  |  S  |  S  |  S  |  S  |  S  |   S  |
	2,2   |  Ph |  Ph |  P  |  P  |  P  |  P  |  H  |  H  |  H  |   H  |
	3,3   |  Ph |  Ph |  P  |  P  |  P  |  P  |  H  |  H  |  H  |   H  |
	4,4   |  H  |  H  |  H  |  Ph |  Ph |  H  |  H  |  H  |  H  |   H  |
	6,6   |  P  |  P  |  P  |  P  |  P  |  Ph |  H  |  H  |  H  |   H  |
	7,7   |  P  |  P  |  P  |  P  |  P  |  P  |  Ph |  H  |  H  |   H  |
	8,8   |  P  |  P  |  P  |  P  |  P  |  P  |  P  |  P  |  P  |   Rp |
	9,9   |  P  |  P  |  P  |  P  |  P  |  S  |  P  |  P  |  S  |   S  |
	A,A   |  P  |  P  |  P  |  P  |  P  |  P  |  P  |  P  |  P  |   P  |
		  --------------------------------------------------------------

	If there is no row for splitting (fives and tens), then look up your hand as a hard total (10 or 20).
	If you can't split because of a limit on re-splitting, then look up your hand as a hard total, except aces.
	In the extremely unlikely event you have a pair of aces you can't re-split and drawing to split aces is allowed, then double against a 5 or 6, otherwise hit.
*/

use std::env;

struct BlackjackCalculator {
	player_cards: Vec<String>,
	dealer_cards: Vec<String>,
	player_options: [bool; 3],
}

impl BlackjackCalculator {
	fn new(player_cards: Vec<String>, dealer_cards: Vec<String>, player_options: [bool; 3]) -> Self {
		BlackjackCalculator { player_cards, dealer_cards, player_options }
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

	fn should_hit(&self, player_total: u8, dealer_total: u8) -> bool {
		match player_total {
			4..=8 => true,
			9 => dealer_total >= 3 && dealer_total <= 6,
			10 => dealer_total <= 9,
			11 => true,
			12 => dealer_total >= 4 && dealer_total <= 6,
			13..=16 => dealer_total >= 2 && dealer_total <= 6,
			17..=21 => false,
			_ => false,
		}
	}

	fn should_stand(&self, player_total: u8, dealer_total: u8) -> bool {
		match player_total {
			12 => dealer_total >= 4 && dealer_total <= 6,
			13..=16 => dealer_total >= 2 && dealer_total <= 6,
			17..=21 => true,
			_ => false,
		}
	}

	fn should_double(&self, player_total: u8, dealer_total: u8) -> bool {
		match player_total {
			9 => dealer_total >= 3 && dealer_total <= 6,
			10 => dealer_total <= 9,
			11 => true,
			_ => false,
		}
	}

	fn should_split(&self, player_total: u8, dealer_total: u8) -> bool {
		match player_total {
			4..=6 => true,
			7 => dealer_total >= 2 && dealer_total <= 7,
			8 => dealer_total <= 7,
			9 => dealer_total != 7 && dealer_total != 10 && dealer_total != 11,
			10 => false,
			11 => true,
			12 => dealer_total >= 2 && dealer_total <= 6,
			13..=16 => dealer_total >= 2 && dealer_total <= 6,
			17..=21 => false,
			_ => false,
		}
	}

	fn should_surrender(&self, player_total: u8, dealer_total: u8) -> bool {
		match player_total {
			15 => dealer_total == 10,
			16 => dealer_total == 9 || dealer_total == 10 || dealer_total == 11,
			_ => false,
		}
	}

	fn get_decision(&self) -> &'static str {
		let player_total = self.hand_total(self.player_cards.clone());
		let dealer_total = self.hand_total(self.dealer_cards.clone());

		println!("Player total: {}", player_total);
		println!("Dealer total: {}", dealer_total);

		let can_double = self.player_options[0];
		let can_split = self.player_options[1];
		let can_surrender = self.player_options[2];

		if can_surrender && self.should_surrender(player_total, dealer_total) {
			return "Surrender";
		}

		if can_split && self.should_split(player_total, dealer_total) {
			return "Split";
		}

		if can_double && self.should_double(player_total, dealer_total) {
			return "Double";
		}

		if self.should_hit(player_total, dealer_total) {
			return "Hit";
		}

		if self.should_stand(player_total, dealer_total) {
			return "Stand";
		}

		"Can't calculate decision"
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 6 {
		println!("Usage: blackjack-calc <player_hand> <dealer_hand> <can_double> <can_split> <can_surrender>");
		return;
	}

	let player_hand = args[1].split(",").map(|s| s.to_string()).collect::<Vec<String>>();
	let dealer_hand = args[2].split(",").map(|s| s.to_string()).collect::<Vec<String>>();
	let can_double: bool = args[3].parse().unwrap_or(false);
	let can_split: bool = args[4].parse().unwrap_or(false);
	let can_surrender: bool = args[5].parse().unwrap_or(false);

	let player_options = [can_double, can_split, can_surrender];
	let calculator = BlackjackCalculator::new(player_hand, dealer_hand, player_options);

	let decision = calculator.get_decision();

	println!("Decision: {}", decision);
}
