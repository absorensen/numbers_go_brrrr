use rand::Rng;
use tobj::Mesh;
use ultraviolet::Vec3;

fn get_random_barycentric_point() -> Vec3 {
    let mut rng = rand::thread_rng();

    let sqrt_r1: f32 = rng.gen_range(0.0f32..1.0f32).sqrt();
    let r2: f32 = rng.gen_range(0.0f32..1.0f32);

    Vec3::new(1.0 - sqrt_r1, sqrt_r1 * (1.0 - r2), r2 * sqrt_r1)
}

fn get_triangle_area(v0: &Vec3, v1: &Vec3, v2: &Vec3) -> f32 {
	let a = v1.x * v0.y - v2.x * v0.y - v0.x * v1.y + v2.x * v1.y + v0.x * v2.y - v1.x * v2.y;
	let b = v1.x * v0.z - v2.x * v0.z - v0.x * v1.z + v2.x * v1.z + v0.x * v2.z - v1.x * v2.z;
	let c = v1.y * v0.z - v2.y * v0.z - v0.y * v1.z + v2.y * v1.z + v0.y * v2.z - v1.y * v2.z;

    0.5 * (a*a + b*b + c*c).sqrt()
}

pub fn point_sample_model(mesh: &Mesh, point_sampling_factor: f32) -> Vec<Vec3> {
    let mut point_cloud: Vec<Vec3> = Vec::<Vec3>::new();

    let face_count = mesh.indices.len() / 3;

    for face_index in 0..face_count {
        let vertex_0_x = mesh.positions[(mesh.indices[face_index * 3 + 0] + 0) as usize];
        let vertex_0_y = mesh.positions[(mesh.indices[face_index * 3 + 0] + 1) as usize];
        let vertex_0_z = mesh.positions[(mesh.indices[face_index * 3 + 0] + 2) as usize];
        let vertex_1_x = mesh.positions[(mesh.indices[face_index * 3 + 1] + 0) as usize];
        let vertex_1_y = mesh.positions[(mesh.indices[face_index * 3 + 1] + 1) as usize];
        let vertex_1_z = mesh.positions[(mesh.indices[face_index * 3 + 1] + 2) as usize];
        let vertex_2_x = mesh.positions[(mesh.indices[face_index * 3 + 2] + 0) as usize];
        let vertex_2_y = mesh.positions[(mesh.indices[face_index * 3 + 2] + 1) as usize];
        let vertex_2_z = mesh.positions[(mesh.indices[face_index * 3 + 2] + 2) as usize];

        let mut vertex_0 = Vec3::broadcast(0.0);
        vertex_0.x = vertex_0_x;
        vertex_0.y = vertex_0_y;
        vertex_0.z = vertex_0_z;

        let mut vertex_1 = Vec3::broadcast(0.0);
        vertex_1.x = vertex_1_x;
        vertex_1.y = vertex_1_y;
        vertex_1.z = vertex_1_z;

        let mut vertex_2 = Vec3::broadcast(0.0);
        vertex_2.x = vertex_2_x;
        vertex_2.y = vertex_2_y;
        vertex_2.z = vertex_2_z;

        point_cloud.push(vertex_0);
        point_cloud.push(vertex_1);
        point_cloud.push(vertex_2);

        let sample_count = (get_triangle_area(&vertex_0, &vertex_1, &vertex_2) * point_sampling_factor).round() as u32;
        for _ in 0..sample_count {
            let random_barycentric_point = get_random_barycentric_point();
            let random_world_space_point = random_barycentric_point.x * vertex_0 + random_barycentric_point.y * vertex_1 + random_barycentric_point.z * vertex_2;
            point_cloud.push(random_world_space_point);
        }
    }

    point_cloud
}

pub fn displace_point_cloud_relative_to_center(displacement_factor: f32, point_cloud: &mut Vec<Vec3>) {
    let point_count: usize = point_cloud.len();
    let displacement_factor: Vec3 = Vec3::broadcast(displacement_factor);
    let mut mean: Vec3 = Vec3::zero();

    for point in point_cloud.iter() {
        mean += *point / point_count as f32;
    }

    for point in point_cloud {
        *point += (*point - mean) * displacement_factor;
    }
}