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

#[derive(Clone, Component, Copy, Debug, PartialEq)] 
enum SolarSystemModel { 
    Ptolemaic(ModelConfig), 
    Copernican(ModelConfig), 
    Tychonic(ModelConfig), 
    Keplerian(ModelConfig), 
    Contemporary(ModelConfig),
}

impl Default for SolarSystemModel {
    fn default() -> Self { 
        Self { 

        }
    }
}


impl SolarSystemModel { 
    fn config(&self) -> ModelConfig { 
        ModelConfig {
        } 
    } 
}

#[derive(Clone, Component, Copy, Debug, Default, PartialEq)] 
struct SpacetimeMesh { 
    enabled: bool, 
}

#[derive(Clone, Component, Copy, Debug, Default, PartialEq)] 
struct Planet { 
    name: &'static str, 
    distance_from_sun: f32, 
    obliquity: f32, 
    equatorial_radius: f32, 
    polar_radius: f32, 
    flattening: f32, 
    sidereal_rotation: f32, 
    orbital_period: f32, 
    mean_solar_distance: f32, 
}

#[derive(Clone, Copy, Component, Debug, Default, PartialEq)]
struct ModelConfig {
    planets: Vec<Planet>, 
}

#[derive(Clone, Component, Copy, Debug, Default, PartialEq)] 
struct HistoricalEpoch { 
}

fn main() {
    let mut app = App::new(); 
    app.add_plugins((
            DefaultPlugins, 
            FeathersPlugins, 
    )); 

}
