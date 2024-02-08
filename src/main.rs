use std::collections::HashSet;
use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn progress<Iter>(iter: Iter, f: fn(Iter::Item) -> ())
where
    Iter: Iterator,
{
    let mut i = 1;
    for n in iter {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        f(n)
    }
}

struct Progress<Iter> {
    iter: Iter,
    i: usize,
}

impl<Iter> Progress<Iter> {
    fn new(iter: Iter) -> Self {
        Progress { iter, i: 0 }
    }
}

impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;

        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn main() {
    let v = vec![1, 2, 3];

    // progress(v.iter(), expensive_calculation);

    for n in v.iter().progress() {
        expensive_calculation(n);
    }
}
