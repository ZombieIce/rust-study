use crate::cars::Manual;
use crate::components::{CarType, Engine, GpsNavigator, Transmission};

use super::Builder;

#[derive(Default)]
pub struct CarManualBuilder {
    car_type: Option<CarType>,
    seats: Option<u8>,
    engine: Option<Engine>,
    transmission: Option<Transmission>,
    gps_navigator: Option<GpsNavigator>,
}

impl Builder for CarManualBuilder {
    type OutputType = Manual;

    fn set_car_type(&mut self, car_type: CarType) {
        self.car_type = Some(car_type);
    }

    fn set_seats(&mut self, seats: u8) {
        self.seats = Some(seats);
    }

    fn set_engine(&mut self, engine: Engine) {
        self.engine = Some(engine);
    }

    fn set_transmission(&mut self, transmission: Transmission) {
        self.transmission = Some(transmission);
    }

    fn set_gps_navigator(&mut self, gps_navigator: GpsNavigator) {
        self.gps_navigator = Some(gps_navigator);
    }

    fn build(self) -> Manual {
        Manual::new(
            self.car_type.expect("Please, set a car type"),
            self.seats.expect("Please, set a count of seats"),
            self.engine.expect("Please, set an engine"),
            self.transmission.expect("Please, set a transmission"),
            self.gps_navigator,
        )
    }
}
