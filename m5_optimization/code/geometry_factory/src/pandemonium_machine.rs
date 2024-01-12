use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha8Rng;
use ultraviolet::Vec3;

trait PandemoniumFunction {
    fn execute(&self, data: &mut Vec3);
}

struct SquareRoot {}
impl SquareRoot { pub fn new() -> Self {Self{}}}
impl PandemoniumFunction for SquareRoot {
    fn execute(&self, data: &mut Vec3) {
        data.x = data.x.sqrt();
        data.y = data.y.sqrt();
        data.z = data.z.sqrt();        
    }
}

struct Polynomial {}
impl Polynomial { pub fn new() -> Self {Self{}}}
impl PandemoniumFunction for Polynomial {
    fn execute(&self, data: &mut Vec3) {
        data.z = data.x * data.x * data.x - data.y + data.y * data.y;   
    }
}

struct Swap {}
impl Swap { pub fn new() -> Self {Self{}}}
impl PandemoniumFunction for Swap {
    fn execute(&self, data: &mut Vec3) {
        let swap: f32 = data.x;
        data.x = data.y;
        data.y = data.z;
        data.z = swap;
    }
}

struct Cos {}
impl Cos { pub fn new() -> Self {Self{}}}
impl PandemoniumFunction for Cos {
    fn execute(&self, data: &mut Vec3) {
        data.x = data.x.cos();
        data.y = data.y.cos();
        data.z = data.z.cos();
    }
}

struct Distance {}
impl Distance { pub fn new() -> Self {Self{}}}
impl PandemoniumFunction for Distance {
    fn execute(&self, data: &mut Vec3) {
        data.z = (data.x * data.x + data.y * data.y).sqrt();
    }
}

fn get_pandemonium_function(index: u32) -> Box<dyn PandemoniumFunction> {
    let index = index % 5;
    match index {
        0 => Box::new(SquareRoot::new()),
        1 => Box::new(Polynomial::new()),
        2 => Box::new(Swap::new()),
        3 => Box::new(Cos::new()),
        4 => Box::new(Distance::new()),
        _ => Box::new(SquareRoot::new()),
    }
}

fn generate_pandemonium(count: usize) -> Vec<Box<dyn PandemoniumFunction>> {
    let mut rng = ChaCha8Rng::seed_from_u64(1337);
    let mut pandemonium: Vec<Box<dyn PandemoniumFunction>> = Vec::<Box<dyn PandemoniumFunction>>::new();

    for _ in 0..count {
        pandemonium.push(get_pandemonium_function(rng.gen()));
    }

    pandemonium
}

// Professor Bærentzen doesn't care in which order the pandemonium
// functions are evaluated or on which points. He's wonky like that.
pub fn generate_pandemonium_particles(point_cloud: &Vec<Vec3>) -> Vec<Vec3> {
    let mut particles: Vec<Vec3> = Vec::<Vec3>::new();
    let pandemonium = generate_pandemonium(point_cloud.len());

    assert!(point_cloud.len() == pandemonium.len());

    for index in 0..point_cloud.len() {
        let mut particle: Vec3 = point_cloud[index].clone();
        pandemonium[index].execute(&mut particle);
        particles.push(particle);
    }

    particles
}