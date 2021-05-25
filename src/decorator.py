"""
Decorator Pattern
=================

The decorator pattern (not to confused with Python's decorators) is
straight-forward in Python.
"""

from abc import ABC, abstractmethod
import math


class Predictor(ABC):
    @abstractmethod
    def fit(self, x, y): pass

    @abstractmethod
    def predict(self, x): pass


class PredictorDecorator(Predictor):
    def __init__(self, decorated_predictor: Predictor):
        self.decorated_predictor = decorated_predictor

    @abstractmethod
    def fit(self, x, y): pass

    @abstractmethod
    def predict(self, x): pass


class LinearPredictor(Predictor):
    def __init__(self):
        self.slope = 0
        self.offset = 0

    def fit(self, x, y):
        mean_x = sum(x) / len(x)
        mean_y = sum(y) / len(y)
        sos_xy = sum((xi - mean_x) * (yi - mean_y) for xi, yi in zip(x, y))
        sos_x = sum((xi - mean_x) ** 2 for xi in x)
        self.slope = sos_xy / sos_x
        self.offset = mean_y - self.slope * mean_x
        return self

    def predict(self, x):
        return [self.offset + self.slope * xi for xi in x]


class LogYDecorator(PredictorDecorator):
    def fit(self, x, y):
        logy = [math.log(yi) for yi in y]
        self.decorated_predictor.fit(x, logy)
        return self

    def predict(self, x):
        logy = self.decorated_predictor.predict(x)
        return [math.exp(yi) for yi in logy]


class LogXDecorator(PredictorDecorator):
    def fit(self, x, y):
        logx = [math.log(xi) for xi in x]
        self.decorated_predictor.fit(logx, y)
        return self

    def predict(self, x):
        logx = [math.log(xi) for xi in x]
        return self.decorated_predictor.predict(logx)


if __name__ == '__main__':
    x_train = [2, 4, 6, 8]
    y_train = [1, 2, 3, 4]

    linear_predictor = LinearPredictor().fit(x_train, y_train)
    print('linear predictor:', linear_predictor.predict(x_train))

    logy_predictor = LogYDecorator(LinearPredictor()).fit(x_train, y_train)
    print('log-y predictor:', logy_predictor.predict(x_train))

    logx_predictor = LogXDecorator(LinearPredictor()).fit(x_train, y_train)
    print('log-x predictor:', logx_predictor.predict(x_train))

    loglog_predictor = LogXDecorator(LogYDecorator(LinearPredictor())).fit(x_train, y_train)
    print('log-log predictor:', loglog_predictor.predict(x_train))
