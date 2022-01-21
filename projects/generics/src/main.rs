struct Nums<T> {
    num_1: T,
    num_2: T,
}

impl<T> Nums<T> {
    fn num_1(&self) -> &T {
        &self.num_1
    }

    fn num_2(&self) -> &T {
        &self.num_2
    }
}

impl Nums<usize> {
    fn product(&self) -> usize {
        &self.num_1 * &self.num_2
    }
}

impl Nums<f64> {
    fn product(&self) -> f64 {
        &self.num_1 * &self.num_2
    }
}

fn main() {
    let p1 = Nums { num_1: 3, num_2: 2 };

    let p2 = Nums {
        num_1: 2.5,
        num_2: 5.0,
    };

    println!("p1 num_1: {}", p1.num_1());
    println!("p1 num_2: {}", p1.num_2());
    println!("p1 product: {}\n", p1.product());

    println!("p2 num_2: {}", p2.num_2());
    println!("p2 num_2: {}", p2.num_2());
    println!("p2 product: {}", p2.product());
}
