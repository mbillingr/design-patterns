//! Observer Pattern
//! ================
//!
//! The first question popping up when writing the traits (interfaces) for this
//! pattern in Rust is *who owns the observer?* Immediately followed by *what
//! about mutability?* And *Oh no, lifetimes!*
//! I think the Subject should *not* own the observer. The observer exist on
//! their own and they happen just to be interested in a subject. One observer
//! might even be subscribed to multiple subjects.
//! This suggests either shared ownership or borrowing. Either one prevents
//! mutability, so we will need interior mutability.
//! Assume that the Subject must outlive any registered observers, but does not
//! need to outlive observers that have been unregistered. I don't think Rust's
//! lifetimes can model this behavior, so we're limited to shared ownership.
//! The subject could hold an owning reference (`Rc` or `Arc`) to the
//! observers. However, this would keep observers alive that have not been
//! explicitly removed, even if they are no longer used elsewhere in the
//! program. Thus, a weak reference seems more appropriate.
//!
//! Making Subject and Observer generic over the state allows concrete
//! subjects/observers to implement these interfaces for different types.
//!

use std::sync::{Arc, Weak};

trait Subject<T> {
    fn register_observer(&mut self, observer: &Arc<dyn Observer<T>>);
    fn unregister_observer(&mut self, observer: &Arc<dyn Observer<T>>);
    fn notify_observers(&mut self);
    fn get_state(&self) -> &T;
}

trait Observer<T> {
    fn update(&self, subject: &dyn Subject<T>);
}

#[derive(Default)]
struct ExampleSubject {
    observers: Vec<Weak<dyn Observer<i32>>>,
    state: i32,
}

impl ExampleSubject {
    fn set_state(&mut self, state: i32) {
        self.state = state;
        self.notify_observers();
    }
}

impl Subject<i32> for ExampleSubject {
    fn register_observer(&mut self, observer: &Arc<dyn Observer<i32>>) {
        self.observers.push(Arc::downgrade(observer))
    }

    fn unregister_observer(&mut self, observer: &Arc<dyn Observer<i32>>) {
        let observer = Arc::downgrade(observer);
        if let Some(idx) = self
            .observers
            .iter()
            .position(|obs| Weak::ptr_eq(obs, &observer))
        {
            self.observers.swap_remove(idx);
        }
    }

    fn notify_observers(&mut self) {
        let mut idx = 0;
        while idx < self.observers.len() {
            if let Some(observer) = self.observers[idx].upgrade() {
                observer.update(self);
                idx += 1;
            } else {
                self.observers.swap_remove(idx);
            }
        }
    }

    fn get_state(&self) -> &i32 {
        &self.state
    }
}

#[derive(Default)]
struct ExampleObserver {
    history: std::cell::RefCell<Vec<i32>>,
}

impl Observer<i32> for ExampleObserver {
    fn update(&self, subject: &dyn Subject<i32>) {
        self.history.borrow_mut().push(*subject.get_state());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn working_example() {
        let mut subject = ExampleSubject::default();
        let obs1 = Arc::new(ExampleObserver::default());
        let obs2 = Arc::new(ExampleObserver::default());

        let dyn1: Arc<dyn Observer<_>> = obs1.clone();
        let dyn2: Arc<dyn Observer<_>> = obs2.clone();

        subject.set_state(1);
        subject.register_observer(&dyn1);
        subject.set_state(2);
        subject.register_observer(&dyn2);
        subject.set_state(3);
        subject.unregister_observer(&dyn1);
        subject.set_state(4);

        assert_eq!(&*obs1.history.borrow(), &vec![2, 3]);
        assert_eq!(&*obs2.history.borrow(), &vec![3, 4]);

        drop(obs2);
        drop(dyn2);
        subject.set_state(5);

        // no observers left although one of them was not explicitly removed
        assert_eq!(subject.observers.len(), 0);
    }

    /* does not compile
    #[test]
    fn dynamic_lifetime() {
        struct Sub<'o> {
            observers: Vec<&'o Obs>,
        }

        struct Obs { }

        let mut sub = Sub { observers: vec![] };

        {
            let obs = Obs { };
            sub.observers.push(&obs);
            sub.observers.pop();
            // obs is no longer subscribed here but the compiler does not know that
        }

        let obs2 = Obs { };
        sub.observers.push(&obs2);
    }*/
}
