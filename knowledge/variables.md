# Variables
## Mutable
* For a variable to be mutable it needs the argument mut as in let mut

Types can be declared like this:

```rust
let x: i32 
let y: i64 
let w: f32 
let z: bool 
let k: char
```

## Scope
* Is not possible to access local variable scope in superior hierarchies
```rust
fn main(){
    let x: i32 = 10;
    {
        let y: i32 = 5;
    }
    // aqui so e possivel acesar o valor de x porem nao de y
}
```

### Return
* To return a variable from a function the indicator "->" is needed also the type of return as of "String".

* In this case there are two types of string variable and to use String i need to use the method to_string() in the return. return x.to_string()
```rust
// Fix the error with the use of define_x
fn main() {
    let x;
    x = define_x();
    println!("{}, world", x); 
}

fn define_x() -> String {
    let x = "hello";
    return x.to_string()
}
```

### assert_eq!
assert_eq! macro verify if the two statements are equals. The error msg shows the expected value and the current value.
```shell
thread 'main' panicked at src/main.rs:7:9:
assertion `left == right` failed
  left: 12
 right: 5
```

## Shadowing
Is possible to reassert a variable with the same name using let again
```rust
fn main() {
    let x = 5;
    let x = 7;   
    
    println!("{}", x);
}
```
which is different than
```rust
fn main() {
    let x = 5;
    x = 7;

    println!("{}", x);
}
```
giving the output error:
```
2 |     let x = 5;
  |         ^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

error[E0384]: cannot assign twice to immutable variable `x`
```
