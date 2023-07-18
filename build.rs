use std::{
    io::{self, Write},
    path::Path,
};

use faust_types::{FaustDsp, UI};
include!("src/dsp.rs");

#[derive(Debug)]
#[allow(unused)]
enum Param {
    Normal {
        label: String,
        param: i32,
        init: f32,
        min: f32,
        max: f32,
        step: f32,
    },
}

#[derive(Debug)]
struct CollectParameters {
    collected: Vec<Param>,
}

impl CollectParameters {
    pub fn new() -> Self {
        Self { collected: vec![] }
    }

    pub fn write_nih_params_struct(&self, to: &Path, struct_name: &str) -> io::Result<()> {
        let mut file = std::fs::File::create(to)?;
        let mut content = format!("#[derive(Params)]\nstruct {} {{\n", struct_name);
        for (index, parameter) in self.collected.iter().enumerate() {
            let is_last = index == self.collected.len() - 1;
            match parameter {
                Param::Normal { label, .. } => {
                    content += &format!("    #[id = \"{}\"]\n", label);
                    content += &format!("    {}: FloatParam", label.to_lowercase());
                    content += &format!("{}\n", if is_last { "" } else { "," });
                }
            }
        }
        content += "}\n\n";
        content += &format!("impl Default for {} {{\n", struct_name);
        content += "    fn default() -> Self {\n";
        content += "        Self {\n";
        for (index, parameter) in self.collected.iter().enumerate() {
            let is_last = index == self.collected.len() - 1;
            match parameter {
                Param::Normal {
                    label,
                    init,
                    min,
                    max,
                    ..
                } => {
                    // TODO: Properly format floats, {:.01} is just a hack
                    content += &format!("            {}: FloatParam::new(\"{}\", {:.01}, FloatRange::Linear {{ min: {:.01}, max: {:.01}}})", label.to_lowercase(), label, init, min, max);
                    content += &format!("{}\n", if is_last { "" } else { "," });
                }
            }
        }

        content += "        }\n";
        content += "    }\n";
        content += "}\n";

        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl UI<f32> for CollectParameters {
    fn open_tab_box(&mut self, _label: &str) {}

    fn open_horizontal_box(&mut self, _label: &str) {}

    fn open_vertical_box(&mut self, _label: &str) {}

    fn close_box(&mut self) {}

    fn add_button(&mut self, _label: &str, _param: faust_types::ParamIndex) {}

    fn add_check_button(&mut self, _label: &str, _param: faust_types::ParamIndex) {}

    fn add_vertical_slider(
        &mut self,
        label: &str,
        param: faust_types::ParamIndex,
        init: f32,
        min: f32,
        max: f32,
        step: f32,
    ) {
        self.collected.push(Param::Normal {
            label: label.to_string(),
            param: param.0,
            init,
            min,
            max,
            step,
        })
    }

    fn add_horizontal_slider(
        &mut self,
        label: &str,
        param: faust_types::ParamIndex,
        init: f32,
        min: f32,
        max: f32,
        step: f32,
    ) {
        self.collected.push(Param::Normal {
            label: label.to_string(),
            param: param.0,
            init,
            min,
            max,
            step,
        })
    }

    fn add_num_entry(
        &mut self,
        label: &str,
        param: faust_types::ParamIndex,
        init: f32,
        min: f32,
        max: f32,
        step: f32,
    ) {
        self.collected.push(Param::Normal {
            label: label.to_string(),
            param: param.0,
            init,
            min,
            max,
            step,
        })
    }

    fn add_horizontal_bargraph(
        &mut self,
        _label: &str,
        _param: faust_types::ParamIndex,
        _min: f32,
        _max: f32,
    ) {
    }

    fn add_vertical_bargraph(
        &mut self,
        _label: &str,
        _param: faust_types::ParamIndex,
        _min: f32,
        _max: f32,
    ) {
    }

    fn declare(&mut self, _param: Option<faust_types::ParamIndex>, _key: &str, _value: &str) {}
}

fn main() {
    faust_build::build_dsp_to_destination("dsp/gain.dsp", "src/dsp.rs");

    println!("cargo:rerun-if-changed=dsp");
    let mut my_ui = CollectParameters::new();
    dsp::mydsp::build_user_interface_static(&mut my_ui);
    my_ui
        .write_nih_params_struct(Path::new("src/params.rs"), "GainFaustNihPlugParams")
        .expect("Failed writing nih params");
}
