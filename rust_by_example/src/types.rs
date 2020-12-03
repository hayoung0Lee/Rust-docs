use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{:?}\n{:?}", (self.0, self.1), (self.2, self.3))
        // write!(f, "{} {} {} {}", self.0, self.1, self.2, self.3)
    }
}

fn transpose(data: Matrix)-> Matrix {    
    Matrix(data.0, data.2, data.1, data.3)
}


fn main(){
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Matrix:\n{}", transpose(matrix));

    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple first value: {}", long_tuple.1);
    println!("tuple of tuples: {:?}", long_tuple);

    println!("{:?}", (5u32,));
    println!("{:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a,b,c,d) = tuple;

    println!("{:?}, {:?},{:?}, {:?}", a, b, c, d);

    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    println!("{} {}", a_float, an_integer);
    println!("One million is written as {}", 1_000_000u32);
}