use rs_nsga2::evolve::Evolution;
use rs_nsga2::problem::Schaffer;

fn main() {
    println!("Running Schaffer with IGD and GD metric tracking...");

    // Generate a mock or analytical true Pareto front for Schaffer N.1
    // For Schaffer, f1 = x^2, f2 = (x-2)^2 for x in [0, 2].
    let true_front: Vec<Vec<f64>> = (0..=50)
        .map(|i| {
            let x = (i as f64) * 2.0 / 50.0;
            vec![x.powi(2), (x - 2.0).powi(2)]
        })
        .collect();

    let result = Evolution::new(Schaffer::default(), 100, 150)
        .with_seed(42)
        .with_true_front(true_front)
        .evolve();

    println!("Evolution complete!");
    println!("Generations completed: {}", result.generations_completed);

    if let Some(final_igd) = result.igd_history.last() {
        println!(
            "Final Inverted Generational Distance (IGD): {:.6}",
            final_igd
        );
    }

    if let Some(final_gd) = result.gd_history.last() {
        println!("Final Generational Distance (GD): {:.6}", final_gd);
    }
}
