//! Glossary:
//! - Chromosome - a single solution to a problem
//! - Evaluation - evaluates each chromosome with some fitness function and sorts the population

use std::fmt::Debug;

type FitnessScore = i32;

fn evolve<C: Chromosome>(population: Vec<C>, max_fitness: FitnessScore) -> Option<C> {
    let new_population = evaluate(population);
    let best = new_population.first().unwrap();
    println!("Current best {:?}", best);
    if best.calculate_fitness() == max_fitness {
        Some(best.clone())
    } else {
        todo!()
    }
}

fn evaluate<C: Chromosome>(_population: Vec<C>) -> Vec<C> {
    todo!()
}

trait Fitness {
    fn calculate_fitness(&self) -> FitnessScore;
}
trait Chromosome: Debug + Clone + PartialEq + Fitness {}
