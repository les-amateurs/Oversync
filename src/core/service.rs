use std::sync::Arc;

use async_trait::async_trait;

use super::messaging::ServiceMessage;
// bruh we need extra dep for async trait
#[async_trait]
pub trait Service {
    async fn recieve(&self, message: Arc<&ServiceMessage>) {

    }

    async fn start(&mut self) {
        println!("Starting empty service :(");
    }
}
