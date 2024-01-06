pub mod checks;


use checks::card_checks::is_ace;
pub const CARD_ORDER: [&str; 13] = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
pub const FACE_CARDS: [&str; 3] = ["J", "Q", "K"];

pub fn convert_face_cards(hand: &Vec<String>) -> Vec<String> {
	hand
		.iter()
		.map(|card| FACE_CARDS.contains(&card.as_str()).then(|| "10".to_string()).unwrap_or(card.to_string()))
		.collect()
}

pub fn order_hand(hand: &Vec<String>) -> Vec<String> {
	hand
		.iter()
		.map(|card| {
			CARD_ORDER.iter().find(|c| c == &&card.as_str()).map(|c| c.to_string()).unwrap_or(card.to_string())
		})
		.collect()
}

pub fn hand_total(hand: &Vec<String>) -> u8 {
	hand
		.iter()
		.map(|card| {
			if is_ace(card) {
				11
			} else if FACE_CARDS.contains(&card.as_str()) {
				10
			} else {
				card.parse::<u8>().unwrap()
			}
		})
		.sum()
}