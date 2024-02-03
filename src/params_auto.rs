/// This file is auto generated by the build.rs file.
use faust_types::ParamIndex;
#[derive(Params)]
struct GainFaustNihPlugParams {
// nr of params: 9
    #[id = "input_gain"]
    input_gain: FloatParam,
    #[id = "strength"]
    strength: FloatParam,
    #[id = "thresh"]
    thresh: FloatParam,
    #[id = "attack"]
    attack: FloatParam,
    #[id = "attack_shape"]
    attack_shape: FloatParam,
    #[id = "release"]
    release: FloatParam,
    #[id = "release_shape"]
    release_shape: FloatParam,
    #[id = "knee"]
    knee: FloatParam,
    #[id = "link"]
    link: FloatParam
}

impl Default for GainFaustNihPlugParams {
    fn default() -> Self {
        Self {
            input_gain: FloatParam::new("input_gain", 0.0, FloatRange::Linear { min: -24.0, max: 24.0}),
            strength: FloatParam::new("strength", 100.0, FloatRange::Linear { min: 0.0, max: 100.0}),
            thresh: FloatParam::new("thresh", -1.0, FloatRange::Linear { min: -30.0, max: 0.0}),
            attack: FloatParam::new("attack", 30.0, FloatRange::Linear { min: 0.0, max: 100.0}),
            attack_shape: FloatParam::new("attack_shape", 2.0, FloatRange::Linear { min: -4.0, max: 4.0}),
            release: FloatParam::new("release", 42.0, FloatRange::Linear { min: 1.0, max: 1000.0}),
            release_shape: FloatParam::new("release_shape", -3.0, FloatRange::Linear { min: -4.0, max: 4.0}),
            knee: FloatParam::new("knee", 2.0, FloatRange::Linear { min: 0.0, max: 30.0}),
            link: FloatParam::new("link", 100.0, FloatRange::Linear { min: 0.0, max: 100.0})
        }
    }
}

pub const INPUT_GAIN_PI: ParamIndex = ParamIndex(0);
pub const STRENGTH_PI: ParamIndex = ParamIndex(1);
pub const THRESH_PI: ParamIndex = ParamIndex(2);
pub const ATTACK_PI: ParamIndex = ParamIndex(3);
pub const ATTACK_SHAPE_PI: ParamIndex = ParamIndex(4);
pub const RELEASE_PI: ParamIndex = ParamIndex(5);
pub const RELEASE_SHAPE_PI: ParamIndex = ParamIndex(6);
pub const KNEE_PI: ParamIndex = ParamIndex(7);
pub const LINK_PI: ParamIndex = ParamIndex(8);
