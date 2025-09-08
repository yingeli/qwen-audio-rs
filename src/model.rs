use futures::io::AsyncBufRead;
use anyhow::Result;

pub struct Model {

}

impl Model {
    pub fn new() -> Self {
        Model {}
    }

    pub async fn generate<R: AsyncBufRead>(&self, input: &str, audio: R) -> Result<String> {
        Ok("This is a model.".to_string())
    }
}