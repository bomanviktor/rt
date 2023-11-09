use crate::light_sources::LightSource;
use crate::objects::Object;
use crate::raytracer::{Camera, Ray, ViewPort};

#[derive(Debug)]
pub struct State {
    pub scene: Scene,
    pub camera: Camera,
    pub view_port: ViewPort,
    pub rays: Vec<Ray>,
}

impl State {}

pub struct SceneBuilder {
    width: f64,
    height: f64,
    objects: Option<Vec<Object>>,
    light_sources: Option<Vec<LightSource>>,
}

impl SceneBuilder {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            objects: None,
            light_sources: None,
        }
    }
    pub fn add_objects(&mut self, objects: Vec<Object>) {
        self.objects = Some(objects);
    }

    pub fn add_light_sources(&mut self, light_sources: Vec<LightSource>) {
        self.light_sources = Some(light_sources);
    }
}

// TODO: Remove this macro. It is allowing the unused struct fields in ´Scene`
#[allow(dead_code)]
#[derive(Debug)]
pub struct Scene {
    width: f64,
    height: f64,
    objects: Vec<Object>,
    light_sources: Vec<LightSource>,
}

impl Scene {
    pub fn build(sb: SceneBuilder) -> Self {
        Self {
            width: sb.width,
            height: sb.height,
            objects: sb.objects.unwrap_or_default(),
            light_sources: sb.light_sources.unwrap_or_default(),
        }
    }
}
