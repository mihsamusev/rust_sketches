use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Var {
    x: f32,
    dx: f32,
}

impl ops::Add for Var {
    type Output = Var;
    fn add(self, rhs: Self) -> Self::Output {
        Var {
            x: self.x + rhs.x,
            dx: self.dx + rhs.dx,
        }
    }
}

impl ops::Sub for Var {
    type Output = Var;
    fn sub(self, rhs: Self) -> Self::Output {
        Var {
            x: self.x - rhs.x,
            dx: self.dx - rhs.dx,
        }
    }
}

impl ops::Mul for Var {
    type Output = Var;
    fn mul(self, rhs: Self) -> Self::Output {
        Var {
            x: self.x * rhs.x,
            dx: self.x * rhs.dx + self.dx * rhs.x,
        }
    }
}

impl ops::Mul<f32> for Var {
    type Output = Var;
    fn mul(self, rhs: f32) -> Self::Output {
        Var {x: self.x * rhs, dx: self.dx * rhs}
    }
}

impl ops::Mul<Var> for f32 {
    type Output = Var;
    fn mul(self, rhs: Var) -> Self::Output {
        Var {x: self* rhs.x, dx: self * rhs.dx}
    }
}

impl ops::Div for Var {
    type Output = Var;
    fn div(self, rhs: Self) -> Self::Output {
        Var {
            x: self.x / rhs.x,
            dx: (self.dx * rhs.x - self.x * rhs.dx) / (rhs.x * rhs.x),
        }
    }
}

impl Var {
    pub fn new(x: f32, dx: f32) -> Self {
        Self { x, dx }
    }

    pub fn sin(self) -> Self {
        Var {
            x: self.x.sin(),
            dx: self.x.cos() * self.dx,
        }
    }
    pub fn cos(self) -> Self {
        Var {
            x: self.x.cos(),
            dx: -self.x.sin() * self.dx,
        }
    }
    pub fn tan(self) -> Self {
        self.sin() / self.cos()
    }
    pub fn ln(self) -> Self {
        Var {
            x: self.x.ln(),
            dx: self.dx / self.x,
        }
    }
    pub fn exp(self) -> Self {
        Var {
            x: self.x.exp(),
            dx: self.x.exp() * self.dx,
        }
    }
    pub fn powi(self, n: i32) -> Self {
        Var {
            x: self.x.powi(n),
            dx: (n as f32) * self.x.powi(n - 1) * self.dx,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn it_works() {
        let var = Var::new(1.0, 0.0) + Var::new(2.0, 0.0);
        assert_eq!(var, Var::new(3.0, 0.0));
    }

    #[test]
    fn algebraic_expression_derivative_ok() {
        let x = Var::new(24.0, 1.0);
        let a = Var::new(22.0, 0.0);
        let b = Var::new(2.0, 0.0);
        let result = x * a / (x * x + b);
        assert_approx_eq!(result.x, 0.9134948096885813);
        assert_approx_eq!(result.dx, -0.03779887692915554);
    }

    #[test]
    fn mixed_expression_derivative_ok() {
        let x = Var::new(10.0, 1.0);
        let a = Var::new(2.0, 0.0);
        let c = Var::new(1.5, 0.0);
        let expr = |x: Var, a: Var, c: Var| {
            ((a * x.powi(3) + a * c * x).powi(4) * (a * x).sin()) / (256.0 * a.powi(5) * x.powi(9))
                - (c * x).cos()
        };

        assert_approx_eq!(expr(x, a, c).x, 2.652201219184028);
        assert_approx_eq!(expr(x, a, c).dx, 3.2126995936768443);
        //for df/da 7.513185262318497
        //for df/dc 6.577460206746546
    }
} 


