#[allow(non_snake_case)]
pub struct Renderer {
    pub numCircles: i32,
    pub position: Vec<f32>,
    pub velocity: Vec<f32>,
    pub color: Vec<f32>,
    pub radius: Vec<f32>
}

impl Renderer {
    pub fn hello(&self) {
        println!("Hello")
    }

    #[allow(non_snake_case)]
    pub fn loadScene(&mut self, scene: &str) {
        self.numCircles = match scene {
            "circle_rgb" => 3,
            _ => 0
        };
        
        if scene == "circle_rgb"
        {
            &self.radius.push(0.3f32);
            &self.radius.push(0.3f32);
            &self.radius.push(0.3f32);
            &self.position.push(0.4f32);
            &self.position.push(0.5f32);
            &self.position.push(0.75f32);
            &self.color.push(1.0f32);
            &self.color.push(0.0f32);
            &self.color.push(0.0f32);
            &self.position.push(0.5f32);
            &self.position.push(0.5f32);
            &self.position.push(0.5f32);
            &self.color.push(0.0f32);
            &self.color.push(1.0f32);
            &self.color.push(0.0f32);
            &self.position.push(0.6f32);
            &self.position.push(0.5f32);
            &self.position.push(0.25f32);
            &self.color.push(0.0f32);
            &self.color.push(0.0f32);
            &self.color.push(1.0f32);
        }
    }
}