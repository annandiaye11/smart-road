use crate::vehicle::Vehicle;

pub struct Intersection {
    pub vehicles: Vec<Vehicle>,
}

impl Intersection {
    pub fn new() -> Self {
        Self {
            vehicles: Vec::new(),
        }
    }

    pub fn add_vehicle(&mut self, vehicle: Vehicle) {
        self.vehicles.push(vehicle);
    }

    pub fn update(&mut self, delta_time: f64) {
        for vehicle in &mut self.vehicles {
            vehicle.update_position(delta_time);
        }
    }
}
