/// Embedded quantum runtime header
pub const QUANTUM_RUNTIME_H: &str = include_str!("quantum_runtime.h");

/// Write quantum runtime to current directory
pub fn write_runtime() -> std::io::Result<()> {
    std::fs::write("quantum_runtime.h", QUANTUM_RUNTIME_H)
}
