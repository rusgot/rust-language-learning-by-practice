fn main() {
    // Create immutable i32 vector
    let v = vec![1, 2, 3, 4, 5];

    // Iterate through a vector
    // Use a for loop to get immutable3 references to get each element in a vector of i32 values
    for i in &v {
        println!("{}", i);
    }

    // Create mutable i32 vector
    let mut v = vec![5, 4, 3, 2, 1];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // Store different types in vector by creating an enum
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row.get(0));
}
