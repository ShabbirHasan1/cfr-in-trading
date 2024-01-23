use cfr_proto::{datasets, Config, DatasetRef, Iteration};

type Point = datasets::nq_f1_f2_f4::Point;
use datasets::nq_f1_f2_f4::load as load_dataset;

fn main() {
    let config: Config = Config::parse().unwrap();
    if config.print_config {
        println!("{:#?}", config);
        return;
    }
    // println!("Reading dataset...");
    let offset: f64 = config.iteration.offset;
    let limit: f64 = config.iteration.limit;
    let dateset_ref: DatasetRef<Point> = load_dataset(&config.dataset_path, offset, limit);
    println!("Dataset size: {}", dateset_ref.len());
    let start = config.start_iteration as usize;
    let stop = start + config.n_iterations as usize;
    for iteration_index in start..stop {
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
