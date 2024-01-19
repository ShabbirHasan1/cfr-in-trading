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
