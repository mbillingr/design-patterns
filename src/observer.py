"""
Observer Pattern
================

Python does not have many of the restrictions in old C++ that led to the
original design patterns. We can easily define the Observer interface to be any
callable (with matching signature). This allows simple functions to serve as
observers as well as traditional objects. It requires slightly more work when
registering an observer because one has to choose which method should receive
the notifications.
"""


class Subject:
    def __init__(self):
        self.observers = []

    def register_observer(self, observer):
        self.observers.append(observer)

    def unregister_observer(self, observer):
        self.observers.remove(observer)

    def notify_observers(self):
        for observer in self.observers:
            observer(self)


if __name__ == '__main__':
    class Counter(Subject):
        def __init__(self):
            super().__init__()
            self.count = 0

        def inc(self):
            self.count += 1
            self.notify_observers()


    def count_observer(subject):
        # We rely on duck typing; this observer should only be registered
        # with subjects that provide a `.count` attribute.
        print(f"Received {subject.count} from {subject}")


    counter_a = Counter()
    counter_b = Counter()

    counter_a.inc()
    counter_b.inc()

    counter_a.register_observer(count_observer)

    counter_a.inc()
    counter_b.inc()

    counter_b.register_observer(count_observer)

    counter_a.inc()
    counter_b.inc()

    counter_a.unregister_observer(count_observer)

    counter_a.inc()
    counter_b.inc()
