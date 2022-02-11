use std::ops::{Deref};

impl<Data> Observable<Data> {
    pub fn new(value: Data) -> Observable<Data> {
        Observable {
            data: value,
            listeners: Vec::new(),
        }
    }

    ///It takes as an argument a function to change
    /// the value of the data, and then notifies all
    /// listeners. It doesn't matter if the new data
    /// value is different from the previous ones, the
    ///  listeners will still be notified.
    pub fn change(&mut self, change_fn: fn(&mut Data) -> ()) {
        change_fn(&mut self.data);
        self.notify();
    }

    pub fn add_listener(&mut self, listener: fn(&Data) -> ()) {
        self.listeners.push(listener);
    }

    fn notify(&mut self) {
        for listener in self.listeners.iter() {
            listener(&self.data);
        }
    }
}

pub struct Observable<Data> {
    data: Data,
    listeners: Vec<fn(&Data) -> ()>,
}

impl<Data> Deref for Observable<Data> {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<Data: Default> Default for Observable<Data> {
    fn default() -> Self {
        Observable{
            data: Data::default(),
            listeners: Vec::new(),
        }
    }
}