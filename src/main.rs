use rand::{distributions::Uniform, prelude::Distribution, Rng, seq::SliceRandom};

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
        .iter()
        .max_by_key(|v| v.iter().filter(|b| **b).count())
        .unwrap().to_vec();
    // .iter().map(|p| p.iter()
    // .map(|b| if *b { 1 } else { 0 }).sum::<i32>()).max().unwrap();
    let best_value = best.iter().filter(|&&b| b).count();
    println!("Current best: {}", best_value);
    if best_value == 1000 {
        best
    } else {
        let sorted = evaluate(population);
        let selections = selection(sorted);
        let new_gen = crossover(selections);
        let mutated_a_bit = mutate(new_gen);
        algorithm(mutated_a_bit)
    }
}

type SelectionPair = (Solution, Solution);

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

/// analogous to reproduction. takes two (or more) parent chromosomes (solutons) and
/// produces two or more child chromosomes.
fn crossover(sel: Vec<SelectionPair>) -> MyPopulation {
    sel.into_iter().fold(Vec::new(), |mut acc, (p1, p2)| {
        let mut rng = rand::thread_rng();
        let crossover_pt = rng.gen_range(0..1000);
        let (head1, tail1) = p1.split_at(crossover_pt);
        let (head2, tail2) = p2.split_at(crossover_pt);
        let child1 = [head1, tail2].concat();
        let child2 = [head2, tail1].concat();
        acc.push(child1);
        acc.push(child2);
        acc
    })
}

fn mutate(new_gen: MyPopulation) -> MyPopulation {
    new_gen.into_iter().map(|mut chromosome| {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..1.0) < 0.05 {
            chromosome.shuffle(&mut rng);
            chromosome
        } else {
            chromosome            
        }
    }).collect()
}

fn main() {
    let mut rng = rand::thread_rng(); // R-N-Jesus
    let random_bool = Uniform::from(0..=1);
    let initial_population: Vec<_> = (0..100)
        .map(|_| {
            (0..1000)
                .map(|_| random_bool.sample(&mut rng) != 0)
                .collect::<Vec<bool>>()
        })
        .collect();

    let solution = algorithm(initial_population);

    let c = solution.iter().filter(|&&b| b).count();

    println!("Answer is {:?}", c);
}
