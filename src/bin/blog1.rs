use rand::Rng;

fn main() {
    // dbg!(gen_initial_population(3));
    evolve();
}

const HAMLET: &str = "To be, or not to be--that is the question";
const LENGTH_HAMLET: usize = HAMLET.len();
const POPULATION_SIZE: usize = 1000;

#[derive(Debug)]
struct Individual(String);

fn evolve()  {
    let mut population = gen_initial_population(POPULATION_SIZE);

    loop {
        population = breed_next_generation(population);

        let fittest = population.iter().max_by_key(|i| i.fitness()).unwrap();
        println!("{}", fittest.0);

        if &fittest.0 == HAMLET {
            return;
        }
    }
}

fn gen_initial_population(size: usize) -> Vec<Individual> {
    let mut population = Vec::new();

    for _ in 0..size {
        let mut individual = Individual(String::new());
        for _ in 0..LENGTH_HAMLET {
            individual.0.push(gen_ascii_char());
        }
        population.push(individual);
    }

    population
}

fn breed_next_generation(mut population: Vec<Individual>) -> Vec<Individual> {
    let size = population.len();
    select_fittest_individuals(&mut population);
    let mut new_population = crossover_population(&population, size);
    mutate_population(&mut new_population);
    new_population
}

fn select_fittest_individuals(population: &mut Vec<Individual>) {
    population.sort_by_cached_key(|i| -i.fitness());
    population.truncate(population.len() / 2);
}

fn crossover_population(parents: &[Individual], size: usize) -> Vec<Individual> {
    let mut population = Vec::new();

    use rand::seq::SliceRandom; // for `choose` method
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        let parent1 = parents.choose(&mut rng).unwrap();
        let parent2 = parents.choose(&mut rng).unwrap();
        let child = parent1.crossover(parent2);
        population.push(child);
    }

    population
}

fn mutate_population(population: &mut [Individual]) {
    for individual in population {
        individual.mutate();
    }
}

impl Individual {
    fn fitness(&self) -> i32 {
        let my_chars = self.0.chars();
        let target_chars = HAMLET.chars();
        let chars_same = my_chars.zip(target_chars).filter(|(x, y)| x == y);
        chars_same.count() as i32
    }

    fn crossover(&self, other: &Individual) -> Individual {
        let index = rand::thread_rng().gen_range(0, LENGTH_HAMLET);

        let left = self.0[0..index].chars();
        let right = other.0[index..LENGTH_HAMLET].chars();
        let child_string = left.chain(right).collect();

        Individual(child_string)
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();

        self.0 = self.0.chars().map(|current_char| {
            if rng.gen_bool(0.01) {
                gen_ascii_char()
            } else {
                current_char
            }
        }).collect()
    }
}

fn gen_ascii_char() -> char {
    // printable ASCII ranges from 32 (space, ' ') to 126 (tilde, '~')
    rand::thread_rng().gen_range(32u8, 126u8) as char
}
