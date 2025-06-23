use std::{collections::HashMap, hash::Hash};

/// Counter counts the number of times each value of type T has been seen.
pub struct Counter<T> {
    values: HashMap<T, u64>,
}

impl<T> Counter<T>
where
    T: Eq + Hash,
{
    /// Create a new Counter.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Count an occurrence of the given value.
    pub fn count(&mut self, value: T) {
        let e = self.values.entry(value).or_default();
        *e += 1;
    }

    /// Return the number of times the given value has been seen.
    pub fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let mut ctr = Counter::new();
        ctr.count(13);
        ctr.count(14);
        ctr.count(16);
        ctr.count(14);
        ctr.count(14);
        ctr.count(11);

        for i in 10..20 {
            println!("saw {} values equal to {}", ctr.times_seen(i), i);
        }

        let mut strctr = Counter::new();
        strctr.count("apple");
        strctr.count("orange");
        strctr.count("apple");
        println!("got {} apples", strctr.times_seen("apple"));
    }
}
