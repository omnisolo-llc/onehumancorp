## Native Rust Omnichannel Chat System

**Goal**: Implement the core Rust data models, channel adapters, API endpoints, and a mobile-first UI for the native Omnichannel Chat System that replaces the external chat service.

### Data Models
We need to define Rust data models for `Inbox`, `Conversation`, `Message`, and `Contact` with strict `tenant_id` isolation.

### API Endpoints
Implement backend REST/gRPC APIs for retrieving and sending messages.

### UI
Build the Flutter/Web mobile-first UI for the Inbox and Conversation views, utilizing OHC Premium Tokens (Translucent Glass).

### Real-time messaging
Integrate a WebSocket gateway for real-time message delivery.
