extern crate simpleRustRenderer;

use std::env;

use simpleRustRenderer::renderer;

fn usage(progname: &str) {
    println!("Usage: {} [options] scenename", progname);
    println!("Valid scenenames are:");
    println!("Program Options:");
    println!("  Not yet implemented");
}

fn main() {
    let mut scene = String::new();
    if let Some(x) = env::args().nth(1) {
        println!("Running renderer on scene:[{}]", x);
        scene = x;
    } else {
        usage("simpleCudaRenderer");
    }

    let ren = renderer::Renderer {};

    let numCircles: &mut i32;
    let position: &mut Vec<f32>;
    let velocity: &mut Vec<f32>;
    let color: &mut Vec<f32>;
    let radius: &mut Vec<f32>;
    ren.loadScene(&scene, numCircles, position, velocity, color, radius);
}