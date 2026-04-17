use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::Filter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    pub method: String,
    pub params: HashMap<String, String>,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    pub result: Option<String>,
    pub error: Option<String>,
    pub id: u64,
}

pub struct RpcServer {
    port: u16,
    handlers: Arc<Mutex<HashMap<String, fn(HashMap<String, String>) -> Result<String, String>>>>,
}

impl RpcServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_method<F>(&self, method: &str, handler: F)
    where
        F: Fn(HashMap<String, String>) -> Result<String, String> + 'static,
    {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.insert(method.to_string(), handler);
    }

    pub async fn start(&self) {
        let handlers = self.handlers.clone();
        
        let route = warp::post()
            .and(warp::body::json())
            .map(move |req: RpcRequest| {
                let handlers = handlers.lock().unwrap();
                match handlers.get(&req.method) {
                    Some(handler) => {
                        let result = handler(req.params);
                        RpcResponse {
                            result: result.ok(),
                            error: result.err(),
                            id: req.id,
                        }
                    }
                    None => RpcResponse {
                        result: None,
                        error: Some("Method not found".to_string()),
                        id: req.id,
                    }
                }
            });

        warp::serve(route).run(([0, 0, 0, 0], self.port)).await;
    }
}
