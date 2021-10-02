<!--
Unfortunately, GitHub refuses to allow user mentions within READMEs. https://github.com/github/markup/issues/209
Thanks to a custom workflow, its still possible
-->
![crates.io version](https://flat.badgen.net/crates/v/deep_thought)
![crates.io downloads](https://flat.badgen.net/crates/d/deep_thought)

# Deep Thought
This crate implements feedforward-neural Networks in rust.
Unlike the vast majority of NN libraries, like pytorch or tensorflow, deep thought does not use backpropagation to calculate gradients.
Instead, it utilizes Dual Numbers, which allow for the calculation of derivatives during the forward pass.

There is still a long way to go before this crate will become usable (if ever). 

For more information about Dual Numbers, please take a look at these excellent Blogposts by <a class="user-mention" data-hovercard-type="user" data-hovercard-url="/users/Atrix256/hovercard" href="/Atrix256">@Atrix256</a>:
* [Dual Numbers & Automatic Differentiation](https://blog.demofox.org/2014/12/30/dual-numbers-automatic-differentiation/)
* [Multivariable Dual Numbers & Automatic Differentiation](https://blog.demofox.org/2017/02/20/multivariable-dual-numbers-automatic-differentiation/)
* [Neural Network Gradients: Backpropagation, Dual Numbers, Finite Differences](https://blog.demofox.org/2017/03/13/neural-network-gradients-backpropagation-dual-numbers-finite-differences/)

If you want to learn more about the math behind neural networks, take a look at these links:
* ["How the backpropagation algorithm works" by Michael Nielsen](http://neuralnetworksanddeeplearning.com/chap2.html)

## Nightly Usage
Deep_thought makes use of the `negative_impls`, `auto_traits` and `array_zip` features, which are not available on the stable release channel yet.

<a class="user-mention" data-hovercard-type="user" href="/Wuelle" data-hovercard-url="/users/Wuelle/hovercard">Wuelle</a>
<a class="user-mention" data-hovercard-type="user" href="/Wuelle" data-hovercard-url="/users/Wuelle/hovercard">Wuelle</a>
