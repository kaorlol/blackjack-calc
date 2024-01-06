// use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;

pub const CARD_ORDER: [&str; 13] = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];

#[derive(Debug)]
pub enum Action {
	Hit,
	Stand,
	DoubleHit,
	DoubleStand,
	Split,
	SplitHit,
	SurrenderHit,
	SurrenderStand,
	SurrenderSplit,
	Blackjack,
}

impl Action {
	pub fn as_str(&self) -> &str {
		match self {
			Self::Hit => "Hit",
			Self::Stand => "Stand",
			Self::DoubleHit => "Double if allowed, otherwise hit",
			Self::DoubleStand => "Double if allowed, otherwise stand",
			Self::Split => "Split",
			Self::SplitHit => "Split if double after split is allowed, otherwise hit",
			Self::SurrenderHit => "Surrender if allowed, otherwise hit",
			Self::SurrenderStand => "Surrender if allowed, otherwise stand",
			Self::SurrenderSplit => "Surrender if allowed and double after split not allowed, otherwise split",
			Self::Blackjack => "You have blackjack!",
		}
	}

	pub fn from_str(action: &str) -> Option<Self> {
		match action {
			"Hit" => Some(Self::Hit),
			"Stand" => Some(Self::Stand),
			"Double if allowed, otherwise hit" => Some(Self::DoubleHit),
			"Double if allowed, otherwise stand" => Some(Self::DoubleStand),
			"Split" => Some(Self::Split),
			"Split if double after split is allowed, otherwise hit" => Some(Self::SplitHit),
			"Surrender if allowed, otherwise hit" => Some(Self::SurrenderHit),
			"Surrender if allowed, otherwise stand" => Some(Self::SurrenderStand),
			"Surrender if allowed and double after split not allowed, otherwise split" => Some(Self::SurrenderSplit),
			"You have blackjack!" => Some(Self::Blackjack),
			_ => None,
		}
	}
}

type Strategy = HashMap<String, DealerCard>;
type DealerCard = HashMap<String, String>; 
type PlayerCard = Option<HashMap<String, String>>;

fn get_strategy() -> Strategy {
	let strategy_json = include_str!("../strategies/2-deck-hit-soft-17.json");
	let strategy: Strategy = from_str(strategy_json).expect("Failed to parse strategy table JSON");
	strategy
}

fn find_player_card(strategy_table: Strategy, player_count: u8, pair: Option<String>) -> PlayerCard {
	if pair.is_some() {
		let found_pair = strategy_table.get(pair.as_ref()?);

		if let Some(pair_actions) = found_pair {
			println!("Found pair: {}", pair?);
			return Some(pair_actions.clone());
		}
		return None;
	}

	let found_player_card = strategy_table.get(&player_count.to_string());
	if let Some(player_actions) = found_player_card {
		return Some(player_actions.clone());
	}
	return None;
}

pub fn get_action(player_count: u8, dealer_card: String, pair: Option<String>) -> Option<Action> {
	let strategy_table = get_strategy();

	let player_card: PlayerCard = find_player_card(strategy_table, player_count, pair);
	if let Some(player_actions) = player_card {
		if let Some(action) = player_actions.get(&dealer_card) {
			return Action::from_str(action);
		}
		return None;
	}
	None
}
