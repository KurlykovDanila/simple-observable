use simple_observable::Observable;

fn main() {
    let a = 1;
    println!("Initial value: {}", a);
    let mut obs1 = Observable::new(a);
    obs1.add_listener(listener1);
    obs1.add_listener(listener2);
    obs1.add_listener(listener3);
    obs1.change(my_change_function);
}

fn my_change_function(num: &mut i32) {
    *num += 1;
}

fn listener1(num: &i32) {
    println!("(listener1) New value after change: {}", num);
}

fn listener2(num: &i32) {
    println!("(listener2) New value after change: {}", num);
}

fn listener3(num: &i32) {
    println!("(listener3) New value after change: {}", num);
}