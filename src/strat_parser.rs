use serde_json::from_str;
use std::{collections::HashMap, str::FromStr};
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, EnumString, Display)]
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
}

#[derive(Debug)]
pub enum ActionError {
	InvalidAction,
	TooManyCards,
	InvalidHand,
	Blackjack,
}

impl Into<&'static str> for ActionError {
	fn into(self) -> &'static str {
		match self {
			ActionError::InvalidAction => "Invalid action",
			ActionError::TooManyCards => "Too many cards",
			ActionError::InvalidHand => "Invalid hand",
			ActionError::Blackjack => "You have blackjack!",
		}
	}
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
		}
	}
}

type Strategy = HashMap<String, HashMap<String, String>>;
type PlayerCard<'a> = Option<&'a HashMap<String, String>>;

fn get_strategy() -> Strategy {
	let strategy_json = include_str!("../strategies/2-deck-hit-soft-17.json");
	from_str(strategy_json).expect("Failed to parse strategy table JSON")
}

fn find_player_card<'a>(strategy_table: &'a Strategy, player_count: u8, pair: Option<String>) -> PlayerCard<'a> {
	pair.as_ref()
		.and_then(|pair_name| strategy_table.get(pair_name))
		.or_else(|| strategy_table.get(&player_count.to_string()))
}

pub fn get_action(player_count: u8, dealer_card: String, pair: Option<String>) -> Option<Action> {
	let strategy_table = get_strategy();
	let player_actions = find_player_card(&strategy_table, player_count, pair)?;
	let action = player_actions.get(&dealer_card)?;
	Some(Action::from_str(action).expect(ActionError::InvalidAction.into()))
}
