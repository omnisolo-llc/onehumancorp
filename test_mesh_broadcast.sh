curl -X POST http://localhost:8080/api/mesh/broadcast -H "Content-Type: application/json" -d '{"channel":"mesh:tasks", "agent_id":"123", "action":"test", "status":"test", "payload":"test"}'
