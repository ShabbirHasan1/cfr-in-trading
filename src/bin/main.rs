use cfr_proto::{datasets, Config, DatesetRef, Iteration};

type Point = datasets::nq_f1_f2_f4::Point;
use datasets::nq_f1_f2_f4::load as load_dataset;

fn main() {
    let config: Config = Config::parse().unwrap();
    if config.print_config {
        println!("{:#?}", config);
        return;
    }
    println!("Reading dataset...");
    let dateset_ref: DatesetRef<Point> = load_dataset(&config.dataset_path);
    println!("Dataset size: {}", dateset_ref.len());
    for iteration_index in 0..(config.n_iterations as usize) {
        println!("Iteration: {}", iteration_index);
        let iteration: Iteration<Point> = Iteration::new(
            iteration_index,
            dateset_ref.clone(),
            config.iteration.clone(),
        );
        iteration.run();
        println!("{}", iteration.summary());
    }
}
