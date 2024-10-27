
fn main() {
    let x = 5;
    let y = type_of(&x);

    println!("{}", y);
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}