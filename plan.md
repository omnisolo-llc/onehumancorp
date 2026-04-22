1. **Define `SIPPayload` struct** in `srcs/server/orchestration/mesh_api.go`:
   - It will include fields `agent_id` (string), `channel` (string), `event_type` (string), and `data` (json.RawMessage) with corresponding `json` tags.

2. **Update `HandleBroadcast`** in `srcs/server/orchestration/mesh_api.go`:
   - Replace the generic `map[string]interface{}` with the new `SIPPayload` struct.
   - Decode the JSON body into this struct.
   - Add validation:
     - `agent_id` must start with `spiffe://`.
     - `channel` must be one of `mesh:tasks`, `mesh:coordination`, or `mesh:presence`.
   - If validation fails, return `http.StatusBadRequest` with a descriptive message.
   - If validation passes, marshal the struct back to JSON to broadcast.

3. **Update `HandlePublish`** in `srcs/server/orchestration/mesh_api.go`:
   - The current code decodes into `pb.MeshEvent`. Wait, the prompt says: "Update `HandleBroadcast` and `HandlePublish` to decode into this struct." Let's double check if we need to completely replace `pb.MeshEvent` with `SIPPayload` for the request parsing. I will change `HandlePublish` to decode into `SIPPayload` as instructed, validate it similarly, and marshal it back or convert it into what's needed. Wait, in `HandlePublish`:
     ```go
     var event pb.MeshEvent
     if err := json.NewDecoder(r.Body).Decode(&event); err != nil {
     ```
     I will change this to decode into `SIPPayload`. Or I can decode into `SIPPayload`, validate, and then broadcast the payload. Wait, `BroadcastMeshEvent` takes a `payload []byte`. I can marshal the `SIPPayload` struct back to JSON and broadcast. Wait, currently `HandlePublish` broadcasts to a hardcoded `"tasks"` channel. I should probably use `SIPPayload.Channel` if it's set, or the instruction doesn't specify. The prompt says: "Valid payloads are broadcasted to the correct internal mesh channel."

4. **Update tests** in `srcs/server/orchestration/mesh_api_test.go`:
   - Add test cases to verify that requests missing SIP fields, having invalid SPIFFE IDs, or having an invalid channel return a `400 Bad Request`.
   - Update existing valid test cases in `TestMeshAPI_Broadcast` and `TestMeshAPI_Publish` (if it exists) to use proper SPIFFE IDs and valid channels.

5. **Run tests**: Execute `bazelisk test //srcs/server/orchestration/...` to ensure all tests pass and coverage is met.

6. **Complete pre-commit steps**: Run `pre_commit_instructions` to ensure proper testing, verification, review, and reflection are done.

