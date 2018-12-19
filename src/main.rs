mod lib;

use std::mem;

fn main() {
    let a:u8 = 0;
    println!("a:u8 \t= {} \tand a size of {} \tbytes", a, mem::size_of_val(&a));

    let b = 's';
    println!("b:char \t= {} \tand a size of {} \tbytes", b, mem::size_of_val(&b));

    let c = "s";
    println!("c:str \t= {} \tand a size of {} \tbytes", c, mem::size_of_val(&c));

    let d = true;
    println!("d:bool \t= {} \tand a size of {} \tbytes", d, mem::size_of_val(&d));

    let e = 5.5;
    println!("e:f64 \t= {} \tand a size of {} \tbytes", e, mem::size_of_val(&e));

    let f:f32 = 5.5;
    println!("f:f32 \t= {} \tand a size of {} \tbytes", f, mem::size_of_val(&f));

    lib::stack_and_heap();
}
