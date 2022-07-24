use noise::{NoiseFn, Perlin, Seedable};

pub const HEIGHT_SEED: u32 = 1; // for building height
pub const WIDTH_SEED: u32 = 116; // for building width

pub struct HeightNoiseFn{
    pub function: Perlin,
}

impl Default for HeightNoiseFn{
    fn default() -> Self {
        let mut perlin = Perlin::new();
        perlin = Seedable::set_seed(perlin, HEIGHT_SEED);
        let base = HeightNoiseFn{
            function: perlin,
        };
        base
    }
}

