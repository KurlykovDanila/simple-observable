use std::ops::{Deref, DerefMut};

impl<T> Observable<T> {
    pub fn new(value: T) -> Observable<T> {
        Observable {
            data: value,
            listeners: Vec::new(),
        }
    }

    pub fn set(&mut self, new_value: T) {
        self.data = new_value;
        self.notify();
    }

    pub fn add_listener(&mut self, listener: fn(&T) -> ()) {
        self.listeners.push(listener);
    }

    pub fn notify(&mut self) {
        for listener in self.listeners.iter() {
            listener(&self.data);
        }
    }
}

pub struct Observable<T> {
    data: T,
    listeners: Vec<fn(&T) -> ()>,
}

impl<T> Deref for Observable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Observable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
