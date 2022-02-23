// Copyright 2018-2022 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(unused_imports)]

use argmin::core::{ArgminSlogLogger, CostFunction, Error, Executor, Gradient, ObserverMode};
use argmin::solver::gradientdescent::SteepestDescent;
use argmin::solver::linesearch::HagerZhangLineSearch;
use argmin::solver::linesearch::MoreThuenteLineSearch;
use argmin_testfunctions::{rosenbrock_2d, rosenbrock_2d_derivative};

struct Rosenbrock {
    a: f64,
    b: f64,
}

impl CostFunction for Rosenbrock {
    type Param = Vec<f64>;
    type Output = f64;

    fn cost(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        Ok(rosenbrock_2d(p, self.a, self.b))
    }
}

impl Gradient for Rosenbrock {
    type Param = Vec<f64>;
    type Gradient = Vec<f64>;

    fn gradient(&self, p: &Self::Param) -> Result<Self::Param, Error> {
        Ok(rosenbrock_2d_derivative(p, self.a, self.b))
    }
}

fn run() -> Result<(), Error> {
    // Define cost function (must implement `ArgminOperator`)
    let cost = Rosenbrock { a: 1.0, b: 100.0 };

    // Define initial parameter vector
    // easy case
    let init_param: Vec<f64> = vec![1.2, 1.2];
    // tough case
    // let init_param: Vec<f64> = vec![-1.2, 1.0];

    // Pick a line search.
    // let linesearch = HagerZhangLineSearch::new();
    let linesearch = MoreThuenteLineSearch::new();

    // Set up solver
    let solver = SteepestDescent::new(linesearch);

    // Run solver
    let res = Executor::new(cost, solver)
        .configure(|config| config.param(init_param).max_iters(10))
        .add_observer(ArgminSlogLogger::term(), ObserverMode::Always)
        .run()?;

    // Wait a second (lets the logger flush everything first)
    std::thread::sleep(std::time::Duration::from_secs(1));

    // print result
    println!("{}", res);
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("{}", e);
        std::process::exit(1);
    }
}
