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

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
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

    let number_list = vec![1, 2, 3, 4, 5];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
