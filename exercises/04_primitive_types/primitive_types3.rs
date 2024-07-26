fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [1; 100];
    println!("{}", a.len());
    println!("First: {} Last: {}", a[0], a[a.len()-1]);
    
    for i in a {println!("{}", i)}
    
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
