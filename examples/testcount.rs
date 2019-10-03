extern crate counter;

use counter::counter;

const C: (usize, usize) = (counter!(""), foo());

fn main() {
    dbg!(counter!("mycounter"));
    dbg!(C);
}

const fn foo() -> usize {
    counter!("")
}
