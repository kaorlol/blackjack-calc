use crate::hand_utils::{checks::card_checks::is_ace, convert_face_cards, hand_total, order_hand, FACE_CARDS};

fn is_number(str: &str) -> bool {
	str.parse::<u8>().is_ok()
}

pub fn is_pair(cards: &Vec<String>) -> Option<String> {
	let ordered_cards = order_hand(&convert_face_cards(&cards));

	match ordered_cards.as_slice() {
		[first_card, second_card] if is_ace(first_card) && (is_ace(second_card) || is_number(second_card)) => {
			println!("Found pair: {first_card},{second_card}");
			Some(format!("{},{}", first_card, second_card))
		}
		[first_card, second_card] if is_number(first_card) && is_number(second_card) && first_card == second_card => {
			println!("Found pair: {first_card},{second_card}");
			Some(format!("{},{}", first_card, second_card))
		}
		_ => None,
	}
}

pub fn is_valid_hand(cards: &Vec<String>) -> bool {
	cards.iter().all(|card| is_number(card) || FACE_CARDS.contains(&card.as_str()) || is_ace(card))
}

pub fn is_bust(cards: &Vec<String>) -> bool {
	let ordered_cards = order_hand(&convert_face_cards(&cards));
	let total = hand_total(&ordered_cards);

	!is_pair(&ordered_cards).is_some() && total > 21
}
