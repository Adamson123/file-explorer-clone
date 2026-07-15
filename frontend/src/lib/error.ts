export class RustError extends Error {
    constructor(message: string) {
        super();
        this.message = message;
        this.name = "Rust_Backend_Error";
    }
}
