//! Factory Metod Pattern
//! =====================
//!

trait Pizza {
    fn name(&self) -> &'static str;
    fn dough(&self) -> &'static str;
    fn sauce(&self) -> &'static str;
    fn toppings(&self) -> Vec<&'static str>;

    fn prepare(&mut self) {
        println!("Preparing {}", self.name());
        println!("Tossing dough...");
        println!("Adding sauce...");
        println!("Adding toppings:");
        for topping in self.toppings() {
            println!("    {}", topping);
        }
    }

    fn bake(&mut self) {
        println!("Bake for 25 minutes at 350")
    }

    fn cut(&mut self) {
        println!("Cutting the pizza into diagonal slices")
    }

    fn boxify(&mut self) {
        println!("Place pizza in official PizzaStore box")
    }
}

trait PizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza>;

    fn order_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        let mut pizza = self.create_pizza(pizza_type);

        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.boxify();

        pizza
    }
}

struct NyPizzaStore;

impl PizzaStore for NyPizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        match pizza_type {
            "cheese" => Box::new(NyStyleCheesePizza::new()),
            _ => panic!(
                "Don't know how to create New York style {} pizza",
                pizza_type
            ),
        }
    }
}

struct ChicagoPizzaStore;

impl PizzaStore for ChicagoPizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        match pizza_type {
            "cheese" => Box::new(ChicagoStyleCheesePizza::new()),
            _ => panic!(
                "Don't know how to create Chicago style {} pizza",
                pizza_type
            ),
        }
    }
}

struct NyStyleCheesePizza;

impl NyStyleCheesePizza {
    fn new() -> Self {
        Self
    }
}

impl Pizza for NyStyleCheesePizza {
    fn name(&self) -> &'static str {
        "NY Style Sauce and Cheese Pizza"
    }

    fn dough(&self) -> &'static str {
        "Thin Crust Dough"
    }

    fn sauce(&self) -> &'static str {
        "Marinara Sauce"
    }

    fn toppings(&self) -> Vec<&'static str> {
        vec!["Grated Reggiano Cheese"]
    }
}

struct ChicagoStyleCheesePizza;

impl ChicagoStyleCheesePizza {
    fn new() -> Self {
        Self
    }
}

impl Pizza for ChicagoStyleCheesePizza {
    fn name(&self) -> &'static str {
        "Chicago Style Deep Dish Cheese Pizza"
    }

    fn dough(&self) -> &'static str {
        "Extra Thick Crust Dough"
    }

    fn sauce(&self) -> &'static str {
        "Plum Tomato Sauce"
    }

    fn toppings(&self) -> Vec<&'static str> {
        vec!["Shredded Mozarella Cheese"]
    }
}

#[test]
fn test_run() {
    let ny_store = NyPizzaStore;
    let chicago_store = ChicagoPizzaStore;

    ny_store.order_pizza("cheese");
    chicago_store.order_pizza("cheese");
}
