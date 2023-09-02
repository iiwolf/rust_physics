use std::{f64::consts::PI};

const EPSILON: f64 = 1e-9;  // Or whatever precision you want

fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

// A struct representing state with these fields as single f64 values:
//  't',
// 'stage',
// 'x',
// 'y',
// 'vx', 'vy', 'speed', 'mach',
// 'mass', 'fuel_mass',
// 'thrust', 'drag', 'lift', 'fx', 'fy', 'ax', 'ay',
// 'alpha', 'gamma',
// 'cl', 'cd'
struct State {
    t: f64,
    stage: f64,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    speed: f64,
    mach: f64,
    mass: f64,
    fuel_mass: f64,
    thrust: f64,
    drag: f64,
    lift: f64,
    fx: f64,
    fy: f64,
    ax: f64,
    ay: f64,
    alpha: f64,
    gamma: f64,
    cl: f64,
    cd: f64,
}

impl State {

    // New constructor for State, initializing all fields to zero
    fn new() -> State {
        State {
            t: 0.0,
            stage: 0.0,
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            speed: 0.0,
            mach: 0.0,
            mass: 0.0,
            fuel_mass: 0.0,
            thrust: 0.0,
            drag: 0.0,
            lift: 0.0,
            fx: 0.0,
            fy: 0.0,
            ax: 0.0,
            ay: 0.0,
            alpha: 0.0,
            gamma: 0.0,
            cl: 0.0,
            cd: 0.0,
        }
    }
}

fn projectile_physics(initial_state: State, dt: f64, t_max: f64) -> Vec<State> {

    let capacity: usize = (t_max / dt).ceil() as usize;

    // Initialize a Vec of State structs of length capacity
    let mut state_vec: Vec<State> = Vec::with_capacity(capacity);

    // Initialize the first state in the Vec to the initial_state
    state_vec.push(initial_state);

    // Iterate through the Vec, updating each state based on the previous state
    for i in 1..capacity {
        state_vec.push(State::new());

        // Update the time
        state_vec[i].t = state_vec[i-1].t + dt;

        // Update the position
        state_vec[i].x = state_vec[i-1].x + state_vec[i-1].vx * dt;
        state_vec[i].y = state_vec[i-1].y + state_vec[i-1].vy * dt;
        if state_vec[i].y <= 0.0 {
            state_vec[i].y = 0.0;
            state_vec[i].vy = 0.0;
        }

        // Update the velocity
        state_vec[i].vx = state_vec[i-1].vx + state_vec[i-1].ax * dt;
        state_vec[i].vy = state_vec[i-1].vy + state_vec[i-1].ay * dt;

        // Update the speed
        state_vec[i].speed = (state_vec[i].vx.powi(2) + state_vec[i].vy.powi(2)).sqrt();

        // Update the mass
        state_vec[i].mass = state_vec[i-1].mass - state_vec[i-1].fuel_mass;

        // Update the fx
        state_vec[i].fx = state_vec[i-1].fx;

        // Update the fy
        state_vec[i].fy = state_vec[i-1].fy;

        // Update the ax
        state_vec[i].ax = state_vec[i-1].ax;

        // Update the ay
        state_vec[i].ay = state_vec[i-1].ay - 9.81;
    }
    state_vec
}


mod test {

    // Function testing instantiation of State
    #[test]
    fn test_state() {
        let state = super::State::new();
        assert_eq!(state.t, 0.0);
        assert_eq!(state.stage, 0.0);
        assert_eq!(state.x, 0.0);
        assert_eq!(state.y, 0.0);
        assert_eq!(state.vx, 0.0);
        assert_eq!(state.vy, 0.0);
        assert_eq!(state.speed, 0.0);
        assert_eq!(state.mach, 0.0);
        assert_eq!(state.mass, 0.0);
        assert_eq!(state.fuel_mass, 0.0);
        assert_eq!(state.thrust, 0.0);
        assert_eq!(state.drag, 0.0);
        assert_eq!(state.lift, 0.0);
        assert_eq!(state.fx, 0.0);
        assert_eq!(state.fy, 0.0);
        assert_eq!(state.ax, 0.0);
        assert_eq!(state.ay, 0.0);
        assert_eq!(state.alpha, 0.0);
        assert_eq!(state.gamma, 0.0);
        assert_eq!(state.cl, 0.0);
        assert_eq!(state.cd, 0.0);
    }

    // Function testing instantiation of State in a Vec
    #[test]
    fn test_state_vec() {
        // Initialize a Vec of State structs of length 10000
        let mut state_vec: Vec<super::State> = Vec::with_capacity(10000);
        for _ in 0..10000 {
            state_vec.push(super::State::new());
        }
    }

    // Function testing projectile_physics
    #[test]
    fn test_projectile_physics() {
        // Initialize a State struct
        let mut initial_state = super::State::new();
        initial_state.vx = 500.0;
        initial_state.vy = 500.0;


        // Initialize dt and t_max
        let dt = 0.01;
        let t_max = 100.0;

        // Call projectile_physics
        let state_vec = super::projectile_physics(initial_state, dt, t_max);
        
        // Check that the last state has the correct time (to floating point precision)
        assert!(super::approx_eq(state_vec[9999].t, 99.99, super::EPSILON));
        
        use plotly::{Plot, Scatter};
        let mut plot = Plot::new();
        let trace = Scatter::new(
            state_vec.iter().map(|state| state.x).collect(), 
            state_vec.iter().map(|state| state.y).collect()
        );
        plot.add_trace(trace);
        plot.write_html("out.html");
        plot.show();

        // // Check that the last state has the correct x position
        // assert_eq!(state_vec[9999].x, 0.0);

        // // Check that the last state has the correct y position
        // assert_eq!(state_vec[9999].y, -490.5);

        // // Check that the last state has the correct x velocity
        // assert_eq!(state_vec[9999].vx, 0.0);

        // // Check that the last state has the correct y velocity
        // assert_eq!(state_vec[9999].vy, -9.81);

        // // Check that the last state has the correct speed
        // assert_eq!(state_vec[9999].speed, 9.81);

        // // Check that the last state has the correct mass
        // assert_eq!(state_vec[9999].mass, 0.0);

        // // Check that the last state has the correct fx
        // assert_eq!(state_vec[9999].fx, 0.0);

        // // Check that the last state has the correct fy
        // assert_eq!(state_vec[9999].fy, 0.0);

        // // Check that the last state has the correct ax
        // assert_eq!(state_vec[9999].ax, 0.0);

        // // Check that the last state has the correct ay
        // assert_eq!(state_vec[9999].ay, -9.81);
    }
}


fn main() -> std::io::Result<()> {
    Ok(())
}