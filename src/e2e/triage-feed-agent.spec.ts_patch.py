import os
with open("src/e2e/triage-feed-agent.spec.ts", "r") as f:
    c = f.read()
c = c.replace("page.request.post(`/api/dev/simulate-triage-item?tenant_id=default`)", "page.request.post(`/api/triage/simulate?tenant_id=default`, { data: { source: 'Instagram DM', payload: { message: 'Do you have vegan chocolate cake available this weekend?' } } })")
c = c.replace("await page.goto('/dashboard');", "await page.goto('/triage');")
with open("src/e2e/triage-feed-agent.spec.ts", "w") as f:
    f.write(c)
