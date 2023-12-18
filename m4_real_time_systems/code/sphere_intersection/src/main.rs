// http://www.cs.uu.nl/docs/vakken/magr/2017-2018/files/SIMD%20Tutorial.pdf
// https://jacco.ompf2.com/2020/05/12/opt3simd-part-1-of-2/
// https://www.rustsim.org/blog/2020/03/23/simd-aosoa-in-nalgebra/
// https://github.com/bitshifter/mathbench-rs/blob/master/benches/ray_sphere_intersect.rs

use std::time::Instant;

use ultraviolet as uv;
use ultraviolet::{f32x4, f32x8};
use uv::{Vec3x8, Vec3x4, Vec3};
use wide::CmpGt;

const ELEMENT_COUNT : usize = 1024;

fn ray_sphere_intersect(
    ray_o: &uv::Vec3,
    ray_d: &uv::Vec3,
    sphere_o: &uv::Vec3,
    sphere_r_sq: f32,
) -> f32 {
    let oc = *ray_o - *sphere_o;
    let b = oc.dot(*ray_d);
    let c = oc.mag_sq() - sphere_r_sq;
    let descrim = b * b - c;

    if descrim > 0.0 {
        let desc_sqrt = descrim.sqrt();

        let t1 = -b - desc_sqrt;
        if t1 > 0.0 {
            t1
        } else {
            let t2 = -b + desc_sqrt;
            if t2 > 0.0 {
                t2
            } else {
                f32::MAX
            }
        }
    } else {
        f32::MAX
    }
}

fn ray_sphere_intersect_test(test_count: i32) -> f64 {
    let mut sphere_o_vec: Vec<Vec3> = Vec::<Vec3>::new();
    sphere_o_vec.resize(ELEMENT_COUNT, Vec3::new(1.0, 2.0, 3.0));

    let mut sphere_r_sq_vec: Vec<Vec3> = Vec::<Vec3>::new();
    sphere_r_sq_vec.resize(ELEMENT_COUNT, Vec3::new(1.0, 2.0, 3.0));

    let mut ray_o: Vec<Vec3> = Vec::<Vec3>::new();
    ray_o.resize(ELEMENT_COUNT, Vec3::new(1.0, 2.0, 3.0));

    let mut ray_d: Vec<f32> = Vec::<f32>::new();
    ray_d.resize(ELEMENT_COUNT, 1.0);

    let mut results: Vec<f32> = Vec::<f32>::new();
    results.resize(ELEMENT_COUNT, 1.0);

    let now = Instant::now();
    for _test_index in 0..test_count {
        for element_index in 0..ELEMENT_COUNT {
            results[element_index] = ray_sphere_intersect(&sphere_o_vec[element_index], &sphere_r_sq_vec[element_index], &ray_o[element_index], ray_d[element_index]);
        }
    }
    let elapsed_time = now.elapsed();

    (elapsed_time.as_nanos() as f64 / test_count as f64) as f64
}

struct SetOfVector3 {
    x: [f32; ELEMENT_COUNT],
    y: [f32; ELEMENT_COUNT],
    z: [f32; ELEMENT_COUNT],
}

impl SetOfVector3 {
    fn new(x: f32, y: f32, z: f32) -> SetOfVector3{
        SetOfVector3 { x: [x; ELEMENT_COUNT], y: [y; ELEMENT_COUNT], z: [z; ELEMENT_COUNT] }
    }
}

fn ray_sphere_intersect_soa(
    ray_o: &SetOfVector3,
    ray_d: &SetOfVector3,
    sphere_o: &SetOfVector3,
    sphere_r_sq: &Vec<f32>,
    out: &mut Vec<f32>
) -> () {
    for element_index in 0..ELEMENT_COUNT {
        let oc = Vec3::new(ray_o.x[element_index] - sphere_o.x[element_index], ray_o.y[element_index] - sphere_o.y[element_index], ray_o.z[element_index] - sphere_o.z[element_index]) ;
        let b = oc.dot(Vec3::new(ray_d.x[element_index], ray_d.y[element_index], ray_d.z[element_index]));
        let c = oc.mag_sq() - sphere_r_sq[element_index];
        let descrim = b * b - c;

        if descrim > 0.0 {
            let desc_sqrt = descrim.sqrt();

            let t1 = -b - desc_sqrt;
            if t1 > 0.0 {
                out[element_index] = t1;
            } else {
                let t2 = -b + desc_sqrt;
                if t2 > 0.0 {
                    out[element_index] = t2;
                } else {
                    out[element_index] = f32::MAX;
                }
            }
        } else {
            out[element_index] = f32::MAX;
        }
    }
}

fn ray_sphere_intersect_soa_test(test_count: i32) -> f64 {
    let sphere_o_vec: SetOfVector3 = SetOfVector3::new(1.0, 2.0, 3.0);
    let mut sphere_r_sq_vec: Vec<f32> = Vec::<f32>::new();
    sphere_r_sq_vec.resize(ELEMENT_COUNT, 1.0);

    let ray_o: SetOfVector3 = SetOfVector3::new(1.0, 2.0, 3.0);
    let ray_d: SetOfVector3 = SetOfVector3::new(1.0, 2.0, 3.0);
    let mut results: Vec<f32> = Vec::<f32>::new();
    results.resize(ELEMENT_COUNT, 1.0);

    let now = Instant::now();
    for _test_index in 0..test_count {
        ray_sphere_intersect_soa( &ray_o, &ray_d, &sphere_o_vec, &sphere_r_sq_vec,&mut results);
    }
    let elapsed_time = now.elapsed();

    (elapsed_time.as_nanos() as f64 / test_count as f64) as f64
}

fn ray_sphere_intersect_x4(
    sphere_o: &uv::Vec3x4,
    sphere_r_sq: &uv::f32x4,
    ray_o: &uv::Vec3x4,
    ray_d: &uv::Vec3x4,
) -> uv::f32x4 {
    let oc = *ray_o - *sphere_o;
    let b = oc.dot(*ray_d);
    let c = oc.mag_sq() - sphere_r_sq;
    let descrim = b * b - c;

    let desc_pos = descrim.cmp_gt(0.0);

    let desc_sqrt = descrim.sqrt();

    let t1 = -b - desc_sqrt;
    let t1_valid = t1.cmp_gt(0.0) & desc_pos;

    let t2 = -b + desc_sqrt;
    let t2_valid = t2.cmp_gt(0.0) & desc_pos;

    let t = t2_valid.blend(t2, uv::f32x4::splat(std::f32::MAX));
    let t = t1_valid.blend(t1, t);

    t
}

fn ray_sphere_intersect_x4_test(test_count: i32) -> f64 {
    let mut sphere_o_vec: Vec<Vec3x4> = Vec::<Vec3x4>::new();
    sphere_o_vec.resize(ELEMENT_COUNT / 4, Vec3x4::new_splat(1.0, 2.0, 3.0));

    let mut sphere_r_sq_vec: Vec<f32x4> = Vec::<f32x4>::new();
    sphere_r_sq_vec.resize(ELEMENT_COUNT / 4, f32x4::splat(1.0));

    let mut ray_o: Vec<Vec3x4> = Vec::<Vec3x4>::new();
    ray_o.resize(ELEMENT_COUNT / 4, Vec3x4::new_splat(1.0, 2.0, 3.0));

    let mut ray_d: Vec<Vec3x4> = Vec::<Vec3x4>::new();
    ray_d.resize(ELEMENT_COUNT / 4, Vec3x4::new_splat(1.0, 2.0, 3.0));

    let mut results: Vec<f32x4> = Vec::<f32x4>::new();
    results.resize(ELEMENT_COUNT / 4, f32x4::splat(1.0));

    let now = Instant::now();
    for _test_index in 0..test_count {
        for element_index in 0..ELEMENT_COUNT / 4 {
            results[element_index] = ray_sphere_intersect_x4(&sphere_o_vec[element_index], &sphere_r_sq_vec[element_index], &ray_o[element_index], &ray_d[element_index]);
        }
    }
    let elapsed_time = now.elapsed();

    (elapsed_time.as_nanos() as f64 / test_count as f64) as f64
}

fn ray_sphere_intersect_x8(
    sphere_o: &uv::Vec3x8,
    sphere_r_sq: &uv::f32x8,
    ray_o: &uv::Vec3x8,
    ray_d: &uv::Vec3x8,
) -> uv::f32x8 {
    let oc = *ray_o - *sphere_o;
    let b = oc.dot(*ray_d);
    let c = oc.mag_sq() - sphere_r_sq;
    let descrim = b * b - c;

    let desc_pos = descrim.cmp_gt(0.0);

    let desc_sqrt = descrim.sqrt();

    let t1 = -b - desc_sqrt;
    let t1_valid = t1.cmp_gt(0.0) & desc_pos;

    let t2 = -b + desc_sqrt;
    let t2_valid = t2.cmp_gt(0.0) & desc_pos;

    let t = t2_valid.blend(t2, uv::f32x8::splat(std::f32::MAX));
    let t = t1_valid.blend(t1, t);

    t
}

fn ray_sphere_intersect_x8_test(test_count: i32) -> f64 {
    let mut sphere_o_vec: Vec<Vec3x8> = Vec::<Vec3x8>::new();
    sphere_o_vec.resize(ELEMENT_COUNT / 8, Vec3x8::new_splat(1.0, 2.0, 3.0));

    let mut sphere_r_sq_vec: Vec<f32x8> = Vec::<f32x8>::new();
    sphere_r_sq_vec.resize(ELEMENT_COUNT / 8, f32x8::splat(1.0));

    let mut ray_o: Vec<Vec3x8> = Vec::<Vec3x8>::new();
    ray_o.resize(ELEMENT_COUNT / 8, Vec3x8::new_splat(1.0, 2.0, 3.0));

    let mut ray_d: Vec<Vec3x8> = Vec::<Vec3x8>::new();
    ray_d.resize(ELEMENT_COUNT / 8, Vec3x8::new_splat(1.0, 2.0, 3.0));

    let mut results: Vec<f32x8> = Vec::<f32x8>::new();
    results.resize(ELEMENT_COUNT / 8, f32x8::splat(1.0));

    let now = Instant::now();
    for _test_index in 0..test_count {
        for element_index in 0..ELEMENT_COUNT / 8 {
            results[element_index] = ray_sphere_intersect_x8(&sphere_o_vec[element_index], &sphere_r_sq_vec[element_index], &ray_o[element_index], &ray_d[element_index]);
        }
    }
    let elapsed_time = now.elapsed();

    (elapsed_time.as_nanos() as f64 / test_count as f64) as f64
}

// Make a test program of ray intersection for multiple rays vs single sphere
// or multiple rays vs multiple spheres
fn main() {
    let test_count = 1000;

    // Single classic ray intersection
    let classic_time = ray_sphere_intersect_test(test_count);
    println!("Classic (ns): {}", classic_time);

    // Structure of Arrays ray intersection
    let structure_of_arrays_time = ray_sphere_intersect_soa_test(test_count);
    println!("Structure of Arrays (ns): {}", structure_of_arrays_time);

    // SIMD4 intersection
    let simd_x4_time = ray_sphere_intersect_x4_test(test_count);
    println!("SIMDx4 (ns): {}", simd_x4_time);

    // SIMD8 intersection
    let simd_x8_time = ray_sphere_intersect_x8_test(test_count);
    println!("SIMDx8 (ns): {}", simd_x8_time);

}
