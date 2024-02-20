pub trait FigureFeature {
    fn compute_area(&self) -> f64;
    fn compute_perimeter(&self) -> f64;
}

#[derive(Debug)]
pub enum PlaneFigure {
    Square(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}

impl FigureFeature for PlaneFigure {
    fn compute_area(&self) -> f64 {
        match self {
            PlaneFigure::Square(l) => l * l,
            PlaneFigure::Rectangle(a, b) => a * b,
            PlaneFigure::Triangle(a, b, c) => {
                let semi_p = (a + b + c) / 2.0;
                let squared_a = semi_p * (semi_p - a) * (semi_p - b) * (semi_p - c);
                squared_a.sqrt()
            }
        }
    }

    fn compute_perimeter(&self) -> f64 {
        match self {
            PlaneFigure::Square(l) => 4.0 * l,
            PlaneFigure::Rectangle(a, b) => 2.0 * (a + b),
            PlaneFigure::Triangle(a, b, c) => a + b + c,
        }
    }
}

