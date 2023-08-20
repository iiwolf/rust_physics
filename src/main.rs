/*
Create struct to represent the state of an aircraft
Fields should include position, velocity, acceleration, and attitude (pitch, roll, yaw)
Create a function to update the state of the aircraft
Create a function to print the state of the aircraft
Create a function to calculate the distance between two aircraft
Create a function to calculate the angle between two aircraft
Create a function to calculate the relative velocity between two aircraft
Create a function to calculate the relative acceleration between two aircraft
Create a function to calculate the relative attitude between two aircraft
*/

struct State {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
    acceleration: (f64, f64, f64),
    attitude: (f64, f64, f64),
}

impl State {
    fn update(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
        self.velocity.2 += self.acceleration.2;
    }

    fn print(&self) {
        println!("Position: {:?}", self.position);
        println!("Velocity: {:?}", self.velocity);
        println!("Acceleration: {:?}", self.acceleration);
        println!("Attitude: {:?}", self.attitude);
    }

    fn distance(&self, other: &State) -> f64 {
        let dx = self.position.0 - other.position.0;
        let dy = self.position.1 - other.position.1;
        let dz = self.position.2 - other.position.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    fn angle(&self, other: &State) -> f64 {
        let dx = self.position.0 - other.position.0;
        let dy = self.position.1 - other.position.1;
        let dz = self.position.2 - other.position.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    fn relative_velocity(&self, other: &State) -> (f64, f64, f64) {
        (
            self.velocity.0 - other.velocity.0,
            self.velocity.1 - other.velocity.1,
            self.velocity.2 - other.velocity.2,
        )
    }

    fn relative_acceleration(&self, other: &State) -> (f64, f64, f64) {
        (
            self.acceleration.0 - other.acceleration.0,
            self.acceleration.1 - other.acceleration.1,
            self.acceleration.2 - other.acceleration.2,
        )
    }

    fn relative_attitude(&self, other: &State) -> (f64, f64, f64) {
        (
            self.attitude.0 - other.attitude.0,
            self.attitude.1 - other.attitude.1,
            self.attitude.2 - other.attitude.2,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update() {
        let mut state = State {
            position: (0.0, 0.0, 0.0),
            velocity: (1.0, 1.0, 1.0),
            acceleration: (1.0, 1.0, 1.0),
            attitude: (1.0, 1.0, 1.0),
        };
        state.update();
        assert_eq!(state.position, (1.0, 1.0, 1.0));
        assert_eq!(state.velocity, (2.0, 2.0, 2.0));
    }

    #[test]
    fn test_distance() {
        let state1 = State {
            position: (0.0, 0.0, 0.0),
            velocity: (0.0, 0.0, 0.0),
            acceleration: (0.0, 0.0, 0.0),
            attitude: (0.0, 0.0, 0.0),
        };
        let state2 = State {
            position: (1.0, 1.0, 1.0),
            velocity: (0.0, 0.0, 0.0),
            acceleration: (0.0, 0.0, 0.0),
            attitude: (0.0, 0.0, 0.0),
        };
        assert_eq!(state1.distance(&state2), 1.7320508075688772);
    }

    #[test]
    fn test_angle() {
        let state1 = State {
            position: (0.0, 0.0, 0.0),
            velocity: (0.0, 0.0, 0.0),
            acceleration: (0.0, 0.0, 0.0),
            attitude: (0.0, 0.0, 0.0),
        };
        let state2 = State {
            position: (1.0, 1.0, 1.0),
            velocity: (0.0, 0.0, 0.0),
            acceleration: (0.0, 0.0, 0.0),
            attitude: (0.0, 0.0, 0.0),
        };
        assert_eq!(state1.angle(&state2), 1.7320508075688772);
    }
}


fn main() {
    println!("Hello, world!");
}
