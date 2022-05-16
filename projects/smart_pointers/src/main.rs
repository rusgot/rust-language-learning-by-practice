use crate::ListBox::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

enum ListBox {
    Cons(i32, Box<ListBox>),
    Nil,
}

#[derive(Debug)]
enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Drop trait stuff
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("{} is stored on the heap!", b);

    // Box version
    let list_box = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Rc version
    let list_a = Rc::new(ListRc::Cons(
        5,
        Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil))),
    ));
    println!(
        "Count after creating list_a = {}",
        Rc::strong_count(&list_a)
    );
    let list_b = ListRc::Cons(3, Rc::clone(&list_a));
    println!(
        "Count after creating list_b = {}",
        Rc::strong_count(&list_a)
    );
    {
        let list_c = ListRc::Cons(4, Rc::clone(&list_a));
        println!("{:?}", list_c);
        println!(
            "Count after creating list_c = {}",
            Rc::strong_count(&list_a)
        );
    }
    println!(
        "Count after list_c goes out of scope = {}",
        Rc::strong_count(&list_a)
    );

    println!("{:?}", list_a);
    println!("{:?}", list_b);

    // `y` points to the value of `x`
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // `a` is copied and put on the heap and `b` points to it
    let a = 7;
    let b = Box::new(a);
    assert_eq!(7, a);
    assert_eq!(7, *b);

    // Custom Box example
    let c = 8;
    let d = MyBox::new(c);
    assert_eq!(8, c);
    assert_eq!(8, *d);

    let m = MyBox::new(String::from("Jake"));
    hello(&m);

    // Drop trait stuff
    let drop_test_1 = CustomSmartPointer {
        data: String::from("This is a String!"),
    };
    let drop_test_2 = CustomSmartPointer {
        data: String::from("Another String!!!"),
    };
    println!("CustomSmartPointers created!");
    drop(drop_test_2);
    println!("drop_test_2 dropped manually...");
    println!("-----Out of scope-----");

    let x1 = Box::new("hello");
    // let x2 = x1.clone();
    println!("x1: {:p}, x2: {:p}", x1, x1.clone());
    assert_eq!(x1, x1);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
