use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Schaffer;

fn main() {
    println!("Running Schaffer with early stopping...");

    let result = Evolution::new(Schaffer::default(), 100, 1000)
        .with_seed(42)
        .with_reference_point(vec![10.0, 10.0])
        .with_convergence_threshold(20, 0.0001) // Window of 20 gens, min delta 0.0001
        .evolve();

    println!("Evolution halted!");
    println!("Requested generations: 1000");
    println!(
        "Generations actually completed: {}",
        result.generations_completed
    );

    if let Some(final_hv) = result.hypervolume_history.last() {
        println!("Final Hypervolume: {:.6}", final_hv);
    }
}
