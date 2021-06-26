pub mod algos;
pub mod math;
pub mod swerve;

use crate::algos::control::Control;
use crate::math::point::Point;
use crate::math::position::Position;

use uom::si::angle::radian;
use uom::si::angular_velocity::radian_per_second;
use uom::si::f64::*;
use uom::si::length::foot;
use uom::si::time::second;
use uom::si::velocity::foot_per_second;

fn main() {
    let p1 = Point {
        x: Length::new::<foot>(5.0) / Time::new::<second>(2.0),
        y: Length::new::<foot>(5.0) / Time::new::<second>(2.0),
    };

    println!("{}", p1.x.value);

    test_swerve_control();
}

fn test_swerve_control() {
    run_control_simulation(0.0, 0.0, -2.0);
    // Need to put JUnit tests
}

fn run_control_simulation(xv: f64, yv: f64, b: f64) {
    let mut control = Control {
        radius_vecs: vec![
            Point::<Length>::new(Length::new::<foot>(0.0), Length::new::<foot>(0.0)),
            Point::<Length>::new(Length::new::<foot>(0.0), Length::new::<foot>(0.0)),
            Point::<Length>::new(Length::new::<foot>(0.0), Length::new::<foot>(0.0)),
            Point::<Length>::new(Length::new::<foot>(0.0), Length::new::<foot>(0.0)),
        ],
        pos: Position {
            x: Length::new::<foot>(0.0),
            y: Length::new::<foot>(0.0),
            bearing: Angle::new::<radian>(0.0),
        },
        radius_width: Length::new::<foot>(2.0),
        radius_length: Length::new::<foot>(2.0),
    };

    let module_angles = control.get_swerve_module_angles(
        Point::<Velocity> {
            x: Velocity::new::<foot_per_second>(xv),
            y: Velocity::new::<foot_per_second>(yv),
        },
        AngularVelocity::new::<radian_per_second>(b),
    );

    println!(
        "######## new test ####### \n\n top right: v: {}, b:{} \n\n bottom right: v: {}, b:{} \n\n bottom left: v: {}, b:{} \n\n top left: v: {}, b:{}",
        module_angles.tr.velocity.value,
        module_angles.tr.bearing.value,
        module_angles.br.velocity.value,
        module_angles.br.bearing.value,
        module_angles.bl.velocity.value,
        module_angles.bl.bearing.value,
        module_angles.tl.velocity.value,
        module_angles.tl.bearing.value,
    );
}
