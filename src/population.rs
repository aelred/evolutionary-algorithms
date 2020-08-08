use crate::Individual;
use rand::seq::SliceRandom;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct Population<T> {
    individuals: Vec<T>,
}

impl<T: Individual> Population<T> {
    pub(crate) fn gen_initial(size: usize) -> Self {
        let mut generation = Self::empty(size);

        for _ in 0..size {
            generation.add(T::new_random());
        }

        generation.sort_by_fitness();

        generation
    }

    fn empty(size: usize) -> Self {
        Population {
            individuals: Vec::with_capacity(size),
        }
    }

    pub fn fittest(&self) -> Option<&T> {
        self.first()
    }

    pub(crate) fn breed_next_generation(&mut self) -> Self {
        let size = self.len();
        let parents = &self[0..size / 2];
        let mut new_population = Self::crossover(parents, size);
        new_population.mutate();
        new_population.sort_by_fitness();
        new_population
    }

    fn sort_by_fitness(&mut self) {
        self.sort_by_cached_key(|i| -i.fitness());
    }

    fn crossover(parents: &[T], size: usize) -> Self {
        let mut generation = Self::empty(size);

        let mut rng = rand::thread_rng();

        for _ in 0..size {
            let parent1 = parents.choose(&mut rng).unwrap();
            let parent2 = parents.choose(&mut rng).unwrap();
            let child = parent1.crossover(parent2);
            generation.add(child);
        }

        generation
    }

    fn mutate(&mut self) {
        for individual in &mut self.individuals {
            individual.mutate();
        }
    }

    fn add(&mut self, individual: T) {
        self.individuals.push(individual);
    }
}

impl<T> Deref for Population<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.individuals
    }
}

impl<T> DerefMut for Population<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.individuals
    }
}
