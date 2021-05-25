//! Decorator Pattern
//! =================
//!
//! Traits only define behavior and no state, unlike classes in other
//! languages. Thus, it does not make sense to define a separate
//! Decorator interface, that only differs from the Component
//! interface by containing state in the form of the wrapped object.
//! Concrete decorators each need to implement their own state and
//! they can directly use the Component trait.
//!

trait Predictor {
    fn fit(self, x: impl Iterator<Item = f64>, y: impl Iterator<Item = f64>) -> Self;
    fn predict<'a>(&self, x: impl Iterator<Item = f64> + 'a) -> Box<dyn Iterator<Item = f64> + 'a>;
}

#[derive(Default)]
struct LinearPredictor {
    offset: f64,
    slope: f64,
}

impl Predictor for LinearPredictor {
    fn fit(mut self, x: impl Iterator<Item = f64>, y: impl Iterator<Item = f64>) -> Self {
        let mut n = 0.0;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xx = 0.0;
        let mut sum_xy = 0.0;
        for (xi, yi) in x.zip(y) {
            n += 1.0;
            sum_x += xi;
            sum_y += yi;
            sum_xx += xi * xi;
            sum_xy += xi * yi;
        }
        self.slope = (sum_xy * n - sum_x * sum_y) / (sum_xx * n - sum_x * sum_x);
        self.offset = (sum_y - self.slope * sum_x) / n;
        self
    }

    fn predict<'a>(&self, x: impl Iterator<Item = f64> + 'a) -> Box<dyn Iterator<Item = f64> + 'a> {
        let slope = self.slope;
        let offset = self.offset;
        Box::new(x.map(move |xi| offset + slope * xi))
    }
}

struct LogYDecorator<P: Predictor> {
    decorated_predictor: P,
}

impl<P: Predictor> LogYDecorator<P> {
    pub fn new(decorated_predictor: P) -> Self {
        LogYDecorator {
            decorated_predictor,
        }
    }
}

impl<P: Predictor> Predictor for LogYDecorator<P> {
    fn fit(mut self, x: impl Iterator<Item = f64>, y: impl Iterator<Item = f64>) -> Self {
        let logy = y.map(f64::ln);
        self.decorated_predictor = self.decorated_predictor.fit(x, logy);
        self
    }

    fn predict<'a>(&self, x: impl Iterator<Item = f64> + 'a) -> Box<dyn Iterator<Item = f64> + 'a> {
        Box::new(self.decorated_predictor.predict(x).map(f64::exp))
    }
}

struct LogXDecorator<P: Predictor> {
    decorated_predictor: P,
}

impl<P: Predictor> LogXDecorator<P> {
    pub fn new(decorated_predictor: P) -> Self {
        LogXDecorator {
            decorated_predictor,
        }
    }
}

impl<P: Predictor> Predictor for LogXDecorator<P> {
    fn fit(mut self, x: impl Iterator<Item = f64>, y: impl Iterator<Item = f64>) -> Self {
        let logx = x.map(f64::ln);
        self.decorated_predictor = self.decorated_predictor.fit(logx, y);
        self
    }

    fn predict<'a>(&self, x: impl Iterator<Item = f64> + 'a) -> Box<dyn Iterator<Item = f64> + 'a> {
        let logx = x.map(f64::ln);
        Box::new(self.decorated_predictor.predict(logx))
    }
}

#[test]
fn predict() {
    let x_train = [2.0, 4.0, 6.0, 8.0];
    let y_train = [1.0, 2.0, 3.0, 4.0];

    let linear_predictor =
        LinearPredictor::default().fit(x_train.iter().copied(), y_train.iter().copied());
    println!(
        "linear predictor: {:?}",
        linear_predictor
            .predict(x_train.iter().copied())
            .collect::<Vec<_>>()
    );

    let logy_predictor = LogYDecorator::new(LinearPredictor::default())
        .fit(x_train.iter().copied(), y_train.iter().copied());
    println!(
        "log-y predictor: {:?}",
        logy_predictor
            .predict(x_train.iter().copied())
            .collect::<Vec<_>>()
    );

    let logx_predictor = LogXDecorator::new(LinearPredictor::default())
        .fit(x_train.iter().copied(), y_train.iter().copied());
    println!(
        "log-x predictor: {:?}",
        logx_predictor
            .predict(x_train.iter().copied())
            .collect::<Vec<_>>()
    );

    let loglog_predictor = LogXDecorator::new(LogYDecorator::new(LinearPredictor::default()))
        .fit(x_train.iter().copied(), y_train.iter().copied());
    println!(
        "log-log predictor: {:?}",
        loglog_predictor
            .predict(x_train.iter().copied())
            .collect::<Vec<_>>()
    );
}
