use std::f32::consts::PI;

use lindenmayer_grammar::{
    Axiom, 
    rules, 
    TurtleBuilder, 
    TurtleAction
};
use raqote::{StrokeStyle, LineCap};

const DEPTH: i32 = 10;
const ANGLE: f32 = PI * 0.5;
const FILE_NAME: &'static str = "dragon_curve.png";
const RESOLUTION: f32 = 20.0;

fn main() -> anyhow::Result<()> {
    let mut axiom = Axiom::new(0);

    let rules = rules!(
        0 => 0 : 2 : 1,
        1 => 0 : 3 : 1
    );

    for _ in 0..DEPTH { 
        axiom.rewrite(&rules); 
    }

    use TurtleAction::*;
    let turtle = TurtleBuilder::new()
        .assign_action(0, Forward)
        .assign_action(1, Forward)
        .assign_action(2, Turn(-ANGLE))
        .assign_action(3, Turn(ANGLE))
        .build();

    let style = StrokeStyle {
        width: RESOLUTION * 0.5,
        cap: LineCap::Butt,
        ..Default::default()
    };

    axiom.visualize(turtle).save(RESOLUTION, &style, FILE_NAME)
}