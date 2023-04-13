use std::ops::Add;

struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    // TODO 1 - implement run
    fn new(real: f64, imaginary: f64) -> Complex {
        Complex {
            real, 
            imaginary
        }
    }
}

// TODO 2 - implement the Add, Sub, Mul and Div traits

impl Add for Complex {
    type Output = Self;

    fn add(self, other : Self) -> Self {
        Self {
            imaginary : self.imaginary + other.imaginary,
            real : self.real + other.real,
        }
    }

}



pub fn run() {
    let n1 = Complex::new(2.0, 3.0);
    let n2 = Complex::new(-2.0, 3.0);
    let n3 = Complex::new(2.0, -3.0);
    let n4 = Complex::new(3.0, 0.0);
    let n5 = Complex::new(0.0, 3.0);
    let n7 = Complex::new(2.0, 3.0);
    // println!("The number is {}", n1); // prints 2+3i
    // println!("The number is {}", n2); // prints -2+3i
    // println!("The number is {}", n3); // prints 2-3i
    // println!("The number is {}", n4); // prints 3
    // println!("The number is {}", n5); // prints 3i

    // println!("The number is {}", n1 - n7); // prints 0
    // println!("The number is {}", n1 + n2);
    // println!("The number is {}", n1 - n2);

    // println!(
    //     "The numbers {} and {} are {}",
    //     n1,
    //     n2,
    //     if n1 == n2 { "equal" } else { "not equal" }
    // );

    // println!("The number is {}", n1 < n2);
    // println!("The number is {}", n1 <= n2);
    // println!("The number is {}", n1 > n2);
    // println!("The number is {}", n1 >= n2);
}