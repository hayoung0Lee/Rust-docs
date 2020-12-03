use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age: u8
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} + {}", self.real, self.imag)
    }
}

fn main() {
    // println!("Hello World!");
    // eprintln!("I'm a Rustacean!");
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 4);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);
    println!("My name is {0}, {1}, {0}", "Bond", "James");
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);
    println!("This struct `{:?}` won't print...", Structure(3));
    println!("Pis is roughly {number:.prec$}", prec=3, number=3.141592);

    let name="Peter";
    let age= 27;
    let peter= Person {name, age};
    println!("{:#?}", peter);

    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);
    println!("Display: {:?}", minmax);

    let c = Complex { real: 3.3, imag: 7.2 };
    println!("Display: {:?}", c);
}