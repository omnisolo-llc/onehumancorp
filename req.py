with open("srcs/server/db/BUILD.bazel", "r") as f:
    content = f.read()
    if '"migrations/031_agent_missions_updated_at.sql",' in content:
        print("FOUND")
    else:
        print("NOT FOUND")
