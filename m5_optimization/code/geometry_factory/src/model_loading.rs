pub fn load_a_bunny(path: &String) -> Vec<tobj::Model> {
    let mut options = tobj::LoadOptions::default();
    options.triangulate = true;

    let (models, _materials) =
        tobj::load_obj(&path, &options).expect("Failed to OBJ load file");

    models
}

