use std::collections::HashMap;

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
		}
	}
}

fn strategy_table() -> HashMap<String, HashMap<String, String>> {
	let strategy_json = include_str!("../strategies/2-deck-soft-17.json");
	let strategy_table: HashMap<String, HashMap<String, String>> =
		serde_json::from_str(strategy_json).expect("Failed to parse strategy table JSON");
	strategy_table
}

pub fn suggest_action(player_count: u8, dealer_count: u8) -> Option<BlackjackAction> {
	let strategy_table = strategy_table();

	if let Some(player_actions) = strategy_table.get(&player_count.to_string()) {
		if let Some(action) = player_actions.get(&dealer_count.to_string()) {
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
