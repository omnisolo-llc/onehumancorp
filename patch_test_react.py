import re

with open("apps/web/test/wizard/BusinessSetupWizard.test.tsx", "r") as f:
    content = f.read()

# Make sure it actually waits for the animation frame to be applied so the assertions pass.
# Wait, the tests are already passing.
