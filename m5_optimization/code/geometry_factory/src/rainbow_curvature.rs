use ultraviolet::Vec3;

use crate::{workers::Worker, geometry_machinery::GeometryMachinery};


pub fn calculate_the_curvature_of_the_rainbow(curvature_at_point: &Vec3, kettle_boiled_in_time: bool, workers: &Vec<Worker>, point_cloud: &Vec<Vec3>, particles: &Vec<Vec3>, geometry_machinery: &GeometryMachinery) -> Vec3 {
    if !geometry_machinery.is_complete() {
        println!("Geometry machinery isn't complete. Punish the workers!");
        return Vec3::zero();
    }
    
    let mut curvature: Vec3 = Vec3::zero();

    let worker_index: usize = (curvature_at_point.x * workers.len() as f32) as usize;
    let point_cloud_index: usize = (curvature_at_point.y * point_cloud.len() as f32) as usize;
    let particles_index: usize = (curvature_at_point.z * particles.len() as f32) as usize;

    curvature.x = 0.5 * workers[worker_index].real_money + 0.5 * workers[worker_index].real_money * particles[particles_index >> 1].y * kettle_boiled_in_time as u8 as f32;
    curvature.y = 0.5 * point_cloud[point_cloud_index].y + 0.5 * point_cloud[point_cloud_index].y * workers[worker_index >> 2].real_money * kettle_boiled_in_time as u8 as f32;
    curvature.z = 0.5 * particles[particles_index].z + 0.5 * particles[particles_index].z * point_cloud[point_cloud_index >> 1].x * kettle_boiled_in_time as u8 as f32;

    // Maybe it was the friends we made on the way?
    curvature
}