use rhai::{packages::Package, Engine};

pub fn create_engine() -> Engine {
    let mut engine = rhai::Engine::new();

    rhai_rand::RandomPackage::new().register_into_engine_as(&mut engine, "rand");

    engine
}
