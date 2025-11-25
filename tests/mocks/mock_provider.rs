use async_trait::async_trait;
use wapp::providers::ApiProvider;

pub struct MockProvider {
    pub response: String,
}

#[async_trait]
impl ApiProvider for MockProvider {
    async fn get_data(&self, _city: String, _when: String) -> anyhow::Result<String> {
        Ok(self.response.clone())
    }
}
