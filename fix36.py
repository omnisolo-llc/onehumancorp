# Ah, I removed `Region` from `hireRequest`? No, I removed it from `agent := handoff.Agent{ ... Region: req.Region }`.
# But wait! If `dec.DisallowUnknownFields()` is used in `handlers_agent.go`, and the test sends `{"name": "...", "role": "...", "providerType": "..."}`, then `dec.Decode()` will parse it perfectly.
# Why is the test failing? "expected newly hired agent in snapshot agents"
# Let's run the test with log prints.

with open('srcs/dashboard/handlers_agent.go', 'r') as f:
    content = f.read()

# Let's print out what err is if any
import sys

# It seems `handleHireAgent` succeeds! It says `writeJSON(w, snapshot)`.
# But the test checks if `ag["name"] == "Claude SWE"`.
# Did the test hire "Claude SWE" or "Claude SWE Agent"?
