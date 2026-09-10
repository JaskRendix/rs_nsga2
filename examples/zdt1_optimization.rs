use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::ZDT1;

fn main() {
    println!("Running NSGA-II on ZDT1 problem...");

    let result = Evolution::new(ZDT1::new(30), 100, 300)
        .with_seed(123)
        .with_reference_point(vec![2.0, 12.0]) // f1 max ~1.0, f2 max ~10.0
        .evolve();

    println!("Evolution complete!");
    println!("Generations completed: {}", result.generations_completed);
    println!("Final Pareto front size: {}", result.pareto_front.len());

    if let Some(last_hv) = result.hypervolume_history.last() {
        println!("Final Hypervolume: {:.4}", last_hv);
    }
}
