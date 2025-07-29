use plotters::prelude::*;
use rust_rl::agents::network::nn::{self, NeuralNetwork};

fn main() {
    let mut network = NeuralNetwork::new(
        0.01,
        nn::ActivationFunction::Tanh,
        nn::ActivationFunction::Linear,
        nn::LossFunction::MeanSquaredError,
    );
    network.add_layers(&[1, 32, 32, 64, 1]);

    // Generate 100_000 random points in the range [-5π, 5π]
    let input_data = (0..1000000)
        .map(|_| vec![rand::random_range(-5.0 * std::f64::consts::PI.. 5.0 * std::f64::consts::PI)])
        .collect::<Vec<Vec<f64>>>();
    // Output
    let output_data = sin(&input_data);

    network.train(standardize_input(&input_data), output_data);

    plot_loss(network.get_history()).expect("Failed to plot loss");
    // println!("{:?}", {network.get_history()});
    let step = 0.1;
    let mut test_data = Vec::new();
    let mut x = -5.0 * std::f64::consts::PI;
    while x <= 5.0 * std::f64::consts::PI {
        test_data.push(vec![x]);
        x += step;
    }
    let predictions = network.predict_batch(standardize_input(&test_data.clone()));
    
    plot_sine_approximation(&test_data, &predictions)
        .expect("Failed to plot sine approximation");

}

fn sin(x: &[Vec<f64>]) -> Vec<Vec<f64>> {
    x.iter().map(|v| vec![v[0].sin()]).collect()
}

fn plot_loss(history: &[f64]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("data/plots/loss_plot.png", (1000, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("MSE", ("sans-serif", 50))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            0..history.len() as u32,
            -5.0..5.0,
        )?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        history.iter().enumerate().map(|(i, &v)| (i as u32, v)),
        &RED,
    ))?;

    Ok(())
}

fn plot_sine_approximation(input: &[Vec<f64>], predictions: &[Vec<f64>]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("data/plots/sine_approximation.png", (1000, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sine Function Approximation", ("sans-serif", 50))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            -5.0 * std::f64::consts::PI..5.0 * std::f64::consts::PI,
            -1.5..1.5,
        )?;

    chart.configure_mesh().draw()?;

    // Plot the original sine function
    chart.draw_series(LineSeries::new(
        input.iter().map(|v| (v[0], v[0].sin())),
        &BLUE,
    ))?;

    // Plot the network's predictions
    chart.draw_series(LineSeries::new(
        predictions.iter().enumerate().map(|(i, v)| (input[i][0], v[0])),
        &RED,
    ))?;

    Ok(())
}

fn standardize_input(input: &[Vec<f64>]) -> Vec<Vec<f64>> {
    input.iter().map(|v| vec![(v[0] / (5.0 * std::f64::consts::PI))]).collect()
}