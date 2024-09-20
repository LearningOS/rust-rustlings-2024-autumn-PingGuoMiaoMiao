fn main() {
    let mut res = 42;
    let mut option = Some(12);
    
    while let Some(x) = option {
        res += x;
        // Set option to None to exit the loop
        option = None; // or break; if you want to exit immediately
    }
    
    println!("{}", res);
}