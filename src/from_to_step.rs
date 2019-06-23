pub struct FromToStep {
    from: i8,
    to: i8,
    step: i8,
}

impl Iterator for FromToStep {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let (from, to, step) = (self.from, self.to, self.step);
        let new_from: i8 = from + step;

        debug_assert!(!(step < 0 && from < 1), "from cannot go negative");

        self.from = new_from;

        if new_from <= to && step < 0 {
            return None;
        }
        if new_from >= to && step > 0 {
            return None;
        }
        Some(new_from as u8)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let steps_remaining = ((self.to - self.from) / self.step).abs() as usize - 1;
        (steps_remaining, Some(steps_remaining))
    }
}

impl FromToStep {
    pub fn from_to(from: u8, to: u8) -> FromToStep {
        assert!(from != to, "from and to must be different values");

        if from > to {
            FromToStep {
                from: from as i8,
                to: to as i8,
                step: -1,
            }
        } else {
            FromToStep {
                from: from as i8,
                to: to as i8,
                step: 1,
            }
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

    #[test]
    fn test_size_hint() {
        let ranges = vec![(4, 1), (1, 4), (1, 9)];

        for range in ranges {
            let iterator = FromToStep::from_to(range.0, range.1);
            let mut steps = 0;
            let initial_size_hint = iterator.size_hint();
            for _i in iterator {
                steps += 1;
            }
            assert_eq!(initial_size_hint, (steps, Some(steps)));
        }
    }
}
