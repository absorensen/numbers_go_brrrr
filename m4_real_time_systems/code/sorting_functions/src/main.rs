use rand::{rngs::ThreadRng, Rng};
use ultraviolet::f32x8;
use std::{time::Instant, fmt};

#[derive(Clone, Copy, Debug)]
enum ElementFunction {
    SquareRoot,
    Polynomial,
    Swap,
    Cos,
    Distance,
}

impl ElementFunction {
    fn new(index: u32) -> ElementFunction {
        let index = index % 5;
        match index {
            0 => ElementFunction::SquareRoot,
            1 => ElementFunction::Polynomial,
            2 => ElementFunction::Swap,
            3 => ElementFunction::Cos,
            4 => ElementFunction::Distance,
            _ => ElementFunction::SquareRoot,
        }
    }
}

impl fmt::Display for ElementFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            ElementFunction::SquareRoot => "SquareRoot",
            ElementFunction::Polynomial => "Polynomial",
            ElementFunction::Swap => "Swap",
            ElementFunction::Cos => "Cos",
            ElementFunction::Distance => "Distance",
        };
        write!(f, "{}", out)
    }
}

struct Element {
    function: ElementFunction,
    x: f32,
    y: f32,
    z: f32,
}

impl Element {
    fn new(rng: &mut ThreadRng) -> Element {
        Element { function: ElementFunction::new(rng.gen()), x: rng.gen(), y: rng.gen(), z: rng.gen() }
    }

    #[inline] // This inline makes the runtime of sorted, sorted/split and sorted/split/specific the same. Otherwise each one has slightly better performance
    fn execute(&mut self) {
        match self.function {
            ElementFunction::SquareRoot => square_root(self),
            ElementFunction::Polynomial => polynomial(self),
            ElementFunction::Swap => swap(self),
            ElementFunction::Cos => cos(self),
            ElementFunction::Distance => distance(self),
        }
    }
}

fn square_root(element: &mut Element) {
    element.x = element.x.sqrt();
    element.y = element.y.sqrt();
    element.z = element.z.sqrt();
}

fn polynomial(element: &mut Element) {
    element.z = element.x * element.x * element.x - element.y + element.y * element.y; 
}

fn swap(element: &mut Element) {
    let swap: f32 = element.x;
    element.x = element.y;
    element.y = element.z;
    element.z = swap;
}

fn cos(element: &mut Element) {
    element.x = element.x.cos();
    element.y = element.y.cos();
    element.z = element.z.cos();
}

fn distance(element: &mut Element) {
    element.z = (element.x * element.x + element.y * element.y).sqrt();
}

fn naive(element_count: usize, test_count: usize, rng: &mut ThreadRng) {
    let mut input_data_naive: Vec<Element> = (0..element_count).into_iter().map(|_| Element::new(rng)).collect();

    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for element in &mut input_data_naive {
            element.execute();
        }    
    }
    println!("{} seconds elapsed for naive implementation", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}

fn sorted(element_count: usize, test_count: usize, rng: &mut ThreadRng) { 
    // One where one vector is merely sorted
    let mut input_data_sorted: Vec<Element> = (0..element_count).into_iter().map(|_| Element::new(rng)).collect();

    input_data_sorted.sort_by(|a, b| (a.function as i32).cmp(&(b.function as i32)));


    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for element in &mut input_data_sorted {
            element.execute();
        }    
    }
    println!("{} seconds elapsed for sorted implementation", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}

fn sorted_split(element_count: usize, test_count: usize, rng: &mut ThreadRng) { 
    // One where the initial vector is split into a vector for each category
    let mut input_data_sorted_split: Vec<Element> = (0..element_count).into_iter().map(|_| Element::new(rng)).collect();
    input_data_sorted_split.sort_by(|a, b| (a.function as i32).cmp(&(b.function as i32)));
    let mut input_data_sorted_split_buckets: Vec<Vec<Element>> = Vec::<Vec::<Element>>::new();
    input_data_sorted_split_buckets.push(Vec::<Element>::new());
    input_data_sorted_split_buckets.push(Vec::<Element>::new());
    input_data_sorted_split_buckets.push(Vec::<Element>::new());
    input_data_sorted_split_buckets.push(Vec::<Element>::new());
    input_data_sorted_split_buckets.push(Vec::<Element>::new());
    for _element_index in 0..element_count {
        let element = Element::new(rng);
        input_data_sorted_split_buckets[element.function as usize].push(element);
    }


    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for bucket in &mut input_data_sorted_split_buckets {
            for element in bucket {
                element.execute();
            }   
        }
    }
 
    println!("{} seconds elapsed for sorted and split implementation", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}

fn sorted_split_specific(element_count: usize, test_count: usize, rng: &mut ThreadRng) { 
    // One where the initial vector is split into a vector for each category and every vector has the specific function called on it
    let mut input_data_sorted_split: Vec<Element> = (0..element_count).into_iter().map(|_| Element::new(rng)).collect();
    input_data_sorted_split.sort_by(|a, b| (a.function as i32).cmp(&(b.function as i32)));
    let mut input_data_sorted_split_buckets: Vec<Vec<Element>> = Vec::<Vec::<Element>>::new();
    input_data_sorted_split_buckets.push(Vec::<Element>::new());
    input_data_sorted_split_buckets.push(Vec::<Element>::new());
    input_data_sorted_split_buckets.push(Vec::<Element>::new());
    input_data_sorted_split_buckets.push(Vec::<Element>::new());
    input_data_sorted_split_buckets.push(Vec::<Element>::new());
    for _element_index in 0..element_count {
        let element = Element::new(rng);
        input_data_sorted_split_buckets[element.function as usize].push(element);
    }


    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for element in &mut input_data_sorted_split_buckets[0] {
            square_root(element);
        }

        for element in &mut input_data_sorted_split_buckets[1] {
            polynomial(element);
        }

        for element in &mut input_data_sorted_split_buckets[2] {
            swap(element);
        }

        for element in &mut input_data_sorted_split_buckets[3] {
            cos(element);
        }

        for element in &mut input_data_sorted_split_buckets[4] {
            distance(element);
        }
    }

    println!("{} seconds elapsed for sorted and split with specific calls implementation", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}



struct ElementNoEnum {
    function: u32,
    x: f32,
    y: f32,
    z: f32,
}

impl fmt::Display for ElementNoEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self.function {
            0 => "SquareRoot",
            1 => "Polynomial",
            2 => "Swap",
            3 => "Cos",
            4 => "Distance",
            _ => "Invalid",
        };
        write!(f, "{}", out)
    }
}

impl ElementNoEnum {
    fn new(rng: &mut ThreadRng) -> ElementNoEnum {
        ElementNoEnum { function: rng.gen::<u32>() % 5, x: rng.gen(), y: rng.gen(), z: rng.gen() }
    }

    #[inline] // This inline makes the runtime of sorted, sorted/split and sorted/split/specific the same. Otherwise each one has slightly better performance
    fn execute(&mut self) {
        match self.function {
            0 => square_root_no_enum(self),
            1 => polynomial_no_enum(self),
            2 => swap_no_enum(self),
            3 => cos_no_enum(self),
            4 => distance_no_enum(self),
            _ => square_root_no_enum(self),
        }
    }
}

fn square_root_no_enum(element: &mut ElementNoEnum) {
    element.x = element.x.sqrt();
    element.y = element.y.sqrt();
    element.z = element.z.sqrt();
}

fn polynomial_no_enum(element: &mut ElementNoEnum) {
    element.z = element.x * element.x * element.x - element.y + element.y * element.y; 
}


fn swap_no_enum(element: &mut ElementNoEnum) {
    let swap: f32 = element.x;
    element.x = element.y;
    element.y = element.z;
    element.z = swap;
}


fn cos_no_enum(element: &mut ElementNoEnum) {
    element.x = element.x.cos();
    element.y = element.y.cos();
    element.z = element.z.cos();
}

fn distance_no_enum(element: &mut ElementNoEnum) {
    element.z = (element.x * element.x + element.y * element.y).sqrt();
}

fn naive_no_enum(element_count: usize, test_count: usize, rng: &mut ThreadRng) {
    let mut input_data_naive: Vec<ElementNoEnum> = (0..element_count).into_iter().map(|_| ElementNoEnum::new(rng)).collect();

    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for element in &mut input_data_naive {
            element.execute();
        }    
    }
    println!("{} seconds elapsed for naive implementation - no enum", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}

fn sorted_no_enum(element_count: usize, test_count: usize, rng: &mut ThreadRng) { 
    // One where one vector is merely sorted
    let mut input_data_sorted: Vec<ElementNoEnum> = (0..element_count).into_iter().map(|_| ElementNoEnum::new(rng)).collect();

    input_data_sorted.sort_by(|a, b| (a.function as i32).cmp(&(b.function as i32)));


    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for element in &mut input_data_sorted {
            element.execute();
        }    
    }
    println!("{} seconds elapsed for sorted implementation - no enum", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}

fn sorted_split_no_enum(element_count: usize, test_count: usize, rng: &mut ThreadRng) { 
    // One where the initial vector is split into a vector for each category
    let mut input_data_sorted_split: Vec<ElementNoEnum> = (0..element_count).into_iter().map(|_| ElementNoEnum::new(rng)).collect();
    input_data_sorted_split.sort_by(|a, b| (a.function as i32).cmp(&(b.function as i32)));
    let mut input_data_sorted_split_buckets: Vec<Vec<ElementNoEnum>> = Vec::<Vec::<ElementNoEnum>>::new();
    input_data_sorted_split_buckets.push(Vec::<ElementNoEnum>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementNoEnum>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementNoEnum>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementNoEnum>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementNoEnum>::new());
    for _element_index in 0..element_count {
        let element = ElementNoEnum::new(rng);
        input_data_sorted_split_buckets[element.function as usize].push(element);
    }


    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for bucket in &mut input_data_sorted_split_buckets {
            for element in bucket {
                element.execute();
            }   
        }
    }
    println!("{} seconds elapsed for sorted and split implementation - no enum", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}

fn sorted_split_specific_no_enum(element_count: usize, test_count: usize, rng: &mut ThreadRng) { 
    // One where the initial vector is split into a vector for each category and every vector has the specific function called on it
    let mut input_data_sorted_split: Vec<ElementNoEnum> = (0..element_count).into_iter().map(|_| ElementNoEnum::new(rng)).collect();
    input_data_sorted_split.sort_by(|a, b| (a.function as i32).cmp(&(b.function as i32)));
    let mut input_data_sorted_split_buckets: Vec<Vec<ElementNoEnum>> = Vec::<Vec::<ElementNoEnum>>::new();
    input_data_sorted_split_buckets.push(Vec::<ElementNoEnum>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementNoEnum>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementNoEnum>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementNoEnum>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementNoEnum>::new());
    for _element_index in 0..element_count {
        let element = ElementNoEnum::new(rng);
        input_data_sorted_split_buckets[element.function as usize].push(element);
    }


    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for element in &mut input_data_sorted_split_buckets[0] {
            square_root_no_enum(element);
        }

        for element in &mut input_data_sorted_split_buckets[1] {
            polynomial_no_enum(element);
        }

        for element in &mut input_data_sorted_split_buckets[2] {
            swap_no_enum(element);
        }

        for element in &mut input_data_sorted_split_buckets[3] {
            cos_no_enum(element);
        }

        for element in &mut input_data_sorted_split_buckets[4] {
            distance_no_enum(element);
        }
    }
 
    println!("{} seconds elapsed for sorted and split with specific calls implementation - no enum", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}


struct ElementSIMDx8 {
    function: ElementFunction,
    x: f32x8,
    y: f32x8,
    z: f32x8,
}

impl ElementSIMDx8 {
    fn new(rng: &mut ThreadRng) -> ElementSIMDx8 {
        let function = ElementFunction::new(rng.gen());
        let x = f32x8::splat(rng.gen::<f32>());
        let y = f32x8::splat(rng.gen::<f32>());
        let z = f32x8::splat(rng.gen::<f32>());
        ElementSIMDx8 { function: function, x: x, y: y, z: z }
    }

    #[inline] // This inline makes the runtime of sorted, sorted/split and sorted/split/specific the same. Otherwise each one has slightly better performance
    fn execute(&mut self) {
        match self.function {
            ElementFunction::SquareRoot => square_root_simd_8(self),
            ElementFunction::Polynomial => polynomial_simd_8(self),
            ElementFunction::Swap => swap_simd_8(self),
            ElementFunction::Cos => cos_simd_8(self),
            ElementFunction::Distance => distance_simd_8(self),
        }
    }
}

fn square_root_simd_8(element: &mut ElementSIMDx8) {
    element.x = element.x.sqrt();
    element.y = element.y.sqrt();
    element.z = element.z.sqrt();
}

fn polynomial_simd_8(element: &mut ElementSIMDx8) {
    element.z = element.x * element.x * element.x - element.y + element.y * element.y; 
}

fn swap_simd_8(element: &mut ElementSIMDx8) {
    let swap: f32x8 = element.x;
    element.x = element.y;
    element.y = element.z;
    element.z = swap;
}

fn cos_simd_8(element: &mut ElementSIMDx8) {
    element.x = element.x.cos();
    element.y = element.y.cos();
    element.z = element.z.cos();
}

fn distance_simd_8(element: &mut ElementSIMDx8) {
    element.z = (element.x * element.x + element.y * element.y).sqrt();
}

fn naive_simd_8(element_count: usize, test_count: usize, rng: &mut ThreadRng) {
    let mut input_data_naive: Vec<ElementSIMDx8> = (0..(element_count / 8)).into_iter().map(|_| ElementSIMDx8::new(rng)).collect();

    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for element in &mut input_data_naive {
            element.execute();
        }    
    }
    println!("{} seconds elapsed for naive implementation - simd x 8", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}


fn sorted_simd_8(element_count: usize, test_count: usize, rng: &mut ThreadRng) { 
    // One where one vector is merely sorted
    let mut input_data_sorted: Vec<ElementSIMDx8> = (0..(element_count / 8)).into_iter().map(|_| ElementSIMDx8::new(rng)).collect();

    input_data_sorted.sort_by(|a, b| (a.function as i32).cmp(&(b.function as i32)));


    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for element in &mut input_data_sorted {
            element.execute();
        }    
    }
    println!("{} seconds elapsed for sorted implementation - simd x 8", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}

fn sorted_split_simd_8(element_count: usize, test_count: usize, rng: &mut ThreadRng) { 
    // One where the initial vector is split into a vector for each category
    let mut input_data_sorted_split_buckets: Vec<Vec<ElementSIMDx8>> = Vec::<Vec::<ElementSIMDx8>>::new();
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8>::new());
    for _element_index in 0..(element_count / 8) {
        let element = ElementSIMDx8::new(rng);
        input_data_sorted_split_buckets[element.function as usize].push(element);
    }


    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for bucket in &mut input_data_sorted_split_buckets {
            for element in bucket {
                element.execute();
            }   
        }
    }
    println!("{} seconds elapsed for sorted and split implementation - simd x 8", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}

fn sorted_split_specific_simd_8(element_count: usize, test_count: usize, rng: &mut ThreadRng) { 
    // One where the initial vector is split into a vector for each category and every vector has the specific function called on it
    let mut input_data_sorted_split_buckets: Vec<Vec<ElementSIMDx8>> = Vec::<Vec::<ElementSIMDx8>>::new();
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8>::new());
    for _element_index in 0..(element_count/8) {
        let element = ElementSIMDx8::new(rng);
        input_data_sorted_split_buckets[element.function as usize].push(element);
    }


    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for element in &mut input_data_sorted_split_buckets[0] {
            square_root_simd_8(element);
        }

        for element in &mut input_data_sorted_split_buckets[1] {
            polynomial_simd_8(element);
        }

        for element in &mut input_data_sorted_split_buckets[2] {
            swap_simd_8(element);
        }

        for element in &mut input_data_sorted_split_buckets[3] {
            cos_simd_8(element);
        }

        for element in &mut input_data_sorted_split_buckets[4] {
            distance_simd_8(element);
        }
    }
    println!("{} seconds elapsed for sorted and split with specific calls implementation - simd x 8", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}



struct ElementSIMDx8Aligned {
    x: f32x8,
    y: f32x8,
    z: f32x8,
}

impl ElementSIMDx8Aligned {
    fn new(rng: &mut ThreadRng) -> ElementSIMDx8Aligned {
        let x = f32x8::splat(rng.gen::<f32>());
        let y = f32x8::splat(rng.gen::<f32>());
        let z = f32x8::splat(rng.gen::<f32>());
        ElementSIMDx8Aligned { x: x, y: y, z: z }
    }
}

fn square_root_simd_8_aligned(element: &mut ElementSIMDx8Aligned) {
    element.x = element.x.sqrt();
    element.y = element.y.sqrt();
    element.z = element.z.sqrt();
}

fn polynomial_simd_8_aligned(element: &mut ElementSIMDx8Aligned) {
    element.z = element.x * element.x * element.x - element.y + element.y * element.y; 
}

fn swap_simd_8_aligned(element: &mut ElementSIMDx8Aligned) {
    let swap: f32x8 = element.x;
    element.x = element.y;
    element.y = element.z;
    element.z = swap;
}

fn cos_simd_8_aligned(element: &mut ElementSIMDx8Aligned) {
    element.x = element.x.cos();
    element.y = element.y.cos();
    element.z = element.z.cos();
}

fn distance_simd_8_aligned(element: &mut ElementSIMDx8Aligned) {
    element.z = (element.x * element.x + element.y * element.y).sqrt();
}


fn sorted_split_specific_simd_8_aligned(element_count: usize, test_count: usize, rng: &mut ThreadRng) { 
    // One where the initial vector is split into a vector for each category and every vector has the specific function called on it
    let mut input_data_sorted_split_buckets: Vec<Vec<ElementSIMDx8Aligned>> = Vec::<Vec::<ElementSIMDx8Aligned>>::new();
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8Aligned>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8Aligned>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8Aligned>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8Aligned>::new());
    input_data_sorted_split_buckets.push(Vec::<ElementSIMDx8Aligned>::new());
    for _element_index in 0..(element_count / 8) {
        let element = ElementSIMDx8Aligned::new(rng);
        input_data_sorted_split_buckets[rng.gen::<usize>() % 5].push(element);
    }


    let now: Instant = Instant::now();
    for _test_index in 0..test_count {
        for element in &mut input_data_sorted_split_buckets[0] {
            square_root_simd_8_aligned(element);
        }

        for element in &mut input_data_sorted_split_buckets[1] {
            polynomial_simd_8_aligned(element);
        }

        for element in &mut input_data_sorted_split_buckets[2] {
            swap_simd_8_aligned(element);
        }

        for element in &mut input_data_sorted_split_buckets[3] {
            cos_simd_8_aligned(element);
        }

        for element in &mut input_data_sorted_split_buckets[4] {
            distance_simd_8_aligned(element);
        }
    }
    println!("{} seconds elapsed for sorted and split with specific calls implementation - simd x 8 aligned", now.elapsed().as_millis() as f64 / test_count as f64 * 0.001);
}



fn main() {
    let element_count = 300000 * 8;
    let test_count = 100;
    let mut rng: ThreadRng = rand::thread_rng();

    println!("=== PERFORMANCE TEST ===");
    println!("ELEMENT COUNT: {}", element_count);
    println!("TEST COUNT: {}", test_count);
    println!(" ");
    println!("== ENUM ==");
    naive(element_count, test_count, &mut rng);
    sorted(element_count, test_count, &mut rng); // These 3 have almost the exact same runtime, try removing the inline in the execute function
    sorted_split(element_count, test_count, &mut rng); // These 3 have almost the exact same runtime, try removing the inline in the execute function
    sorted_split_specific(element_count, test_count, &mut rng); // These 3 have almost the exact same runtime, try removing the inline in the execute function
    println!(" ");

    // These have the exact same runspeed as the ones with enums so I will be assuming that enums are optimal enough
    println!("== NO ENUM ==");
    naive_no_enum(element_count, test_count, &mut rng);
    sorted_no_enum(element_count, test_count, &mut rng);
    sorted_split_no_enum(element_count, test_count, &mut rng);
    sorted_split_specific_no_enum(element_count, test_count, &mut rng);
    println!(" ");

    // These would probably pull ahead even more if the functions were more compute bound
    // Aligning the struct didn't make much of a difference. Potentially it makes it slower.
    println!("== SIMD x 8 ==");
    naive_simd_8(element_count, test_count, &mut rng);
    sorted_simd_8(element_count, test_count, &mut rng);
    sorted_split_simd_8(element_count, test_count, &mut rng);
    sorted_split_specific_simd_8(element_count, test_count, &mut rng);
    println!(" ");
    
    // Is about 8% faster than sorted_split_specific_simd_8
    println!("== SIMD x 8 Aligned ==");
    sorted_split_specific_simd_8_aligned(element_count, test_count, &mut rng);
    println!(" ");

}
