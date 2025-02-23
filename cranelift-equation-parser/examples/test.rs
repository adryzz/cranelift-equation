pub fn main() {
    let equation = "3x + 4 * (54 - cos(3))";
    cranelift_equation_parser::parse::<f64>(equation);
}