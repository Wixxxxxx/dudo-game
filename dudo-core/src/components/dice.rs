use rand::random_range;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dice {
    pub face: Option<u8>,
}

impl Dice {
    pub fn new() -> Self {
        Self { face: None }
    }
}

pub struct Hand {
    pub dice: Vec<Dice>,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            dice: vec![Dice::new(); 5],
        }
    }

    pub fn remove_random(&mut self) {
        let idx = random_range(0..self.dice.iter().len());
        self.dice.swap_remove(idx);
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "🎲 ")?;
        for die in &self.dice {
            match die.face {
                Some(1) => write!(f, "⚀ ")?,
                Some(2) => write!(f, "⚁ ")?,
                Some(3) => write!(f, "⚂ ")?,
                Some(4) => write!(f, "⚃ ")?,
                Some(5) => write!(f, "⚄ ")?,
                Some(6) => write!(f, "⚅ ")?,
                None => write!(f, "? ")?,
                _ => write!(f, "� ")?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dice_can_be_initialized() {
        let d = Dice::new();
        assert_eq!(d.face, None)
    }

    #[test]
    fn hand_can_be_initialized() {
        let h = Hand::new();
        assert_eq!(h.dice, vec![Dice::new(); 5])
    }
}
