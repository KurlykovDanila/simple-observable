# Simple Observable

## This simple library is made to notify you when your data changes

 Adding a dependency: `simple-observable = "0.1.1"`

## Base usage example

```rust
use simple_observable::Observable;

fn main() {
    // Create observable with data
    let mut observable = Observable::new(10);

    // Register listeners
    observable.add_listener(my_listener1);
    observable.add_listener(my_listener2);
    
    // Set new data value
    observable.set(17);
}

fn my_listener1(new_value: &i32) {
    println!("New value from listener 1: {}", new_value);
}

fn my_listener2(new_value: &i32) {
    println!("New value from listener 2: {}", new_value);
}
```

 It is important to understand that for data that does not implement the Copy trait, the Observable becomes the owner. Also listeners can only get read refs.

## Changing observed values

```rust
use simple_observable::Observable;

fn main() {

    let data = vec![1, 2, 3];
    // Create observable with data
    let mut observable = Observable::new(data);

    // Register listeners
    observable.add_listener(my_listener1);
    observable.add_listener(my_listener2);
    
    // observable.set(vec![1, 2, 3, 4]);
    // Irrational use, as the value will simply be overwritten, 
    // which leads to overhead


    // Use instead
    (*observable).push(4);
    
    // Don't forget to notify us of changes
    observable.notify();
    
}

fn my_listener1(new_value: &Vec<i32>) {
    println!("New value from listener 1: {:?}", new_value);
}

fn my_listener2(new_value: &Vec<i32>) {
    println!("New value from listener 2: {:?}", new_value);
}
```

`Observable` is a pointer that can be dereferenced and the data can be accessed directly. However, when they change, there will be no automatic notification, you need to call `notify()` manually
