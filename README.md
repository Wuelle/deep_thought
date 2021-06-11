![crates.io version](https://flat.badgen.net/crates/v/deep_thought)
![crates.io downloads](https://flat.badgen.net/crates/d/deep_thought)

# Deep Thought
This crate implements feedforward-neural Networks in rust.

A basix XOR training example might look like this:
```rust
fn main() -> Result<()>{
    // Build the input and label arrays
    let inputs = array![
        [0., 0.],
        [0., 1.],
        [1., 0.],
        [1., 1.],
    ];
    let labels = array![[0.], [1.], [1.], [0.]];
    
    let dataset = Dataset::new(inputs, labels, 1., BatchSize::One)?;
    let loss_fn = Loss::MSE;

    // Build the neural net
    let mut net = NeuralNetworkBuilder::new()
        .learning_rate(0.3)
        .momentum(0.1)
        .add_layer(Layer::new(2, 3).activation(Activation::Sigmoid))
        .add_layer(Layer::new(3, 3).activation(Activation::Sigmoid))
        .add_layer(Layer::new(3, 1).activation(Activation::Sigmoid));
    
    // train the network
    for epoch in 0..11000 {
        for (samples, labels) in dataset.iter_train().into_iter() {
            let _out = net.forward(&samples);
            if epoch % 100 == 0 {
                println!("training epoch {}", epoch);
                println!("  Loss: {}\n", &loss_fn.compute(&_out, &labels).mean().unwrap());
            }
            net.backprop(samples, labels, &loss_fn);
        }
    }
    
    // evaluate the net 
    let mut total_loss: f64 = 0.;
    // should ofc be iter_test but this dataset is kinda minimalistic
    let test_iter = dataset.iter_train();
    let num_test_samples = test_iter.num_batches * test_iter.batch_size;
    for (sample, label) in test_iter {
        let out = net.forward(&sample);
        total_loss += loss_fn.compute(&out, &label).sum();
        println!("{} == {}", out.map(|&x| x.round()), label);
    }
    
    println!("Mean loss over {} test samples: {:.2}", num_test_samples, total_loss / num_test_samples as f64);
    Ok(())
}
```
For more usage examples, please take a look at [/examples](https://github.com/Wuelle/rust_nn/tree/main/examples)

## Features
* Linear Layers
* Common Activation functions like ReLU or Sigmoid
* Optional Serialization/Deserialization support via the `serde` feature

## TODO
* Add decreasing learning rate (probably also easy)
* add CNN's

## Additional Ressources
Some stuff i found to be quite helpful if you are interested in understanding the math behind neural networks
* [Very nice article by Michael Nielsen](http://neuralnetworksanddeeplearning.com/chap2.html)
