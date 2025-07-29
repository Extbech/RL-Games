use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Enum representing different activation functions used in the neural network.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActivationFunction {
    /// Rectified Linear Unit (ReLU) activation function.
    /// ReLU is defined as f(x) = max(0, x).
    ReLU,
    /// Sigmoid activation function.
    /// Sigmoid is defined as f(x) = 1 / (1 + exp(-x)).
    Sigmoid,
    /// Hyperbolic Tangent (Tanh) activation function.
    /// Tanh is defined as f(x) = (exp(x) - exp(-x)) / (exp(x) + exp(-x)).
    Tanh,
    /// Linear activation function.
    /// Linear is defined as f(x) = x.
    Linear,
}

impl ActivationFunction {
    /// Applies the activation function to the input value.
    pub fn apply(&self, x: f64) -> f64 {
        match self {
            ActivationFunction::ReLU => {
                if x < 0.0 {
                    0.0
                } else {
                    x
                }
            }
            ActivationFunction::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            ActivationFunction::Tanh => x.tanh(),
            ActivationFunction::Linear => x,
        }
    }

    /// Returns the derivative of the activation function using the activated value.
    ///
    /// For example, for Sigmoid, f'(x) = f(x)*(1 - f(x)).
    pub fn derivative(&self, activated: f64) -> f64 {
        match self {
            ActivationFunction::ReLU => {
                if activated > 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            ActivationFunction::Sigmoid => activated * (1.0 - activated),
            ActivationFunction::Tanh => 1.0 - activated.powi(2),
            ActivationFunction::Linear => 1.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LossFunction {
    /// Mean Squared Error (MSE) loss function.
    MeanSquaredError,
    BinaryCrossEntropy,
}

impl LossFunction {
    /// Calculates the loss between the predicted output and the target output.
    pub fn loss(&self, predicted: &[f64], target: &[f64]) -> f64 {
        let epsilon = 1e-10; // Small value to prevent log(0) or division by zero
        match self {
            LossFunction::MeanSquaredError => {
                predicted
                    .iter()
                    .zip(target)
                    .map(|(p, t)| (p - t).powi(2))
                    .sum::<f64>()
                    / predicted.len() as f64
            }
            LossFunction::BinaryCrossEntropy => {
                predicted
                    .iter()
                    .zip(target)
                    .map(|(p, t)| {
                        let p = p.clamp(epsilon, 1.0 - epsilon); // Clamp predicted values
                        if *t == 1.0 {
                            -p.ln()
                        } else {
                            -(1.0 - p).ln()
                        }
                    })
                    .sum::<f64>()
                    / predicted.len() as f64
            }
        }
    }

    /// Calculates the gradient of the loss function with respect to the predicted output.
    fn gradient(&self, predicted: &[f64], target: &[f64]) -> Vec<f64> {
        let epsilon = 1e-10; // Small value to prevent division by zero
        match self {
            LossFunction::MeanSquaredError => predicted
                .iter()
                .zip(target)
                .map(|(p, t)| (p - t) / predicted.len() as f64)
                .collect(),
            LossFunction::BinaryCrossEntropy => predicted
                .iter()
                .zip(target)
                .map(|(p, t)| {
                    let p = p.clamp(epsilon, 1.0 - epsilon); // Clamp predicted values
                    if *t == 1.0 {
                        1.0 / p
                    } else {
                        -1.0 / (1.0 - p)
                    }
                })
                .collect(),
        }
    }
}
/// A simple deep neural network struct.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuralNetwork {
    layers: Vec<Layer>,
    learning_rate: f64,
    activation_function: ActivationFunction,
    final_activation: ActivationFunction,
    loss_function: LossFunction,
    history: Vec<f64>,
}

impl NeuralNetwork {
    /// Creates a new NeuralNetwork with the given layer sizes, learning rate and activations.
    ///
    /// For example, layer_sizes = [4, 5, 3] creates a network with 4 inputs,
    /// one hidden layer with 5 neurons and an output layer with 3 neurons.
    pub fn new(
        learning_rate: f64,
        activation_function: ActivationFunction,
        final_activation: ActivationFunction,
        loss_function: LossFunction,
    ) -> Self {
        let layers = Vec::new();
        NeuralNetwork {
            layers,
            learning_rate,
            activation_function,
            final_activation,
            loss_function,
            history: Vec::new(),
        }
    }

    pub fn add_layers(&mut self, layer_sizes: &[usize]) {
        for i in 0..layer_sizes.len() - 1 {
            self.layers
                .push(Layer::new(layer_sizes[i], layer_sizes[i + 1]));
        }
    }

    /// Performs a forward pass through the network.
    /// Each layer computes its output which is fed as input to the next layer.
    /// Returns the activated outputs of the final layer.
    fn forward_inner(&self, mut input: Vec<f64>, with_cache: bool) -> (Vec<Vec<f64>>, Vec<f64>) {
        let mut cache: Vec<Vec<f64>> = Vec::new();
        if with_cache {
            cache.push(input.clone());
        }
        for (i, layer) in self.layers.iter().enumerate() {
            input = if i < self.layers.len() - 1 {
                // Hidden layers use the standard activation function.
                layer.forward(&input, &self.activation_function)
            } else {
                // Final layer uses the final activation function.
                layer.forward(&input, &self.final_activation)
            };
            if with_cache {
                cache.push(input.clone());
            }
        }
        (cache, input)
    }

    fn forward(&self, input: Vec<f64>) -> Vec<Vec<f64>> {
        // Perform the forward pass and return the cache of activations.
        self.forward_inner(input, true).0
    }

    pub fn predict(&self, input: Vec<f64>) -> Vec<f64> {
        self.forward_inner(input, false).1
    }

    pub fn predict_batch(&self, inputs: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        inputs
            .into_iter()
            .map(|input| self.predict(input))
            .collect()
    }

    ///
    fn backpropagation(&mut self, cache: Vec<Vec<f64>>, target: &[f64], log: bool) {
        let max_gradient = 1.0; // Maximum gradient value for clipping

        // 1. Compute initial delta at the output layer.
        // Get the output of the final layer.
        let output = cache.last().unwrap();
        if log {
            self.history.push(self.loss_function.loss(output, target));
        }
        let loss_grad = self.loss_function.gradient(output, target);
        let mut delta: Vec<f64> = loss_grad
            .iter()
            .zip(output)
            .map(|(lg, &o)| lg * self.final_activation.derivative(o))
            .collect();

        // 2. Iterate backwards over layers.
        for i in (1..cache.len() - 1).rev() {
            let activations = &cache[i];
            let layer_index = i;

            // Compute gradients for weights and biases.
            let mut weight_gradients = vec![vec![0.0; activations.len()]; delta.len()];
            for j in 0..delta.len() {
                for k in 0..activations.len() {
                    weight_gradients[j][k] = delta[j] * activations[k];
                }
            }

            // Clip gradients to prevent exploding gradients.
            for grad_row in weight_gradients.iter_mut() {
                for grad in grad_row.iter_mut() {
                    *grad = grad.clamp(-max_gradient, max_gradient);
                }
            }

            // Update weights and biases.
            for j in 0..self.layers[layer_index].weights.len() {
                for k in 0..self.layers[layer_index].weights[j].len() {
                    self.layers[layer_index].weights[j][k] -=
                        self.learning_rate * weight_gradients[j][k];
                }
                self.layers[layer_index].biases[j] -= self.learning_rate * delta[j];
            }

            // 3. Propagate the error to previous layer if not at the first layer.
            if layer_index > 0 {
                let mut new_delta = vec![0.0; self.layers[layer_index - 1].weights.len()];
                for k in 0..new_delta.len() {
                    for j in 0..delta.len() {
                        new_delta[k] += delta[j] * self.layers[layer_index].weights[j][k];
                    }
                    let activation = cache[i][k];
                    new_delta[k] *= self.activation_function.derivative(activation);
                }
                delta = new_delta;
            }
        }
    }
    pub fn train(&mut self, input: Vec<Vec<f64>>, target: Vec<Vec<f64>>) {
        let sty: ProgressStyle = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("#>-");
        let pb = ProgressBar::new(input.len() as u64);
        pb.set_style(sty.clone());

        for (x, y) in input.iter().zip(target.iter()) {
            let cache = self.forward(x.clone());
            self.backpropagation(cache, y, true);
            pb.set_message("Training..");
            pb.inc(1);
        }
    }
    /// Returns the loss history of the training process.
    pub fn get_history(&self) -> &[f64] {
        &self.history
    }
}

/// Represents a single layer in the neural network.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layer {
    pub weights: Vec<Vec<f64>>,
    pub biases: Vec<f64>,
}

impl Layer {
    /// Creates a new Layer with the given input and output sizes.
    /// Weights are randomly initialized using a uniform distribution; biases are set to zero.
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let mut rng = rand::rng();

        let weights: Vec<Vec<f64>> = (0..output_size)
            .map(|_| (0..input_size).map(|_| rng.random_range(-1.0..1.0)).collect())
            .collect();
        let biases = vec![0.0; output_size];
        Layer { weights, biases }
    }

    /// Performs a forward pass for this layer.
    ///
    /// Given an input vector, computes the weighted sum (z) and applies the activation function to produce output.
    /// Returns a tuple containing (z, activated_output).
    pub fn forward(&self, input: &[f64], activation_func: &ActivationFunction) -> Vec<f64> {
        let mut output = Vec::with_capacity(self.weights.len());
        for (i, weights_row) in self.weights.iter().enumerate() {
            let sum: f64 = input.iter().zip(weights_row).map(|(inp, w)| inp * w).sum();
            output.push(activation_func.apply(sum + self.biases[i]));
        }
        output
    }

    pub fn set_weights(&mut self, next_neuron: usize, current_neuron: usize, value: f64) {
        self.weights[next_neuron][current_neuron] = value;
    }
    pub fn update_bias(&mut self, neuron: usize, value: f64) {
        self.biases[neuron] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neural_network_creation() {
        let mut nn = NeuralNetwork::new(
            0.01,
            ActivationFunction::ReLU,
            ActivationFunction::Sigmoid,
            LossFunction::MeanSquaredError,
        );
        nn.add_layers(&[2, 3, 3, 2]);
        // Check if the number of layers is correct.
        assert_eq!(nn.layers.len(), 3);
    }

    #[test]
    fn test_layer_creation() {
        let layer = Layer::new(3, 2);
        assert_eq!(layer.weights.len(), 2); // should have 2 rows for 2 outputs.
        assert_eq!(layer.weights[0].len(), 3); // each row has 3 weights.
    }

    #[test]
    fn test_forward_pass() {
        let mut nn = NeuralNetwork::new(
            0.01,
            ActivationFunction::ReLU,
            ActivationFunction::Sigmoid,
            LossFunction::MeanSquaredError,
        );
        nn.add_layers(&[2, 3, 2]);
        let input = vec![1.0, 2.0];
        let output = nn.forward(input);
        // Check that output length equals the size of the final layer.
        assert_eq!(output.len(), 3);
        assert_eq!(output.last().unwrap().len(), 2); // Final layer should have 2 outputs.
    }

    #[test]
    fn test_train_step() {
        let mut nn = NeuralNetwork::new(
            0.01,
            ActivationFunction::ReLU,
            ActivationFunction::Sigmoid,
            LossFunction::MeanSquaredError,
        );
        nn.add_layers(&[2, 3, 2]);
        let input = vec![vec![0.5, -0.5], vec![0.7, -0.5], vec![0.5, -0.7]];
        let target = vec![vec![1.0, 0.0], vec![1.0, 0.0], vec![1.0, 0.0]];

        // Run training
        nn.train(input, target);
    }

    #[test]
    fn test_backward_pass() {
        let mut nn = NeuralNetwork::new(
            0.01,
            ActivationFunction::ReLU,
            ActivationFunction::ReLU,
            LossFunction::MeanSquaredError,
        );
        nn.add_layers(&[2, 2, 1]);
        
        // Manually set weights and biases for testing.

        // INPUT LAYER
        nn.layers[0].set_weights(0, 0, 0.5);
        nn.layers[0].set_weights(0, 1, 0.5);
        nn.layers[0].set_weights(1, 0, 0.5);
        nn.layers[0].set_weights(1, 1, 1.0);
        nn.layers[0].update_bias(0, 0.1);
        nn.layers[0].update_bias(1, 0.1);

        // HIDDEN LAYER
        nn.layers[1].set_weights(0, 0, 0.3);
        nn.layers[1].set_weights(0, 1, 0.2);
        nn.layers[1].update_bias(0, 0.1);
        
        let input = vec![0.5, 0.2];
        let target = vec![0.5];

        // L2-N0 ==> 0.5 * 0.5 + 0.5 * 0.2 + 0.1 = 0.45
        // L2-N1 ==> 0.5 * 0.5 + 1.0 * 0.2 + 0.1 = 0.55

        // L3-N0 ==> 0.3 * 0.45 + 0.2 * 0.55 + 0.1 = 0.345 
        // Forward pass
        let cache = nn.forward(input.clone());

        assert!((cache.last().unwrap().first().unwrap() - 0.345).abs() < 1e-5);
        // Backward pass
        nn.backpropagation(cache, &target, true);
        assert!((nn.history.first().unwrap() - 0.024025).abs() < 1e-5);
        // // Check if weights and biases have been updated.
        // assert!(nn.layers.iter().any(|layer| {
        //     layer.weights.iter().any(|weights| weights.iter().any(|&w| w != 0.0))
        //         || layer.biases.iter().any(|&b| b != 0.0)
        // }));
    }
}
