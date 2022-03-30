// DECLARATIVE MACRO
#[macro_export]
macro_rules! jake {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x * 2);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = jake![0, 2, 4];
    println!("{:?}", v);
}
