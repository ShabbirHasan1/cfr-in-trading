# Iterative Training of Open and Close Models

## Idea

On each iteration we train 4 models:

+ opening long

+ opening short

+ closing long

+ closing short

Each model accepts a bar (some feature vector) and returns a scalar that is interpreted as **utility**
of non-trivial action.

Non-trivial action of openinig models is opening.

Non-trivial action of closing models is doing nothing.

Utility of the trivial action is 0.

Utility formula:

```rust
let price_return: f64 = current_bar.mid_price.0 - start_bar.mid_price.0;
let sign: f64 = match self.trained_model_type.side {
   ModelSide::Long => 1.0,
   ModelSide::Short => -1.0,
};
let fee: f64 = match self.trained_model_type.action {
   ModelAction::Opening => self.fee * 2.0,
   ModelAction::Closing => 0.0,
};
let utility: f64 = ((price_return * sign) * self.multiplier - fee) * 10_000.0 
    / (start_bar.mid_price.0 * self.multiplier)
   - self.utility_penalty_bps;
```

For the openining models, `utility = price_return - 2 * fee`

For the closing models, `utility = price_return`

Each iteration includes 2 stages: collecting training set and training.

To collect a training, we run **plays**.

To run a play, we randomly select a bar and from that bar do inference with the closing model 
from the previous iteration. If the closing model returns a positive utility, we move to the next bar,
otherwise we consider the play finished: closing now is better than closing later.

After all plays in a batch are finished, we calculate the utility from each play. Price return is calculated
from between the start and end bars of the play.

Training sample is the combination of the starting bar of the play and the utility from that play.

Then we train all the models.

Then we start the next iteration.

This schema assumes that opening models depend on the closing models, but the closing model
do not depend on the opening models. 

The opening models tell if we can gain more than 2 fees by opening now given the current bar and
the closing models.

The closing models tell if closing later is expected to be better than closing now. 
To do so, they do not need to know current floating PnL.

Feature space (shape and semantics of the bars) is domain-specific.

Training and inference is done within the CPython runtime. 
Rust executable is linked against a shared library that initiates CPython and passes calls to it.

Modification in python code can be done without recompiling the rust code.

## Using Trained Models

Opening decision:

1. Get utility from the opening long model.

2. Get utility from the opening short model.

3. Act as follows:
   
   + If none of the utilities is positive, do nothing.

   + If only one of the utilities is positive, open long or short accordingly.

   + If both utilities are positive open long or short with probabilities proportional to the sum of utilities.

Closing decision:

1. Get utility from the closing long or closing short model, depending on position.

2. If this utility is positive, do nothing, otherwise close now.


## Building

```bash
make build
```

This will compile the shared library that embeds CPython.


## Running

```bash
target/release/main -h

Prototype of counterfactual regret minimization for trading

Usage: main [OPTIONS]

Options:
  -p, --print-config          Print config
  -c, --config <CONFIG_PATH>  Config path [default: io/config.toml]
  -h, --help                  Print help
```


## Example 

Dataset: 3-feature points (features 1, 2 and 4 as referred to in index stratagies):
NQ mid-price vs inferred NQ mid-price by referent equities.

Interval: 2023

Model: linear regression

Config:

```toml
dataset_path = "io/dataset.bin"
start_iteration = 1
n_iterations = 100

[iteration]
n_plays = 4_000_000
concurrency = 10
output_dir = "io/models"
fee_per_contract_usd = 1.65 # NQ
multiplier = 20.0 # NQ
utility_penalty_bps = 0.05 # profit decrease due to execution not by mid-price
max_play_duration_in_bars  = 900
```

Results:

```
+-----------+------------------+-------------------------+
| Iteration | Mean Play Length | Mean Utility Prediction |
+-----------+------------------+-------------------------+
|     1     |       2.21       |         -0.0003         |
|     2     |       8.18       |          0.0215         |
|     3     |      52.96       |          0.6811         |
|     4     |      37.27       |          0.8369         |
|     5     |      60.28       |          0.3708         |
|     6     |      71.83       |          0.884          |
|     7     |      80.48       |          0.7733         |
|     8     |      103.05      |          0.7782         |
|     9     |      170.67      |          0.7063         |
|     10    |      109.74      |          0.846          |
|     11    |      356.92      |          1.0552         |
|     12    |      459.05      |          4.7853         |
|     13    |      458.33      |          6.9034         |
|     14    |      458.86      |          6.9154         |
|     15    |      466.45      |          6.7672         |
|     16    |      462.4       |          6.9526         |
|     17    |      464.71      |          6.7602         |
|     18    |      457.02      |          6.9897         |
|     19    |      457.25      |          7.0637         |
|     20    |      459.13      |          6.9206         |
+-----------+------------------+-------------------------+
```

Predicted utility is averaged here over all decision points of the play.

In a play only the last prediction of utility is negative (termination condition). 
This is why the mean prediction increases with the play length.
