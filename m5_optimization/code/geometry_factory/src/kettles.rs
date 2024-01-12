use rand::Rng;

pub fn put_the_kettle_on(target_temperature: f32, max_time_steps: u32) -> bool {
    let rise_probability = 0.0001;
    let temperature_delta = 0.5;
    let catastrophic_failure_probability = 1.0 / (target_temperature / temperature_delta) / 1.5;
    let mut rng = rand::thread_rng();

    let mut temperature: f32 = 0.0;
    for _ in 0..max_time_steps {
        if target_temperature <= temperature {
            break;
        }

        let entropic_metaphores: f32 = rng.gen();
        if entropic_metaphores < rise_probability {
            let entropic_metaphores: f32 = rng.gen();
            if entropic_metaphores < catastrophic_failure_probability {
                return false;
            }

            temperature += temperature_delta;
        }
    }

    target_temperature <= temperature
}