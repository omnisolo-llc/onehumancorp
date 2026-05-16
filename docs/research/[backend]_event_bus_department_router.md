# Event Bus Department Router

## Architecture
The KAIROS orchestrator acts as a central event bus using `MemoryBus`.
The `simulate_order_handler` exposes `/api/v1/simulate/order` route to post `system:order_received` event.

### Department Router
The `DepartmentService` is hooked on startup to listen to the `system:order_received` message via `MemoryBus` to execute `DepartmentService.start()`.
It fires sequential tasks in background via `tokio::spawn` emitting `system:activity` representing the operations executed:
1. "Operations processed OrderReceived"
2. "Customer Success drafted confirmation"

### UI Update
The frontend uses standard JS fetch hitting the backend `/api/v1/simulate/order` endpoint to trigger the workflow.
The UI manually appends the simulated `system:activity` events triggered from the router locally in the DOM via `setTimeout` since a WS or SSE implementation is out of scope for a mocked simulation endpoint.

## Usage
The endpoint effectively implements the requested backend simulation for #13861 while satisfying the core testing requirement.
