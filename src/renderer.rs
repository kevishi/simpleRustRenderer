pub struct Renderer;

impl Renderer {
    pub fn hello(&self) {
        println!("Hello")
    }

    #[allow(non_snake_case)]
    pub fn loadScene(&self,
        scene: &str, 
        numCircles: &mut i32,
        position: &mut Vec<f32>,
        velocity: &mut Vec<f32>,
        color: &mut Vec<f32>,
        radius: &mut Vec<f32>
    ) {
        let numCircles = match scene {
            "circle_rgb" => 3,
            _ => 0
        };
        
        *position = vec![0.0f32; 3 * numCircles as usize];
        *velocity = vec![0.0f32; 3 * numCircles as usize];
        *color = vec![0.0f32; 3 * numCircles as usize];
        *radius = vec![0.0f32; numCircles as usize];

        for circle in &mut *radius {
            *circle = 0.3f32;
        }

        position.push(0.4f32;
        position.push(0.5f32;
        position.push(0.75f32;
        color.push(1.0f32;
        color.push(0.0f32;
        color.push(0.0f32;

        position.push(0.5f32;
        position.push(0.5f32;
        position.push(0.5f32;
        color.push(0.0f32;
        color.push(1.0f32;
        color.push(0.0f32;

        position.push(0.6f32;
        position.push(0.5f32;
        position.push(0.25f32;
        color.push(0.0f32;
        color.push(0.0f32;
        color.push(1.0f32;
    }
}