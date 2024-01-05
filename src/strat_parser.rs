use std::collections::HashMap;

pub const CARD_ORDER: [&str; 13] = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];

#[derive(Debug)]
pub enum BlackjackAction {
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

impl BlackjackAction {
	pub fn as_str(&self) -> &str {
		match self {
			BlackjackAction::Hit => "Hit",
			BlackjackAction::Stand => "Stand",
			BlackjackAction::DoubleHit => "Double if allowed, otherwise hit",
			BlackjackAction::DoubleStand => "Double if allowed, otherwise stand",
			BlackjackAction::Split => "Split",
			BlackjackAction::SplitHit => "Split if double after split is allowed, otherwise hit",
			BlackjackAction::SurrenderHit => "Surrender if allowed, otherwise hit",
			BlackjackAction::SurrenderStand => "Surrender if allowed, otherwise stand",
			BlackjackAction::SurrenderSplit => {
				"Surrender if allowed and double after split not allowed, otherwise split"
			}
			BlackjackAction::Blackjack => "You have blackjack!",
		}
	}
}

type StrategyTable = HashMap<String, HashMap<String, String>>;
type PlayerCard = Option<HashMap<String, String>>; // Dealer card -> action

fn strategy_table() -> StrategyTable {
	let strategy_json = include_str!("../strategies/2-deck-soft-17.json");
	let strategy_table: StrategyTable =
		serde_json::from_str(strategy_json).expect("Failed to parse strategy table JSON");
	strategy_table
}

fn find_player_card(strategy_table: StrategyTable, player_count: u8, pair: Option<String>) -> PlayerCard {
	if pair.is_some() {
		let found_pair = strategy_table.get(pair.as_ref().unwrap());

		if let Some(pair_actions) = found_pair {
			println!("Found pair: {}", pair.as_ref().unwrap());
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

pub fn suggest_action(player_count: u8, dealer_card: String, pair: Option<String>) -> Option<BlackjackAction> {
	let strategy_table = strategy_table();

	let player_card = find_player_card(strategy_table, player_count, pair);
	if let Some(player_actions) = player_card {
		if let Some(action) = player_actions.get(&dealer_card) {
			match action.as_str() {
				"Hit" => Some(BlackjackAction::Hit),
				"Stand" => Some(BlackjackAction::Stand),
				"DoubleHit" => Some(BlackjackAction::DoubleHit),
				"DoubleStand" => Some(BlackjackAction::DoubleStand),
				"Split" => Some(BlackjackAction::Split),
				"SplitHit" => Some(BlackjackAction::SplitHit),
				"SurrenderHit" => Some(BlackjackAction::SurrenderHit),
				"SurrenderStand" => Some(BlackjackAction::SurrenderStand),
				"SurrenderSplit" => Some(BlackjackAction::SurrenderSplit),
				_ => None,
			}
		} else {
			None
		}
	} else {
		None
	}
}
