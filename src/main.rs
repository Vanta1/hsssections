// Group B16 Mechanics Truss Design Project
// The purpose of this program is to calculate the tensile and compressive resistances for various HSS steel sections. 
// The lengths used correspond to the lengths in the design, 3m, 4.67m and 5.55m.
// Code by Cooper Sandys, HSSShapes.txt data taken from the Preliminary Design Report Info.

use std::fs;

const PI: f32 = 3.14159265358979;

fn calculate_tensile_strength(area: f32) -> f32 {
    (0.9 * 370.0 * area) / 1000.0
}

fn calculate_compressive_strength(area: f32, radius: f32, length: f32) -> f32 {
    let sigma_e: f32 = (PI.powi(2) * 200_000.0) / ((length * 1000.0) / radius).powi(2);
    let lambda: f32 = (370.0 / sigma_e).sqrt();
    0.9 * 370.0 * area * (1.0 / (1.0 + lambda.powf(2.0 * 1.34)).powf(1.0 / 1.34)) / 1000.0
}

fn main() {
    println!("DESIGNATION - AREA - RADIUS - TENSILE - COMPRESSIVE @ 3m - COMPRESSIVE @ 4.67m - COMPRESSIVE @ 5.55m\n");
    let file = fs::read_to_string("HSSShapes.txt").unwrap();
    let lines_raw: Vec<&str> = file.lines().collect();
    let mut lines_parsed: Vec<Vec<&str>> = Vec::new();
    for &line in lines_raw.iter() {
        let to_append: Vec<&str> = line.split(",").collect();
        lines_parsed.push(to_append);
    }
    for line in lines_parsed {
        let tensile = calculate_tensile_strength(line[1].parse::<f32>().unwrap());
        let compressive_3m = calculate_compressive_strength(
            line[1].parse::<f32>().unwrap(),
            line[2].parse::<f32>().unwrap(),
            3.0,
        );
        let compressive_4m = calculate_compressive_strength(
            line[1].parse::<f32>().unwrap(),
            line[2].parse::<f32>().unwrap(),
            4.67,
        );
        let compressive_5m = calculate_compressive_strength(
            line[1].parse::<f32>().unwrap(),
            line[2].parse::<f32>().unwrap(),
            5.55,
        );
        println!(
            "{} - {} - {} - {} - {} - {} - {}",
            line[0], line[1], line[2], tensile, compressive_3m, compressive_4m, compressive_5m
        );
    }
}
