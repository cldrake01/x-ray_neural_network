/*

use ndarray::Array2;

#[derive(Debug)]
struct Network {
    num_layers: usize,
    sizes: Vec<usize>,
    biases: Vec<Array2<f64>>,
    weights: Vec<Array2<f64>>,
}

use ndarray_rand::rand_distr::StandardNormal;
use ndarray::{Array};
use ndarray_rand::RandomExt;

impl Network {
    fn new(sizes: &[usize]) -> Network {
        let num_layers = sizes.len();
        let mut biases: Vec<Array2<f64>> = Vec::new();
        let mut weights: Vec<Array2<f64>> = Vec::new();
        for i in 1..num_layers {
            biases.push(Array::random((sizes[i], 1), StandardNormal));
            weights.push(Array::random((sizes[i], sizes[i - 1]), StandardNormal));
        }
        Network {
            num_layers,
            sizes: sizes.to_owned(),
            biases,
            weights,
        }
    }
}

 */