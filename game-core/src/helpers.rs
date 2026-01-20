use crate::components::Dice;
use crate::components::Hand;
use rand::random_range;

pub fn roll_dice(dice: &mut Dice) {
    dice.face = Some(random_range(1..7));
}

pub fn roll_hand(hand: &mut Hand) {
    hand.dice.iter_mut().for_each(|die| roll_dice(die));
}
