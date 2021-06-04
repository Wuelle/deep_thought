use anyhow::Result;
use ndarray::prelude::*;
use crate::{
    error::Error,
    loss::Loss,
    activation::Activation,
};
use ndarray_rand::{
    RandomExt,
    rand_distr::Uniform,
};


pub struct NeuralNetworkBuilder {
    layers: Vec<Layer>,
    lr: f64,
}

#[allow(non_snake_case)] // non snake case kinda makes sense with matrices
pub struct Layer {
    /// Weight matrix
    W: Array2<f64>,
    /// Bias vector
    B: Array2<f64>,
    /// Activation function which turns self.Z into self.A
    activation: Activation,
    /// inp * weight  + bias
    Z: Array2<f64>,
    /// Activation(Z), the actual activation of the neurons
    A: Array2<f64>,
    /// Accumulated weight gradients
    d_W: Array2<f64>,
    /// Accumulated bias gradients
    d_B: Array1<f64>,
}

impl Layer {
    /// construct a new layer with provided dimensions and random weights/biases
    pub fn new(input_dim: usize, output_dim: usize) -> Layer {
        Layer {
            W: Array::random((output_dim, input_dim), Uniform::new(-1., 1.)),
            B: Array::random((output_dim, 1), Uniform::new(-1., 1.)),
            activation: Activation::default(),
            Z: Array::zeros((0, output_dim)),
            A: Array::zeros((0, output_dim)),
            d_W: Array::zeros((0,0)),
            d_B: Array::zeros(0),
        }
    }

    /// construct a layer from provided weight/bias parameters
    pub fn from_parameters(parameters: (Array2<f64>, Array2<f64>)) -> Result<Layer> {
        let input_dim = parameters.0.ncols();
        let output_dim = parameters.0.nrows();
        Ok(Layer {
            W: parameters.0,
            B: parameters.1,
            Z: Array::zeros((0, output_dim)),
            A: Array::zeros((0, output_dim)),
            activation: Activation::default(),
            d_W: Array::zeros((0,0)),
            d_B: Array::zeros(0),
        })
    }

    /// get the weights/biases of the neurons
    pub fn get_parameters(&self) -> (Array2<f64>, Array2<f64>) {
        (self.W.clone(), self.B.clone())
    }

    /// manually set weights/biases for the neurons
    pub fn set_parameters(&mut self, parameters: (Array2<f64>, Array2<f64>)) -> Result<()> {
        // make sure the dimensions match before replacing the old ones
        if self.W.raw_dim() != parameters.0.raw_dim() {
            return Err(Error::MismatchedDimensions{
                expected: self.W.raw_dim().into_dyn(), 
                found: parameters.0.raw_dim().into_dyn(),
            }.into());
        }
        else if self.B.raw_dim() != parameters.1.raw_dim() {
            return Err(Error::MismatchedDimensions{
                expected: self.B.raw_dim().into_dyn(),
                found: parameters.1.raw_dim().into_dyn(),
            }.into())
        }

        self.W = parameters.0;
        self.B = parameters.1;

        Ok(())
    }

    /// define a activation function for that layer (default is f(x) = x )
    pub fn activation(mut self, a: Activation) -> Layer {
        self.activation = a;
        self
    }

    /// forward-pass a batch of input vectors through the layer
    pub fn forward(&mut self, inp: &Array2<f64>) {
        // println!("------");
        // println!("self.B.shape: {:?}", self.B.shape());
        // println!("self.W.shape: {:?}", self.W.shape());
        // println!("inp.shape:  {:?}", inp.shape());
        // println!("------");
        self.Z = self.W.dot(inp) + &self.B;
        self.A = self.activation.compute(&self.Z);
    }
}

impl NeuralNetworkBuilder {
    pub fn new() -> NeuralNetworkBuilder {
        NeuralNetworkBuilder {
            layers: vec![],
            lr: 0.01,
        }
    }

    /// add a hidden layer to the network
    pub fn add_layer(mut self, layer: Layer) -> NeuralNetworkBuilder {
        self.layers.push(layer);
        self
    }

    /// manually set the learning rate, default is 0.01
    pub fn learning_rate(mut self, lr: f64) -> NeuralNetworkBuilder {
        self.lr = lr;
        self
    }

    /// forward-pass a batch of input vectors through the network
    pub fn forward(&mut self, inp: &Array2<f64>) -> Array2<f64> {
        for index in 0..self.layers.len() {
            if index == 0 {
                self.layers[index].forward(&inp);
            } else {
                let prev_activation = self.layers[index - 1].A.clone();
                self.layers[index].forward(&prev_activation);
            }
        }
        self.layers.iter().last().unwrap().A.clone()
    }

    /// Backpropagate the output through the network and adjust weights/biases to further match the 
    /// desired target
    pub fn backprop(&mut self, input: Array2<f64>, target: Array2<f64>, loss: Loss) {
        let num_layers = self.layers.len();
        // Initial dz for the last layer
        // might need to be .A in the activation derivative
        let mut dz = &self.layers[num_layers - 1].activation.derivative(&self.layers[num_layers - 1].Z) * 
            loss.derivative(&self.layers[num_layers - 1].A, &target);
        //let mut dz = loss.derivative(&self.layers[num_layers - 1].A, &target);

        for n in (0..num_layers).rev() {
            let nth_layer = &self.layers[n];

            // determine the vector that is fed into the nth layer
            let nth_layer_input = if n == 0 {
                &input
            } else {
                &self.layers[n - 1].A
            };

            // find the derivative of the cost function with respect to the nth layers Z value
            if n != num_layers - 1 {
                dz = nth_layer.activation.derivative(&nth_layer.Z) * &self.layers[n + 1].W.clone().reversed_axes().dot(&dz);
            }
            println!("optimizing the layer which goes from {} to {}", nth_layer.W.nrows(), nth_layer.W.ncols());

            // println!("dz shape: {:?}", dz.shape());
            
            let dw = &dz.dot(&nth_layer_input.clone().reversed_axes());
            let db = (&dz.sum_axis(Axis(1))).to_shape((dz.nrows(), 1)).unwrap().to_owned(); // need to add an extra dim

            let nth_layer_mut = &mut self.layers[n];
            nth_layer_mut.W = &nth_layer_mut.W - dw * self.lr;
            // println!("B shape before: {:?}", nth_layer_mut.B.shape());
            nth_layer_mut.B = &nth_layer_mut.B - db * self.lr;
            // println!("B shape after: {:?}", nth_layer_mut.B.shape());
        }
    }
}

