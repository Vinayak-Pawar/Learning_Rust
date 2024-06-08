use linfa::prelude::*;
use linfa_datasets::iris;
use linfa_nn::NeuralNetwork;
use ndarray::Array2;

fn main() {
    // Load the Iris dataset
    let (train, test) = iris().unwrap();

    // Define the architecture of the neural network
    let layers = vec![
        Layer::Dense(Dense::new(4, 10).with_activation(Activation::ReLU)),
        Layer::Dense(Dense::new(10, 3).with_activation(Activation::Softmax)),
    ];

    // Create the neural network
    let mut network = NeuralNetwork::new(layers, Loss::CrossEntropy);

    // Train the neural network
    network.fit(&train.records().view(), &train.targets().view()).unwrap();

    // Evaluate the neural network
    let predictions = network.predict(&test.records().view());
    let accuracy = predictions
        .into_iter()
        .zip(test.targets().iter())
        .filter(|(pred, target)| pred == target)
        .count() as f64
        / test.targets().len() as f64;

    println!("Test Accuracy: {:.2}%", accuracy * 100.0);
}
