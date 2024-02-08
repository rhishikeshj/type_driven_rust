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
    bound: Option<usize>,
    delims: (char, char),
}

impl<Iter> Progress<Iter> {
    fn new(iter: Iter) -> Self {
        Progress {
            iter,
            i: 0,
            bound: None,
            delims: ('[', ']'),
        }
    }
}

impl<Iter> Progress<Iter>
where
    Iter: ExactSizeIterator,
{
    fn with_bounds(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }

    fn with_delims(mut self, delims: (char, char)) -> Self {
        self.delims = delims;
        self
    }
}

impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        match self.bound {
            Some(bound) => {
                println!(
                    "{}{}{}{}",
                    self.delims.0,
                    "*".repeat(self.i),
                    " ".repeat(bound - self.i),
                    self.delims.1
                )
            }
            None => println!("{}", "*".repeat(self.i)),
        };

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
    let brkts = ('<', '>');

    // progress(v.iter(), expensive_calculation);

    for n in v.iter().progress().with_bounds().with_delims(brkts) {
        expensive_calculation(n);
    }

    // for i in (0..).progress().with_delims(brkts) {
    //     expensive_calculation(&i)
    // }
}
