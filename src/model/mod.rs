mod errors;

pub use errors::ModelError;
use tch::Tensor;

pub struct Model {
    pub model: tch::CModule,
    pub predicted: usize,
    pub probabilities: [f32; 10],
    pub highest_probability: f32,
}

impl Model {
    pub fn new(model_path: &str) -> Result<Self, ModelError> {
        if let Ok(model) = tch::CModule::load(model_path) {
            return Ok(Self {
                model,
                predicted: 0,
                probabilities: [0.; 10],
                highest_probability: 0.,
            });
        }

        Err(ModelError::UnableToOpenModel(model_path.to_string()))
    }

    pub fn predict(&mut self, image: &Tensor) {
        let predictions = self
            .model
            .forward_ts(&[image.unsqueeze(0)])
            .unwrap()
            .exp()
            .get(0);
        for i in 0..10 {
            self.probabilities[i] = predictions.get(i as i64).into();

            if self.probabilities[i] > self.highest_probability {
                self.highest_probability = self.probabilities[i];
                self.predicted = i;
            }
        }
    }
}
