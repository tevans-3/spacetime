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
                PtolemaicSolarSystem::default(), 
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

/// Parameters for the Ptolemaic model of the solar system.
///
/// The basic idea behind the Ptolemaic solar system seems to be that, starting with a bunch of
/// data, you draw circles in the sky that fit that data, then just add hacky tweaks until your
/// prediction errors seem reasonable. 
///
/// The interesting thing is that his estimates aren't actually that inaccurate, and he had
/// basically no equipment by modern standards. 
///
/// There are two types of circles: the epicycle and the deferent. The deferent is the big circle:
/// it's the orbital path traced by a body as it circles the Earth. The epicycle is a smaller
/// circle. It's a clever hack that explains variation in a planet's brightness and retrograde motion.
/// The epicycle rotates around a fixed point on the deferent, and the planet orbits that fixed
/// point, the epicycle's center. So to an observer on Earth, if the planet travels faster around
/// its epicycle then the epicycle travels around the deferent, when the planet travels along the
/// inner semicircle of the epicycle, it looks like the planet is travelling backwards. It would
/// also be closest to Earth at this point, so it would appear brighter. 
struct PtolemaicSolarSystem { 
    deferent_radius: f32, 
    epicycle_radius: f32, 
    earth_dist_to_deferent_centre: f32, 
    apogee_direction: f32, 
    mean_motion_longitude: f32,
    mean_motion_anomaly: f32, 
    epoch_lambda: f32, 
    epoch_alpha: f32, 
    latitude_hack1: f32, 
    latitude_hack2: f32, 
    latitude_hack3: f32, 
    mercury_hack: f32, 
    moon_hack: f32, 
}

/// Parameters for the Copernican model of the solar system. 
struct CopernicanSolarSystem { 
    orbital_radius: f32, // units of Earth's orbit 
    eccentricity: f32, 
    epicyclet: f32, 
    aphelion_direction: f32, // the orbit's most distant point 
    sidereal_period: f32, // one full turn around the stars 
    mean_longitude: f32, 
    orbital_tilt: f32, 
    orbit_crosses_reference_plane: f32, 
    libration_terms: Vec<f32>, 
    moon_epicycles: Vec<f32>, 
}

/// Parameters for the Tychonic model of the solar system. 
///
/// We're the center of the solar system again. 
struct TychonicSolarSystem { 
    orbital_radius: f32, // units of Earth's orbit 
    eccentricity: f32, 
    epicyclet: f32, 
    aphelion_direction: f32, // the orbit's most distant point 
    sidereal_period: f32, // one full turn around the stars 
    mean_longitude: f32, 
    orbital_tilt: f32, 
    orbit_crosses_reference_plane: f32, 
    libration_terms: Vec<f32>, 
    moon_epicycles: Vec<f32>, 
    daily_rotation_rate: f32, 
    distance_to_the_stars: f32, 
}


/// Parameters for the Keplerian model of the solar system. 
struct KeplerianSolarSystem { 
}



/// A struct for storing an astronomical body's gLTF rendering data.
#[derive(Clone, Component, Copy, Debug, PartialEq)] 
struct CelestialBody {  
    name: &'static str, 
    obliquity: f64, 
    equatorial_radius: f64, 
    polar_radius: f64,
    flattening: f64,
    sidereal_rotation: f64, 
    orbital_period: f64,
    mean_solar_distance: f64,  
}

impl Default for CelestialBody { 
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

impl CelestialBody { 
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
    CelestialBody: Vec<CelestialBody>, 
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
