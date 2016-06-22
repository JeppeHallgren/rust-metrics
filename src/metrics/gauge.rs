use metrics::metric::{Metric, MetricValue};
use time::get_time;

#[derive(Copy, Clone, Debug, Default)]
pub struct StdGauge {
    pub value: f64,
}

// Gauge is a Metric that represents a single numerical value that can
// arbitrarily go up and down.
//
// A Gauge is typically used for measured values like temperatures or current
// memory usage, but also "counts" that can go up and down.
//
pub trait Gauge {
    fn inc(&mut self);
    fn dec(&mut self);
    // How much we raise the gauge
    fn add(&mut self, value: f64);
    // How much we lower the gauge
    fn sub(&mut self, value: f64);
    fn set(&mut self, value: f64);
    fn set_to_current_time(&mut self);

    fn snapshot(self) -> Self;
}

// Naive implementation of a gauge, it might be nice to make one build on atomics
impl Gauge for StdGauge {
    // dec(double v): Decrement the gauge by the given amount
    // set(double v): Set the gauge to the given value

    // inc(): Increment the gauge by 1
    fn inc(&mut self) {
        self.value += 1.0;
    }

    // dec(): Decrement the gauge by 1
    fn dec(&mut self) {
        self.value -= 1.0;
    }

    // Implementing Prometheus inc(double v): Increment the gauge by the given amount
    fn add(&mut self, value: f64) {
        self.value += value;
        // TODO check for negative
    }

    // Implementing Prometheus dec(double v): Decrement the gauge by the given amount
    fn sub(&mut self, value: f64) {
        self.value -= value;
    }

    fn set(&mut self, value: f64) {
        self.value = value
    }

    fn set_to_current_time(&mut self) {
        self.value = timestamp();
    }

    fn snapshot(self) -> StdGauge {
        StdGauge { value: self.value }
    }
}

impl Metric for StdGauge {
    fn export_metric(&self) -> MetricValue {
        MetricValue::Gauge(self.snapshot())
    }
}

fn timestamp() -> f64 {
    let timespec = get_time();
    // 1459440009.113178
    timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_and_snapshot() {
        let g = StdGauge::default();
        let mut g_snapshot = g.snapshot();

        g_snapshot.set(10.0);

        assert_eq!(g.value, 0.0);
        assert_eq!(g_snapshot.value, 10.0);
    }
}
