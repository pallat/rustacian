fn main() {
    fibinacci(10);
}

fn fibinacci(_n: i32) {
    let mut f = (0, 1);

    for _i in 0.._n {
        print!("{} ", f.0);
        f = (f.1, f.0 + f.1);
    }
}
