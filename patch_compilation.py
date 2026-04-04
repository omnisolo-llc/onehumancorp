with open('srcs/server/orchestration/task_orchestrator.go', 'r') as f:
    content = f.read()
import re
content = re.sub(r'NewAutoDreamWorker\(to.db, to.redisClient\)', 'NewAutoDreamWorker(to.db)', content)
content = re.sub(r'NewAutoDreamWorker\(to.db, nil\)', 'NewAutoDreamWorker(to.db)', content)
with open('srcs/server/orchestration/task_orchestrator.go', 'w') as f:
    f.write(content)

with open('srcs/server/orchestration/ultraplan.go', 'r') as f:
    content = f.read()
content = content.replace('Payload:   string(stateMachineJSON),', 'Content:   string(stateMachineJSON),')
content = content.replace('Status:    plan.Status,', '')
content = content.replace('Status:    newStatus,', '')
content = content.replace('m.hub.Publish(msg)', 'm.hub.PublishAgentNotification("system", msg)')
with open('srcs/server/orchestration/ultraplan.go', 'w') as f:
    f.write(content)
