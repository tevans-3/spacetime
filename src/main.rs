//! A chronological visualization of the evolution of solar system models throughout history.  

use bevy::prelude::*; 
use bevy::{ 
    feathers::{controls::{FeathersSlider}, theme::UiTheme, FeathersPlugins}, 
    ui_widgets::{slider_self_update, SliderPrecision, SliderStep, SliderValue, 
        ValueChange, 
    }, 
}; 

#[derive(Resource, Default)] 
struct AppStatus { 
    model: SolarSystemModel, 
}

#[derive(Clone, Component, Copy, Debug, Default, PartialEq)] 
enum SolarSystemModel { 
    #[default]
    Ptolemaic, 
    Copernican, 
    Tychonic, 
    Keplerian, 
    Contemporary,
}

impl SolarSystemModel {
    fn get(&self) -> ModelConfig { 
        match self { 
            SolarSystemModel::Ptolemaic => {
                ModelConfig::ptolemaic() 
            }, 
            SolarSystemModel::Copernican => { 
                ModelConfig::copernican() 
            }, 
            SolarSystemModel::Tychonic => { 
                ModelConfig::tychonic()
            }, 
            SolarSystemModel::Keplerian => {
                ModelConfig::keplerian() 
            }, 
            SolarSystemModel::Contemporary => {
                ModelConfig::contemporary()
            }, 
        }
    }
}

#[derive(Clone, Component, Copy, Debug, PartialEq)] 
struct Planet { 
    name: &'static str, 
    obliquity: f64, 
    equatorial_radius: f64, 
    polar_radius: f64,
    flattening: f64,
    sidereal_rotation: f64, 
    orbital_period: f64,
    mean_solar_distance: f64,  
}

impl Default for Planet { 
    fn default() -> Self { 
        Self { 
            name: "", 
            obliquity: 0.0, 
            equatorial_radius: 0.0, 
            polar_radius: 0.0, 
            flattening: 0.0, 
            sidereal_rotation: 0.0, 
            orbital_period: 0.0, 
            mean_solar_distance: 0.0, 
        } 
    }
} 

impl Planet { 
    fn new(_name: &'static str) -> Self {
        Self { 
            name: _name,  
            ..default() 
        } 
    }

    fn load_data(&mut self, data: serde_json::Value) { 
        self.obliquity = data["axialTilt_deg"].as_f64().unwrap(); 
        self.equatorial_radius = data["equatorialRadius_km"].as_f64().unwrap(); 
        self.polar_radius = data["polarRadius_km"].as_f64().unwrap(); 
        self.flattening = data["flattening"].as_f64().unwrap(); 
        self.sidereal_rotation = data["siderealRotationPeriod_hours"].as_f64().unwrap(); 
        self.orbital_period = data["siderealOrbitPeriod_days"].as_f64().unwrap(); 
        self.mean_solar_distance = data["meanDistanceFromSun_millionKm"].as_f64().unwrap(); 
    }
}

#[derive(Clone, Component, Debug, Default, PartialEq)]
struct ModelConfig {
    planets: Vec<Planet>, 
}

impl ModelConfig { 
    fn ptolemaic() -> Self { 
    } 
    fn copernican() -> Self { 
    } 
    fn tychonic() -> Self { 
    } 
    fn keplerian() -> Self { 
    } 
    fn contemporary() -> Self { 
    } 
}

fn main() {
    let mut app = App::new(); 
    app.add_plugins((
            DefaultPlugins, 
            FeathersPlugins, 
    )); 

}
