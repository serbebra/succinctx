use std::env;

use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::config::AlgebraicHasher;
use plonky2x::circuit::Circuit;
use plonky2x::function::CircuitFunction;
use plonky2x::prelude::CircuitBuilder;
use plonky2x::vars::Variable;

struct Function {}

impl CircuitFunction for Function {
    fn build<F, C, const D: usize>() -> Circuit<F, C, D>
    where
        F: RichField + Extendable<D>,
        C: plonky2::plonk::config::GenericConfig<D, F = F> + 'static,
        <C as plonky2::plonk::config::GenericConfig<D>>::Hasher: AlgebraicHasher<F>,
    {
        let mut builder = CircuitBuilder::<F, D>::new();
        let a = builder.read::<Variable>();
        let b = builder.read::<Variable>();
        let c = builder.add(a, b);
        builder.write(c);
        builder.build::<C>()
    }
}

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    Function::run();
}
