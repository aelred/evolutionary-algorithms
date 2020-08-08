//! Example of https://blog.ael.red/2020/08/02/evolutionary-algorithms.html, but using my library
//! instead of hard-coding everything.

use evolutionary_algorithms::{evolve, Individual, Population};
use rand::Rng;

fn main() {
    evolve(1000, should_terminate);
}

const HAMLET: &str = "To be, or not to be--that is the question";
const LENGTH_HAMLET: usize = HAMLET.len();

#[derive(Debug)]
struct Soliloquy(String);

impl Individual for Soliloquy {
    fn new_random() -> Self {
        let mut sol = Soliloquy(String::new());
        for _ in 0..LENGTH_HAMLET {
            sol.0.push(gen_ascii_char());
        }
        sol
    }

    fn fitness(&self) -> i32 {
        let my_chars = self.0.chars();
        let target_chars = HAMLET.chars();
        let chars_same = my_chars.zip(target_chars).filter(|(x, y)| x == y);
        chars_same.count() as i32
    }

    fn crossover(&self, other: &Self) -> Self {
        let len = self.0.len();
        let index = rand::thread_rng().gen_range(0, len);

        let first_half = self.0[0..index].chars();
        let second_half = other.0[index..len].chars();
        let crossover_string = first_half.chain(second_half).collect();

        Soliloquy(crossover_string)
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();

        self.0 = self
            .0
            .chars()
            .map(|c| {
                if rng.gen_bool(0.01) {
                    gen_ascii_char()
                } else {
                    c
                }
            })
            .collect()
    }
}

fn should_terminate(population: &Population<Soliloquy>) -> bool {
    match population.fittest() {
        None => true,
        Some(fittest) => {
            println!("{}", fittest.0);
            &fittest.0 == HAMLET
        }
    }
}

fn gen_ascii_char() -> char {
    // printable ASCII ranges from 32 (space, ' ') to 126 (tilde, '~')
    rand::thread_rng().gen_range(32u8, 126u8) as char
}
