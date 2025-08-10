use axum::{
    body::Bytes,
    routing::{get, post},
    http::StatusCode,
    Router,
};
use std::sync::{Arc, Mutex};
use crate::vm::VM;

// Global VM instance wrapped in Arc<Mutex<>> for thread safety
static VM_INSTANCE: std::sync::LazyLock<Arc<Mutex<VM>>> = std::sync::LazyLock::new(|| {
    Arc::new(Mutex::new(VM::default()))
});

pub async fn main() {
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/run", post(run));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    return "npc 1.0"
}

async fn run(
    body: Bytes
) -> (StatusCode, String) {
    let raw_data = String::from_utf8(body.to_vec())
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UTF-8".to_string()));
    match raw_data {
        Ok(data) => {
            let opcode: u16 = match data.trim().parse() {
                Ok(parsed_opcode) => parsed_opcode,
                Err(_) => return (StatusCode::BAD_REQUEST, "Invalid opcode format".to_string())
            };
            
            // Run the opcode on the VM instance
            {
                let mut vm = VM_INSTANCE.lock().unwrap();
                vm.run(opcode);
            }
            
            return (StatusCode::OK, format!("Successfully ran opcode: {}", opcode))
        }
        Err(error) => return error
    }
}