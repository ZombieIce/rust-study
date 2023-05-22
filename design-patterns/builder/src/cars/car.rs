use crate::components::{CarType, Engine, GpsNavigator, Transmission};

pub struct Car {
    car_type: CarType,
    seats: u8,
    engine: Engine,
    transmission: Transmission,
    gps_navigator: Option<GpsNavigator>,
    fuel: f64,
}

impl Car {
    pub fn new(
        car_type: CarType,
        seats: u8,
        engine: Engine,
        transmission: Transmission,
        gps_navigator: Option<GpsNavigator>,
        fuel: f64,
    ) -> Self {
        Self {
            car_type,
            seats,
            engine,
            transmission,
            gps_navigator,
            fuel,
        }
    }

    pub fn car_type(&self) -> CarType {
        self.car_type
    }

    pub fn seats(&self) -> u8 {
        self.seats
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn fuel(&self) -> f64{
        self.fuel
    }

    pub fn set_fuel(&mut self, fuel: f64) {
        self.fuel = fuel;
    }

    pub fn transmission(&self) -> Transmission {
        self.transmission
    }

    pub fn gps_navigator(&self) -> &Option<GpsNavigator> {
        &self.gps_navigator
    }
}
