use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::*;

pub fn sierpinski_carpet(
    depth: u32,
    drawing_area: &DrawingArea<BitMapBackend, Shift>,
) -> Result<(), Box<dyn std::error::Error>> {
    if depth == 0 {
        return Ok(());
    }
    let (width, height) = drawing_area.dim_in_pixel();
    let mut sub_areas = Vec::new();
    let a = drawing_area.clone();
    let a1 = drawing_area.clone();
    let a2 = drawing_area.clone();

    // blueprint
    // top left
    let area = a.shrink((0, 0), (width / 2, height / 2));
    // bottom left
    let area1 = a1.shrink((0, height / 2), (width / 2, height / 2));
    // bottom right
    let area2 = a2.shrink((width / 2, height / 2), (width / 2, height / 2));
    sub_areas.push(area);
    sub_areas.push(area1);
    sub_areas.push(area2);

    // blueprint
    // let sub_areas = drawing_area.split_evenly((3, 3));
    for (idx, sub_area) in (0..).zip(sub_areas.iter()) {
        if depth == 1 {
            sub_area.fill(&WHITE)?;
        } else {
            sub_area.fill(&BLUE)?;
            sierpinski_carpet(depth - 1, sub_area)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 1024;
    let height = 1024;
    let root = BitMapBackend::new("sierpinski.png", (width, height)).into_drawing_area();

    root.fill(&BLACK)?;

    sierpinski_carpet(9, &root)
}
#[test]
fn entry_point() {
    main().unwrap()
}
