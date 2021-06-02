"""
Singleton Pattern
=================

Of course, there is no true singleton in Python, but this is as close
as one gets with reasonable effort.

The main idea of the Singleton pattern is to make the constructor
private, which is not possible in Python. Instead, we can modify the
constructor to always return the same object. However, in the simple
case it's easy to reset the "private" instance variable and get
another instance of the class.

An alternative is to hide the instance variable in a closure.
Although this requires more boilerplate it would take far more
introspection effort to hack the instance variable.

The most Pythonic approach would probably be to use a metaclass. To
redefine what it means for a class to be a singleton. This also
creates a nice separation of concern ("singleton-ness" vs "class logic").
It's also possible to wrap the meta class in a closure to hide the
instance variables.
"""


class SimpleSingleton:
    _the_instance = None

    def __new__(cls):
        if cls._the_instance is None:
            cls._the_instance = super().__new__(cls)
        return cls._the_instance


def singleton():
    the_instance = None

    class Singleton:
        def __new__(cls):
            nonlocal the_instance
            if the_instance is None:
                the_instance = super().__new__(cls)
            return the_instance

    return Singleton


SaferSingleton = singleton()


def make_singleton_meta():
    instances = {}

    class SingletonMetaClass(type):

        def __call__(cls, *args, **kwargs):
            try:
                return instances[cls]
            except KeyError:
                instance = super().__call__(*args, **kwargs)
                instances[cls] = instance
                return instance

    return SingletonMetaClass


SingletonMetaClass = make_singleton_meta()


class MetaSingleton(metaclass=SingletonMetaClass):
    pass


if __name__ == '__main__':
    def test_singleton(cls):
        assert cls() is cls()
        assert type(cls())() is cls()
        cls().foo = 42
        assert cls().foo == 42


    test_singleton(SimpleSingleton)
    test_singleton(SaferSingleton)
    test_singleton(MetaSingleton)

    import inspect
    print(inspect.getclosurevars(MetaSingleton.__class__.__call__).nonlocals['instances'])
