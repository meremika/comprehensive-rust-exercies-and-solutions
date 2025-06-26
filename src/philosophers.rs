use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Chopstick;

#[derive(Debug)]
pub struct Philosopher {
    name: String,
    first_chopstick: Arc<Mutex<Chopstick>>,
    second_chopstick: Arc<Mutex<Chopstick>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    pub fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    pub fn eat(&self) {
        let _first_chopstick = self.first_chopstick.lock().unwrap();
        let _second_chopstick = self.second_chopstick.lock().unwrap();
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

    #[test]
    fn main() {
        // Create chopsticks
        let chopsticks: Vec<_> = PHILOSOPHERS
            .iter()
            .map(|_| Arc::new(Mutex::new(Chopstick)))
            .collect();

        // Create philosophers
        let (thoughts_tx, thoughts_rx) = mpsc::channel();

        let philosophers: Vec<_> = PHILOSOPHERS
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let name = String::from(*name);
                let mut first_chopstick = Arc::clone(&chopsticks[i]);
                let mut second_chopstick = Arc::clone(&chopsticks[(i + 1) % chopsticks.len()]);
                let thoughts = thoughts_tx.clone();

                if i == 0 {
                    std::mem::swap(&mut first_chopstick, &mut second_chopstick);
                }
                Philosopher {
                    name,
                    first_chopstick,
                    second_chopstick,
                    thoughts,
                }
            })
            .collect();

        // Make each of them think and eat 100 times
        for p in philosophers {
            thread::spawn(move || {
                for _ in 0..100 {
                    p.think();
                    p.eat();
                }
            });
        }
        println!("Waiting for clever thoughts...");

        // Output their thoughts
        drop(thoughts_tx);
        while let Ok(thought) = thoughts_rx.recv() {
            println!("{thought}");
        }
    }
}
