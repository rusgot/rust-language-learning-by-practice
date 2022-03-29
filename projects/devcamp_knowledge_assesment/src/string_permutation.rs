// Name: Jake Edwards
// Email: edwardsjake@live.com
// Just pasting the whole program for each answer as they are required fields

fn main() {
    let string = "big";
    let s: Vec<char> = String::from(string).chars().collect();
    // println!("{:?}", s);
    let start_index = 0;
    let end_index = s.len() - 1;

    println!("Permutations of {}\n---------------------", string);
    recursive_permutation(s.clone(), start_index, end_index);
}

fn recursive_permutation(s: Vec<char>, start_index: usize, end_index: usize) {
    if start_index == end_index {
        let str_s: String = s.into_iter().collect();
        println!("{}", str_s);
    } else {
        let mut v = s;
        for i in start_index..end_index + 1 {
            // shuffle
            let temp = v[i];
            v[i] = v[start_index];
            v[start_index] = temp;
            recursive_permutation(v.clone(), start_index + 1, end_index);
        }
    }
}
