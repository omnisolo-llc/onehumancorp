import re

with open("srcs/server/orchestration/service_extra_test.go", "r") as f:
    content = f.read()

content = content.replace("hub.getShard(\\\"a\\\").subs[\\\"a\\\"]", "hub.getShard(\"a\").subs[\"a\"]")
content = content.replace("hub.getShard(\\\"a\\\").subs[", "hub.getShard(\"a\").subs[")

with open("srcs/server/orchestration/service_extra_test.go", "w") as f:
    f.write(content)
