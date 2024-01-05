fn main() {
    let a: u8 = 10;
    let b: f32 = 3.0;
    let c: f32 = a as f32 / b;
    println!("c is {0:08.3}\na is {1}\nonce again, c is {0}", c, a);
}
