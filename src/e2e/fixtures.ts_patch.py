import os
with open("src/e2e/fixtures.ts", "r") as f:
    c = f.read()
c = c.replace("await page.goto('/dashboard');", "await page.goto('/triage');")
with open("src/e2e/fixtures.ts", "w") as f:
    f.write(c)
