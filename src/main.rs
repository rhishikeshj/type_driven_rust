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

struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char),
}

struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound,
}

impl<Iter> Progress<Iter, Unbounded> {
    fn new(iter: Iter) -> Self {
        Progress {
            iter,
            i: 0,
            bound: Unbounded,
        }
    }
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "*".repeat(progress.i))
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            "*".repeat(progress.i),
            " ".repeat(self.bound - progress.i),
            self.delims.1
        );
    }
}

impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    fn with_bounds(self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        Progress {
            iter: self.iter,
            i: self.i,
            bound,
        }
    }
}

impl<Iter> Progress<Iter, Bounded> {
    fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where
    Iter: Iterator,
    Bound: ProgressDisplay,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        self.bound.display(&self);
        self.i += 1;

        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self, Unbounded> {
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
