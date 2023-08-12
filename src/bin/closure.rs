use std::ptr::{null, null_mut};

fn test_return<T>(value: T) -> T{
    value
}

fn move_ownership(value: i32){
    let num_ptr = &value as *const i32;
    println!("Move ownership Num {:?} {:?}", value, num_ptr);
}

fn main() {
    let outer_var = 42;

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i     |          i + outer_var  ;
    let closure_test  = || outer_var + 10;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {:?}", closure_inferred(1));

    let one = || 1;
    let test = closure_test();
    println!("closure returning one: {}", one());
    println!("closure test {}", test);
    let value = Some(10);
    let value_ptr: *const Option<i32> = &value as *const Option<i32>;
    let value_empty = None;
    println!("Value (10) -> {} {:?}", value.unwrap_or_else( closure_test), value_ptr);
    println!("Value (null) -> {}", value_empty.unwrap_or_else( closure_test));

    // closure take ownership using `move`
    let move_value_closure = move || value.unwrap() * 10;
    let move_value  = move_value_closure();
    let move_value_ptr : *const i32 = &move_value as *const i32;
    println!("Move value {} {:?}", move_value, move_value_ptr);
    // casting check memory address/pointer
    let value_ptr_2: *const Option<i32> = &value as *const Option<i32>;
    println!("Previous value {:?} {:?}", value, value_ptr_2);
    let x = 10;
    let x_ptr: *const i32 = &x as *const i32;
    println!("Memory address of x: {:?}", x_ptr);

    let test_func = test_return(value);
    let test_func_ptr= &test_func as *const Option<i32>;
    let test_func_2 = test_return(value);
    let test_func_2_ptr= &test_func_2 as *const Option<i32>;
    println!(
        "Test func {:?} {:?}, test func 2 {:?} {:?}", test_func, test_func_ptr,
        test_func_2, test_func_2_ptr
    );

    let num = 1;
    move_ownership(num);
    let num_ptr = &num as *const i32;
    println!("Num {:?} {:?}", num, num_ptr);

}