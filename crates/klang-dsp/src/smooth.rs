/// need ts to avoid sudden pop noise

#[derive(Clone, Copy, Debug)]
pub struct Smoother {
    current: f32,
    target: f32,
    coef: f32,
}

impl Smoother {
    pub fn new(value: f32) -> Self {
        Self { current: value, target: value, coef: 0.0 }
    }

    pub fn set_time(&mut self, time_s: f32, rate: f32) {
        self.coef = if time_s <= 0.0 || rate <= 0.0 { 0.0 } else { (-1.0 / (time_s * rate)).exp() };
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn reset(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }

    #[inline(always)]
    pub fn next(&mut self) -> f32 {
        if self.current != self.target {
            self.current = self.target + (self.current - self.target) * self.coef;
            if (self.current - self.target).abs() < 1e-5 { ///to avoid forever calculation loop
                self.current = self.target;
            }
        }
        self.current
    }

    pub fn value(&self) -> f32 {
        self.current
    }

    pub fn is_settled(&self) -> bool {
        self.current == self.target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glides_then_snaps() {
        let mut s = Smoother::new(0.0);
        s.set_time(0.01, 1000.0);
        s.set_target(1.0);
        let first = s.next();
        assert!(first > 0.0 && first < 1.0);
        for _ in 0..1000 {
            s.next();
        }
        assert!(s.is_settled());
        assert_eq!(s.value(), 1.0);
    }
}