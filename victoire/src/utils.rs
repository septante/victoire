use rand::prelude::*;
use std::collections::VecDeque;

pub fn shuffle<T>(v: &mut VecDeque<T>) {
    shuffle_with_rng(v, &mut rand::thread_rng())
}

fn shuffle_with_rng<T>(v: &mut VecDeque<T>, rng: &mut impl Rng) {
    v.make_contiguous().shuffle(rng);
}
