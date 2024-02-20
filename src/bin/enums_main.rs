use crate::enums::{FigureFeature, PlaneFigure};

mod enums;

fn main() {
    println!("enums");

    let figures: [PlaneFigure; 3] = [
        PlaneFigure::Square(4.0),
        PlaneFigure::Rectangle(5.0, 2.0),
        PlaneFigure::Triangle(2.0, 3.0, 4.0)];

    for f in figures {
        let area = f.compute_area();
        let perimeter = f.compute_perimeter();

        println!("Details of {:?}:\tarea = {area}\tperimeter = {perimeter}", f);
    }
}