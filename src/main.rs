use rand::{distributions::Uniform, prelude::Distribution};

/*
Steps in a genetic algorithm:

population
|> evaluate
|> selection
|> crossover
|> algorithm

a single step = one "evolution"
*/

type Solution = Vec<bool>;
type MyPopulation = Vec<Solution>; // todo: swap array of bools to bitstring

/// get best solution for the given population
fn algorithm(population: MyPopulation) -> Solution {
    let best: Vec<bool> = population
        .into_iter()
        .max_by_key(|v| v.iter().filter(|b| **b).count())
        .unwrap();
    // .iter().map(|p| p.iter()
    // .map(|b| if *b { 1 } else { 0 }).sum::<i32>()).max().unwrap();
    let best_value = best.iter().filter(|&&b| b).count();
    println!("Current best: {}", best_value);
    if best_value == 1000 {
        best
    } else {
        best
    }
}

type SelectionPair = (Solution, Solution);
struct TODO;

/// takes a population, evaluates each chromosome based on a fitness functiion, and sorts the population
/// by fitness.
fn evaluate(mut pop: MyPopulation) -> MyPopulation {
    pop.sort_by_key(|p| p.iter().filter(|&&b| b).count());
    pop
}

/// picks the parents that will be combined to create new solutions
fn selection(pop: MyPopulation) -> Vec<SelectionPair> {
    pop.chunks_exact(2)
        .map(|n| (n[0].clone(), n[1].clone()))
        .collect()
}

fn crossover(_sel: Vec<SelectionPair>) -> TODO {
    todo!()
}

fn main() {
    let mut rng = rand::thread_rng(); // R-N-Jesus
    let random_bool = Uniform::from(0..=1);
    let population: Vec<_> = (0..100)
        .map(|_| {
            (0..1000)
                .map(|_| random_bool.sample(&mut rng) != 0)
                .collect::<Vec<bool>>()
        })
        .collect();

    let pairs = selection(evaluate(population));
    println!("{:?}", pairs);
}
