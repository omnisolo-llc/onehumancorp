import re

with open("srcs/server/orchestration/sync_daemon_test.go", "r") as f:
    content = f.read()

# Let's see what the original test was testing. It was testing that m1 (CLOUD_ESCALATION) is synced.
# If I changed the daemon to fetch PENDING, it will fetch m3.
# Let's change the test to match the daemon's new logic (PENDING).
content = content.replace("('m1', 'CLOUD_ESCALATION', '{\"task\":\"test-mission\", \"details\":\"[PRIVATE:secret] email is a@b.com\"}', false),\n\t\t\t('m2', 'COMPLETED', '{\"task\":\"synced-mission\"}', true),\n\t\t\t('m3', 'PENDING', '{\"task\":\"ignored\"}', false)", "('m1', 'PENDING', '{\"task\":\"test-mission\", \"details\":\"[PRIVATE:secret] email is a@b.com\"}', false),\n\t\t\t('m2', 'COMPLETED', '{\"task\":\"synced-mission\"}', true),\n\t\t\t('m3', 'IGNORED', '{\"task\":\"ignored\"}', false)")

content = content.replace('if receivedPayloads[0].Status != "CLOUD_ESCALATION" {', 'if receivedPayloads[0].Status != "PENDING" {')
content = content.replace('t.Errorf("expected status CLOUD_ESCALATION, got %s", receivedPayloads[0].Status)', 't.Errorf("expected status PENDING, got %s", receivedPayloads[0].Status)')

with open("srcs/server/orchestration/sync_daemon_test.go", "w") as f:
    f.write(content)
