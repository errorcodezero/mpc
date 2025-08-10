use crate::vm::VM;
use axum::{
    Router,
    body::Bytes,
    http::StatusCode,
    routing::{get, post},
};
use std::{net::IpAddr, sync::{Arc, Mutex}};

// Global VM instance wrapped in Arc<Mutex<>> for thread safety
static VM_INSTANCE: std::sync::LazyLock<Arc<Mutex<VM>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(VM::default())));

pub async fn main(ip: IpAddr, port: u16) {
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/run", post(run))
        .route("/state", get(get_vm_state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();
    println!("server listening on {}:{}", ip, port);
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "mpc"
}

async fn run(body: Bytes) -> (StatusCode, String) {
    let raw_data = String::from_utf8(body.to_vec())
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UTF-8".to_string()));
    match raw_data {
        Ok(data) => {
            let trimmed_data = data.trim();
            let opcode: u16 = if trimmed_data.starts_with("0x") || trimmed_data.starts_with("0X") {
                // Parse hexadecimal (with 0x prefix)
                match u16::from_str_radix(&trimmed_data[2..], 16) {
                    Ok(parsed_opcode) => parsed_opcode,
                    Err(_) => return (StatusCode::BAD_REQUEST, "Invalid hexadecimal opcode format".to_string()),
                }
            } else {
                // Parse decimal (default behavior)
                match trimmed_data.parse() {
                    Ok(parsed_opcode) => parsed_opcode,
                    Err(_) => return (StatusCode::BAD_REQUEST, "Invalid decimal opcode format".to_string()),
                }
            };

            // Run the opcode on the VM instance
            {
                let mut vm = VM_INSTANCE.lock().unwrap();
                vm.run(opcode);
            }

            (
                StatusCode::OK,
                format!("Successfully ran opcode: {} (0x{:x})", opcode, opcode),
            )
        }
        Err(error) => error,
    }
}

async fn get_vm_state() -> (StatusCode, String) {
    let vm = VM_INSTANCE.lock().unwrap();
    
    let mut output = String::new();
    
    // Display registers (A through F based on typical naming)
    let registers = vm.get_registers();
    let register_names = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P"];
    
    for (i, name) in register_names.iter().enumerate() {
        output.push_str(&format!("Register {}: 0x{:x}\n", name, registers[i]));
    }
    
    // Display special registers
    output.push_str(&format!("Accumulator: 0x{:x}\n", vm.get_accumulator()));
    output.push_str(&format!("Instruction Pointer: 0x{:x}\n", vm.get_instruction_pointer()));
    output.push_str(&format!("Stack Pointer: 0x{:x}\n", vm.get_stack_pointer()));
    output.push_str(&format!("Base Pointer: 0x{:x}\n", vm.get_base_pointer()));
    output.push_str(&format!("Status Register: 0x{:x}\n", vm.get_status_register()));
    
    // Display non-zero memory addresses
    output.push_str("Memory:\n");
    let memory = vm.get_memory();
    for (addr, &value) in memory.iter().enumerate() {
        if value != 0 {
            output.push_str(&format!("0x{:x} -> 0x{:x}\n", addr, value));
        }
    }
    
    (StatusCode::OK, output)
}
