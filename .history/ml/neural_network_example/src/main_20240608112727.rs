use linfa::prelude::*;
use linfa_trees::DecisionTree;
use linfa_datasets::diabetes;

fn main() {
    // Load the Diabetes dataset
    let (train, test) = diabetes();

    // Create a decision tree regressor
    let model = DecisionTree::params().fit(&train).unwrap();

    // Predict on the test dataset
    let prediction = model.predict(&test);

    // Calculate mean squared error
    let mse = prediction.mean_squared_error(&test).unwrap();
    println!("Test mean squared error: {:.2}", mse);
}
