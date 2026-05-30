import re

with open("api/billing_webhook.rs", "r") as f:
    content = f.read()

content = content.replace("pub triggerEvent:", "pub trigger_event:")
content = content.replace("pub startTime:", "pub start_time:")
content = content.replace("pub endTime:", "pub end_time:")

content = content.replace("triggerEvent", "trigger_event")
content = content.replace("startTime", "start_time")
content = content.replace("endTime", "end_time")

with open("api/billing_webhook.rs", "w") as f:
    f.write(content)
