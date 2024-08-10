use nih_plug::prelude::*;
use react_plug::prelude::*;

rp_params! {
    {{struct_name}}Params {
        gain: FloatParam {
            name: "Gain",
            value: util::db_to_gain(0.0),
            range: FloatRange::Linear {
                min: util::db_to_gain(-48.0),
                max: util::db_to_gain(6.0),
            },
            smoother: SmoothingStyle::Logarithmic(50.0),
            unit: " dB",
            value_to_string: formatters::v2s_f32_gain_to_db(2),
            string_to_value: formatters::s2v_f32_gain_to_db(),
        },
    }
}
