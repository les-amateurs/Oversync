use async_trait::async_trait;
// bruh we need extra dep for async trait
#[async_trait]
pub trait Service {
    async fn recieve(&self);
    
    async fn start(&mut self){
        println!("Starting empty service :(");
    }
}