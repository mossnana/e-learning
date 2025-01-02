fn tower_of_hanoi(n: i32, src: i32, des: i32, tmp: i32) {
    if n > 0 {
        tower_of_hanoi(n - 1, src, tmp, des);
        println!("move {src} to {des}");
        tower_of_hanoi(n - 1, tmp, des, src);
    }
}

fn main() {
    println!("Tower of Hanoi");
    tower_of_hanoi(100, 1, 3, 2)
}
