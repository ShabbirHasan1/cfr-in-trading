use std::cell::RefCell;
use std::rc::Rc;

use cfr_proto::strategies::BasicStrategy;
use cfr_proto::{
    datasets, Backtester, Config, DatasetRef, InstrumentId, InstrumentSpec, ModelSet, ModelSetRef,
    ModelType, Position, Profit, Strategy,
};

type Point = datasets::nq_f1_f2_f4::Point;
use datasets::nq_f1_f2_f4::load as load_dataset;

fn main() {
    let config: Config = Config::parse().unwrap();
    if config.print_config {
        println!("{:#?}", config);
        return;
    }
    let instrument_id: InstrumentId = InstrumentId {
        symbol: "NQ".to_string(),
        index: 0,
    };
    let instrument_spec: InstrumentSpec = InstrumentSpec {
        multiplier: 20.0,
        fee: 1.65,
    };
    let position: Position = Position::new(instrument_id, instrument_spec);
    let positions: Rc<RefCell<Vec<Position>>> = Rc::new(RefCell::new(vec![position]));
    let model_set: ModelSetRef<Point> =
        ModelSet::new(config.backtest.iteration, &config.backtest.models_dir);
    model_set.load_model_params_with_close_from_previous_iteration();
    for model_type in ModelType::all() {
        let model = model_set.model(model_type);
        println!("{}: {}", model_type, model.params());
    }
    let strategy: BasicStrategy<Point> = BasicStrategy::new(positions.clone(), model_set);
    let strategy: Box<dyn Strategy<Point>> = Box::new(strategy);

    let offset: f64 = config.backtest.offset;
    let limit: f64 = config.backtest.limit;
    let dateset_ref: DatasetRef<Point> = load_dataset(&config.dataset_path, offset, limit);
    println!("Dataset size: {}", dateset_ref.len());

    let mut backtester: Backtester<Point> = Backtester::new(positions, dateset_ref, strategy);
    backtester.run();
    let profits: &[Profit] = backtester.realized_profits();
    let final_profit_usd: f64 = profits.iter().map(|profit| profit.profit).sum();
    let final_profit_bps: f64 = final_profit_usd / backtester.traded_volume_usd() * 10000.0;
    println!("N trades    : {}", profits.len());
    println!("Final profit: {} USD", final_profit_usd);
    println!("Final profit: {} bps", final_profit_bps);
    let mut csv_writer = csv::Writer::from_path(config.backtest.profits_output_file).unwrap();
    for profit in profits {
        csv_writer.serialize(profit).unwrap();
    }
    // let profits_str = json!(&profits).to_string();
    // std::fs::write(config.backtest.profits_output_file, profits_str).unwrap();
}
