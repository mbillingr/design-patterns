//! Strategy Pattern
//! ================
//!

trait QuackBehavior: 'static {
    fn do_quack(&mut self);
}

struct Duck {
    kind: &'static str,
    quack_behavior: Box<dyn QuackBehavior>,
}

impl Duck {
    pub fn new(kind: &'static str, quack_behavior: impl QuackBehavior) -> Self {
        Duck {
            kind,
            quack_behavior: Box::new(quack_behavior),
        }
    }

    pub fn quack(&mut self) {
        self.quack_behavior.do_quack()
    }
}

struct Quack;
impl QuackBehavior for Quack {
    fn do_quack(&mut self) {
        println!("quack")
    }
}

struct Squeak;
impl QuackBehavior for Squeak {
    fn do_quack(&mut self) {
        println!("squeak")
    }
}

#[test]
fn duck_concert() {
    let mut ducks = vec![
        Duck::new("Ancona Duck", Quack),
        Duck::new("Rubber Duck", Squeak),
    ];

    for duck in &mut ducks {
        duck.quack();
        duck.quack();
    }
}
