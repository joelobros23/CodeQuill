# Project Plan: CodeQuill

**Description:** A collaborative, real-time code editor with advanced features like code completion and syntax highlighting, built for performance and scalability.


## Development Goals

- [ ] Set up the basic Actix Web server in `src/main.rs` with a health check endpoint and WebSocket route.
- [ ] Create a WebSocket actor in `src/websocket.rs` to handle client connections and message passing.
- [ ] Implement data models (e.g., `Document`, `User`, `Message`) in `src/models.rs` for managing code documents and user interactions.
- [ ] Design a message format for WebSocket communication to handle operations like code updates, cursor movements, and user presence.
- [ ] Implement logic within the WebSocket actor to handle incoming messages, update the document state, and broadcast changes to all connected clients.
- [ ] Add basic user authentication (e.g., using a simple token-based system) to identify users connected to the editor.
- [ ] Implement a mechanism for creating and managing code documents, potentially storing them in memory or a simple database.
- [ ] Implement rudimentary code completion suggestions based on simple prefix matching (can be expanded later with more sophisticated techniques).
- [ ] Integrate basic syntax highlighting using a suitable Rust crate for lexing and tokenizing code (e.g., `syntect`).
- [ ] Implement graceful handling of WebSocket disconnections and cleanup resources.
