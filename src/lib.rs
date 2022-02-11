use std::ops::{Deref, DerefMut};

impl<Data> Observable<Data> {
    pub fn new(value: Data) -> Observable<Data> {
        Observable {
            data: value,
            listeners: Vec::new(),
        }
    }

    pub fn change(&mut self, change_fn: fn(&mut Data) -> ()) {
        change_fn(&mut self.data);
        self.notify();
    }

    pub fn add_listener(&mut self, listener: fn(&Data) -> ()) {
        self.listeners.push(listener);
    }

    pub fn notify(&mut self) {
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
