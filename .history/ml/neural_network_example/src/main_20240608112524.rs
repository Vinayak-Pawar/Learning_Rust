use linfa::prelude::*;
use linfa_trees::DecisionTree;
use linfa_datasets::iris;

fn main() {
    // Load the Iris dataset
    let (train, test) = iris();

    // Create a decision tree classifier
    let model = DecisionTree::params().fit(&train).unwrap();

    // Predict on the test dataset
    let prediction = model.predict(&test);

    // Calculate accuracy
    let accuracy = prediction.confusion_matrix(&test).unwrap().accuracy();
    println!("Test accuracy: {:.2}%", accuracy * 100.0);
}
