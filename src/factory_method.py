"""
Factory Method Pattern
======================

"""

from abc import ABC, abstractmethod


# Product
class Pizza(ABC):
    def __init__(self, name, dough, sauce, toppings):
        self.name = name
        self.dough = dough
        self.sauce = sauce
        self.toppings = toppings

    def prepare(self):
        print(f"Preparing {self.name}")
        print("Tossing dough...")
        print("Adding sauce...")
        print("Adding toppings:")
        for topping in self.toppings:
            print("   ", topping)

    def bake(self):
        print("Bake for 25 minutes at 350")

    def cut(self):
        print("Cutting the pizza into diagonal slices")

    def box(self):
        print("Place pizza in official PizzaStore box")


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
        if pizza_type == "cheese":
            return NyStyleCheesePizza()
        if pizza_type == "veggie":
            return NyStyleVeggiePizza()
        else:
            raise ValueError(f"Don't know how to create New York style {pizza_type} pizza")


# Concrete Creator
class ChicagoPizzaStore(PizzaStore):
    def create_pizza(self, pizza_type: str) -> Pizza:
        pizza_type = pizza_type.lower()
        if pizza_type == "cheese":
            return ChicagoStyleCheesePizza()
        if pizza_type == "veggie":
            return ChicagoStyleVeggiePizza()
        else:
            raise ValueError(f"Don't know how to create Chicago style {pizza_type} pizza")


# Concrete Product
class NyStyleCheesePizza(Pizza):
    def __init__(self):
        super().__init__(name="NY Style Sauce and Cheese Pizza",
                         dough="Thin Crust Dough",
                         sauce="Marinara Sauce",
                         toppings=["Grated Reggiano Cheese"])


# Concrete Product
class ChicagoStyleCheesePizza(Pizza):
    def __init__(self):
        super().__init__(name="Chicago Style Deep Dish Cheese Pizza",
                         dough="Extra Thick Crust Dough",
                         sauce="Plum Tomato Sauce",
                         toppings=["Shredded Mozarella Cheese"])

    def cut(self):
        print("Cutting the pizza into square slices")


if __name__ == '__main__':
    ny_store = NyPizzaStore()
    chicago_store = ChicagoPizzaStore()

    pizza = ny_store.order_pizza("cheese")

    pizza = chicago_store.order_pizza("cheese")
