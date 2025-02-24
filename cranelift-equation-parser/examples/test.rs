use std::io::BufRead;

pub fn main() {
    let mut buffer = String::with_capacity(2048);
    // Lock our standard input to eliminate synchronization overhead (unlocks when dropped)
    let mut stdin = std::io::stdin().lock();

    // Read our first line.
    stdin.read_line(&mut buffer).unwrap();

    cranelift_equation_parser::parse::<f64>(&buffer[..buffer.len() - 1]);
}
