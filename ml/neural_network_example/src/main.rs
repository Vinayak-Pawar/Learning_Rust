use linfa::prelude::*;
use linfa_trees::DecisionTree;
use linfa_datasets::diabetes;
use ndarray::Array2;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    // Load the Diabetes dataset
    let dataset = diabetes();

    // Shuffle and split the dataset into training and testing sets
    let mut rng = thread_rng();
    let n_samples = dataset.nsamples();
    let split_idx = (0.8 * n_samples as f32) as usize; // 80% train, 20% test

    // Shuffle the indices
    let mut indices: Vec<usize> = (0..n_samples).collect();
    indices.shuffle(&mut rng);

    // Create the training and testing datasets
    let train_indices = &indices[..split_idx];
    let test_indices = &indices[split_idx..];

    let train = dataset.sample_with_indices(train_indices);
    let test = dataset.sample_with_indices(test_indices);

    // Create a decision tree regressor
    let model = DecisionTree::params().fit(&train).unwrap();

    // Predict on the test dataset
    let prediction = model.predict(&test);

    // Calculate mean squared error
    let mse = prediction.mean_squared_error(&test).unwrap();
    println!("Test mean squared error: {:.2}", mse);
}
