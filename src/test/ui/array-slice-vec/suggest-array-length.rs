// run-rustfix
#![allow(unused_variables, dead_code, non_upper_case_globals)]

fn main() {
    const Foo: [i32; _] = [1, 2, 3];
    //~^ ERROR in expressions, `_` can only be used on the left-hand side of an assignment
    //~| ERROR using `_` for array lengths is unstable
    let foo: [i32; _] = [1, 2, 3];
    //~^ ERROR in expressions, `_` can only be used on the left-hand side of an assignment
    //~| ERROR using `_` for array lengths is unstable
    let bar: [i32; _] = [0; 3];
    //~^ ERROR in expressions, `_` can only be used on the left-hand side of an assignment
    //~| ERROR using `_` for array lengths is unstable
}
