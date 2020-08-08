mod population;

pub use population::Population;

pub trait Individual: Sized {
    fn new_random() -> Self;
    fn fitness(&self) -> i32;
    fn crossover(&self, other: &Self) -> Self;
    fn mutate(&mut self);
}

pub fn evolve<T, F>(population_size: usize, until: F)
where
    T: Individual,
    F: Fn(&Population<T>) -> bool,
{
    let mut population = Population::<T>::gen_initial(population_size);

    while !until(&population) {
        population = population.breed_next_generation();
    }
}
