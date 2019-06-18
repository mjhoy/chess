pub struct FromToStep {
    from: u8,
    to: u8,
    step: i8,
}

impl Iterator for FromToStep {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let new_next: i8 = (self.from as i8) + self.step;

        if self.step < 0 {
            assert!(self.from > 0, "from cannot go negative");
        }

        self.from = new_next as u8;

        if self.from <= self.to && self.step < 0 {
            return None;
        }
        if self.from >= self.to && self.step > 0 {
            return None;
        }
        return Some(self.from);
    }
}

impl FromToStep {
    pub fn from_to(from: u8, to: u8) -> FromToStep {
        assert!(from != to, "from and to must be different values");

        if from > to {
            FromToStep { from, to, step: -1 }
        } else {
            FromToStep { from, to, step: 1 }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_to_positive() {
        let mut iterator = FromToStep::from_to(1, 4);
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_from_to_negative() {
        let mut iterator = FromToStep::from_to(4, 1);
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), None);
    }
}