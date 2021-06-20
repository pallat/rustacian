fn main() {
    println!("Hello, 🦀");
    daimond(10)
}

fn daimond(_n: i32) {

    for x in (0.._n).rev() {
        let x = x as usize;
        println!("{}{}"," ".repeat(x), "*".repeat((_n as usize * 2)-(2 * x) -1));
        
    }
    
    for x in 1.._n {
        let x = x as usize;
        println!("{}{}"," ".repeat(x), "*".repeat((_n as usize * 2)-(2 * x) -1));
    }

}