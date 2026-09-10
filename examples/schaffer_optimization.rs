use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Schaffer;

fn main() {
    println!("Running NSGA-II on Schaffer N.1 problem...");

    let result = Evolution::new(Schaffer::default(), 100, 200)
        .with_seed(42)
        .with_reference_point(vec![10.0, 10.0])
        .evolve();

    println!("Evolution complete!");
    println!("Generations completed: {}", result.generations_completed);
    println!("Final Pareto front size: {}", result.pareto_front.len());

    for (i, ind) in result.pareto_front.iter().enumerate().take(5) {
        println!("  Individual {}: objectives = {:?}", i, ind.objectives);
    }
}
