"""
Factory Method Pattern
======================

"""

from abc import ABC, abstractmethod


# Product
class Pizza(ABC):
    def __init__(self, name):
        self.name = name
        self.dough = None
        self.sauce = None
        self.veggies = []
        self.cheese = None
        self.pepperoni = None
        self.clam = None

    @abstractmethod
    def prepare(self): pass

    def bake(self):
        print("Bake for 25 minutes at 350")

    def cut(self):
        print("Cutting the pizza into diagonal slices")

    def box(self):
        print("Place pizza in official PizzaStore box")


class PizzaIngredientFactory(ABC):
    @abstractmethod
    def create_dough(self): pass

    @abstractmethod
    def create_sauce(self): pass

    @abstractmethod
    def create_cheese(self): pass

    @abstractmethod
    def create_veggies(self): pass

    @abstractmethod
    def create_pepperoni(self): pass

    @abstractmethod
    def create_clam(self): pass


class NYPizzaIngredientFactory(PizzaIngredientFactory):
    def create_dough(self): return "Thin Crust Dough"

    def create_sauce(self): return "Marinara Sauce"

    def create_cheese(self): return "Reggiano Cheese"

    def create_veggies(self): return ["Garlic", "Onion", "Mushroom", "RedPepper"]

    def create_pepperoni(self): return "Sliced Pepperoni"

    def create_clam(self): return "Fresh Clams"


class ChicagoIngredientFactory(PizzaIngredientFactory):
    def create_dough(self): return "Thick Crust Dough"

    def create_sauce(self): return "Plum Tomato Sauce"

    def create_cheese(self): return "Shredded Mozzarella Cheese"

    def create_veggies(self): return ["Spinach", "Black Olives", "Eggplant"]

    def create_pepperoni(self): return "Sliced Pepperoni"

    def create_clam(self): return "Frozen Clams"


class CheesePizza(Pizza):
    def __init__(self, ingredient_factory: PizzaIngredientFactory):
        super().__init__("Cheese Pizza")
        self.ingredient_factory = ingredient_factory

    def prepare(self):
        print("Preparing ", self.name)
        self.dough = self.ingredient_factory.create_dough()
        self.sauce = self.ingredient_factory.create_sauce()
        self.cheese = self.ingredient_factory.create_cheese()


class ClamPizza(Pizza):
    def __init__(self, ingredient_factory: PizzaIngredientFactory):
        super().__init__("Clam Pizza")
        self.ingredient_factory = ingredient_factory

    def prepare(self):
        print("Preparing ", self.name)
        self.dough = self.ingredient_factory.create_dough()
        self.sauce = self.ingredient_factory.create_sauce()
        self.cheese = self.ingredient_factory.create_cheese()
        self.clam = self.ingredient_factory.create_clam()


# Creator
class PizzaStore(ABC):
    @abstractmethod
    def create_pizza(self, pizza_type: str) -> Pizza:
        pass

    def order_pizza(self, pizza_type: str) -> Pizza:
        pizza = self.create_pizza(pizza_type)

        pizza.prepare()
        pizza.bake()
        pizza.cut()
        pizza.box()

        return pizza


# Concrete Creator
class NyPizzaStore(PizzaStore):
    def create_pizza(self, pizza_type: str) -> Pizza:
        pizza_type = pizza_type.lower()
        ingredient_factory = NYPizzaIngredientFactory()
        if pizza_type == "cheese":
            pizza = CheesePizza(ingredient_factory)
        elif pizza_type == "clam":
            pizza = ClamPizza(ingredient_factory)
        else:
            raise ValueError(f"Don't know how to create New York style {pizza_type} pizza")
        pizza.name = "Now York Style" + pizza.name
        return pizza


# Concrete Creator
class ChicagoPizzaStore(PizzaStore):
    def create_pizza(self, pizza_type: str) -> Pizza:
        pizza_type = pizza_type.lower()
        ingredient_factory = ChicagoIngredientFactory()
        if pizza_type == "cheese":
            pizza = CheesePizza(ingredient_factory)
        elif pizza_type == "clam":
            pizza = ClamPizza(ingredient_factory)
        else:
            raise ValueError(f"Don't know how to create Chicago style {pizza_type} pizza")
        pizza.name = "Chicago Style" + pizza.name
        return pizza


if __name__ == '__main__':
    ny_store = NyPizzaStore()
    chicago_store = ChicagoPizzaStore()

    pizza = ny_store.order_pizza("cheese")

    pizza = chicago_store.order_pizza("cheese")
