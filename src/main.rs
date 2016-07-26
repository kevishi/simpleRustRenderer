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

    

    let mut numCircles = 0;
    let mut position: Vec<f32> = Vec::new();
    let mut velocity: Vec<f32> = Vec::new();
    let mut color: Vec<f32> = Vec::new();
    let mut radius: Vec<f32> = Vec::new();
    let mut ren = renderer::Renderer {
        numCircles: numCircles,
        position: position,
        velocity: velocity,
        color: color,
        radius: radius
    };
    

    ren.loadScene(&scene);
}