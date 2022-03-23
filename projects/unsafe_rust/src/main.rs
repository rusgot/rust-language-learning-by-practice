static HELLO_WORLD: &str = "Hello world!";
static mut COUNTER: u32 = 0;

fn main() {
    // How to create an immutable and a mutable raw pointer from references
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("{:?}", *r1);
        println!("{:?}", *r2);
    }

    unsafe {
        dangerous();
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("Name is {}", HELLO_WORLD);

    inc(5);
    unsafe {
        println!("COUNTER is now {}", COUNTER);
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe fn dangerous() {}

fn inc(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // Methods go here
}

unsafe impl Foo for i32 {
    // Method implementations go here
}
