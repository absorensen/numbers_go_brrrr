use std::{thread::{self, JoinHandle}, sync::{Mutex, Arc}};

use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha8Rng;
use ultraviolet::Vec3;

#[derive(Clone)]
pub struct GeometryMachinery {
    gizmo_progress: u64,
    gizmo_count: u64,
    completed: bool,
}

impl GeometryMachinery {
    pub fn new(gizmo_count: u64) -> Self {
        Self {
            gizmo_progress: 0,
            gizmo_count,
            completed: false,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.completed
    }

    pub fn add_work(&mut self, work_amount: u64) {
        if self.gizmo_count <= self.gizmo_progress { return; }
        self.gizmo_progress += work_amount;
        if self.gizmo_count <= self.gizmo_progress {
            self.completed = true;
        }
    }

    // Shamelessly stolen from the ultraviolet examples
    fn gizmo_work(
        ray_o: Vec3,
        ray_d: Vec3,
        sphere_o: Vec3,
        sphere_r_sq: f32,
    ) -> f32 {
        let oc = ray_o - sphere_o;
        let b = oc.dot(ray_d);
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

    pub fn make_gizmos(gizmo_data: &Vec<GizmoData>, amount: u64) -> u64 {
        for _ in 0..amount {
            for data in gizmo_data {
                Self::gizmo_work(data.ray_o, data.ray_d, data.sphere_o, data.sphere_r_sq);
            }
        }

        amount
    }
}

pub struct GizmoData {
    ray_o: Vec3,
    ray_d: Vec3,
    sphere_o: Vec3,
    sphere_r_sq: f32,
}

impl GizmoData {
    pub fn new() -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(1337);

        let ray_o = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let ray_d = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let sphere_o = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let sphere_r_sq = rng.gen();

        Self { ray_o, ray_d, sphere_o, sphere_r_sq }
    }
}

pub fn work_on_geometry_machinery(thread_count: u64, work_per_gizmo: u64, gizmo_count: u64) -> GeometryMachinery {
    let machine = GeometryMachinery::new(gizmo_count);
    let machinery_control = Arc::new(Mutex::new(machine));

    let mut gizmo_data: Vec<GizmoData> = Vec::<GizmoData>::new();
    for _ in 0..work_per_gizmo {
        gizmo_data.push(GizmoData::new());
    }

    let gizmo_data = Arc::new(gizmo_data);

    let mut thread_handles: Vec<JoinHandle<()>> = Vec::<JoinHandle<()>>::new();
    for _ in 0..thread_count {
        let local_control = machinery_control.clone();
        let gizmo_data = gizmo_data.clone();
        let handle = thread::spawn(move || {
            loop {
                {
                    let access = local_control.lock().expect("Failed to get lock in work_on_geometry_machine");
                    if access.is_complete() { break; }
                }

                let amount = GeometryMachinery::make_gizmos(&gizmo_data, 1);

                {
                    let mut access = local_control.lock().expect("Failed to get lock in work_on_geometry_machine");
                    access.add_work(amount);
                }
            }
        });
        thread_handles.push(handle);
    }
    
    for handle in thread_handles {
        let _ =handle.join();
    }

    let machine = machinery_control.lock().expect("Failed to get lock around geometry machinery").clone();

    machine
}