pub type Callback = fn();

pub struct Store<T> {
    listeners: Vec<Callback>,
    pub def: T,
    value: T,
}

impl<T> Store<T> {
    pub fn new(def: T, b: T) -> Self {
        let vec: Vec<Callback> =Vec::new();
        let store: Store<T> = Store {
            def,
            value: b,
            listeners: vec,
        };

        store
    }

    pub fn get(self) -> T {
        self.value
    }

    pub fn set(mut self, v: T) {
        self.value = v;
        self.emmit();
    }

    fn emmit(self) {
        for callback in self.listeners {
            callback();
        }
    }
}
