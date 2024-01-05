fn main() {
    let a = true;
    let b = false;
    let c = (a ^ b) | (a & b);

    println!("a is {a} and b is {b}");
    println!("NOT a is {} ", !a);
    println!("a AND b is {} ", a & b);
    println!("a OR b is {} ", a | b);
    println!("a XOR b is {} ", a ^ b);

    println!("c is {c}");
}
