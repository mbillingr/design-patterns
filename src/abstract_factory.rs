//! Abstract Factory Pattern
//! ========================
//!

trait Pizza {
    fn set_name(&mut self, s: String);
    fn name(&self) -> &str;

    fn prepare(&mut self);

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

trait PizzaIngredientFactory {
    fn create_dough(&self) -> &'static str;
    fn create_sauce(&self) -> &'static str;
    fn create_veggies(&self) -> Vec<&'static str>;
    fn create_cheese(&self) -> &'static str;
    fn create_pepperoni(&self) -> &'static str;
    fn create_clam(&self) -> &'static str;
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

struct NyPizzaIngredientFactory;

impl PizzaIngredientFactory for NyPizzaIngredientFactory {
    fn create_dough(&self) -> &'static str {
        "Thin Crust Dough"
    }
    fn create_sauce(&self) -> &'static str {
        "Marinara Sauce"
    }
    fn create_veggies(&self) -> Vec<&'static str> {
        vec!["Garlic", "Onion", "Mushroom", "RedPepper"]
    }
    fn create_cheese(&self) -> &'static str {
        "Reggiano Cheese"
    }
    fn create_pepperoni(&self) -> &'static str {
        "Sliced Pepperoni"
    }
    fn create_clam(&self) -> &'static str {
        "Fresh Clams"
    }
}

struct ChicagoPizzaIngredientFactory;

impl PizzaIngredientFactory for ChicagoPizzaIngredientFactory {
    fn create_dough(&self) -> &'static str {
        "Thick Crust Dough"
    }
    fn create_sauce(&self) -> &'static str {
        "Plum Tomato Sauce"
    }
    fn create_veggies(&self) -> Vec<&'static str> {
        vec!["Spinach", "Black Olives", "Eggplant"]
    }
    fn create_cheese(&self) -> &'static str {
        "Shredded Mozzarella Cheese"
    }
    fn create_pepperoni(&self) -> &'static str {
        "Sliced Pepperoni"
    }
    fn create_clam(&self) -> &'static str {
        "Frozen Clams"
    }
}

struct CheesePizza {
    name: String,
    ingredient_factory: Box<dyn PizzaIngredientFactory>,
}

impl CheesePizza {
    pub fn new(ingredient_factory: impl 'static + PizzaIngredientFactory) -> Self {
        Self {
            name: "Cheese Pizza".to_string(),
            ingredient_factory: Box::new(ingredient_factory),
        }
    }
}

impl Pizza for CheesePizza {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn prepare(&mut self) {
        println!("Preparing {}", self.name);
        println!("  kneading {}", self.ingredient_factory.create_dough());
        println!("  smearing {}", self.ingredient_factory.create_sauce());
        println!("  spreading {}", self.ingredient_factory.create_cheese());
    }
}

struct ClamPizza {
    name: String,
    ingredient_factory: Box<dyn PizzaIngredientFactory>,
}

impl ClamPizza {
    pub fn new(ingredient_factory: impl 'static + PizzaIngredientFactory) -> Self {
        Self {
            name: "Clam Pizza".to_string(),
            ingredient_factory: Box::new(ingredient_factory),
        }
    }
}

impl Pizza for ClamPizza {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn prepare(&mut self) {
        println!("Preparing {}", self.name);
        println!("  kneading {}", self.ingredient_factory.create_dough());
        println!("  smearing {}", self.ingredient_factory.create_sauce());
        println!("  spreading {}", self.ingredient_factory.create_cheese());
        println!("  placing {}", self.ingredient_factory.create_clam());
    }
}

struct NyPizzaStore;

impl PizzaStore for NyPizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        let ingredient_factory = NyPizzaIngredientFactory;
        let mut pizza: Box<dyn Pizza> = match pizza_type {
            "cheese" => Box::new(CheesePizza::new(ingredient_factory)),
            "clam" => Box::new(ClamPizza::new(ingredient_factory)),
            _ => panic!(
                "Don't know how to create New York style {} pizza",
                pizza_type
            ),
        };
        pizza.set_name(format!("New York Style {}", pizza.name()));
        pizza
    }
}

struct ChicagoPizzaStore;

impl PizzaStore for ChicagoPizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        let ingredient_factory = ChicagoPizzaIngredientFactory;
        let mut pizza: Box<dyn Pizza> = match pizza_type {
            "cheese" => Box::new(CheesePizza::new(ingredient_factory)),
            "clam" => Box::new(ClamPizza::new(ingredient_factory)),
            _ => panic!(
                "Don't know how to create Chicago style {} pizza",
                pizza_type
            ),
        };
        pizza.set_name(format!("New Chicago Style {}", pizza.name()));
        pizza
    }
}

#[test]
fn test_run() {
    let ny_store = NyPizzaStore;
    let chicago_store = ChicagoPizzaStore;

    ny_store.order_pizza("cheese");
    chicago_store.order_pizza("clam");
}
