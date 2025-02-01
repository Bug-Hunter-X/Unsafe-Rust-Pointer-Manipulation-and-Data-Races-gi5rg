fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of unsafe operations, we use safe methods
    v[0] = 10; 
    println!(" {:?}", v);
} 