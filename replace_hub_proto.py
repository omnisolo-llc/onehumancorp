import sys

def replace_hub_proto():
    with open('srcs/proto/hub.proto', 'r') as f:
        content = f.read()

    new_messages = """
message SwarmTask {
  string id = 1;
  string mission_id = 2;
  string title = 3;
  string status = 4;
  string assigned_agent_id = 5;
  int64 locked_until_unix = 6;
  string payload = 7;
  int64 created_at_unix = 8;
  int64 updated_at_unix = 9;
}

message ClaimTaskRequest {
  string task_id = 1;
  string agent_id = 2;
}

message ClaimTaskResponse {
  bool success = 1;
  SwarmTask task = 2;
}
"""

    new_rpc = "  rpc ClaimTask(ClaimTaskRequest) returns (ClaimTaskResponse);\n"

    content = content.replace("service HubService {", new_messages + "\nservice HubService {")
    content = content.replace("rpc DelegateSubTask(SubTask) returns (DelegateTaskResponse);", "rpc DelegateSubTask(SubTask) returns (DelegateTaskResponse);\n" + new_rpc)

    with open('srcs/proto/hub.proto', 'w') as f:
        f.write(content)

replace_hub_proto()
