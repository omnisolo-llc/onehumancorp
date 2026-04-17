with open('srcs/server/agents/builtin/loop.go', 'r') as f:
    content = f.read()

content = content.replace('systemPrompt += fmt.Sprintf("', 'systemPrompt += fmt.Sprintf("\\n\\n[System] Current Session Cost: $%.4f", costUSD)')
content = content.replace('[System] Current Session Cost: $%.4f", costUSD)', '')

# remove the first instance of 'systemPrompt += fmt.Sprintf("\n\n[System] Current Session Cost: $%.4f", costUSD)'
content = content.replace('systemPrompt += fmt.Sprintf("\\n\\n[System] Current Session Cost: $%.4f", costUSD)', 'systemPrompt += fmt.Sprintf("\\n\\n[System] Current Session Cost: $%.4f", costUSD)', 1)

# fix the newline error
fixed = """		systemPrompt := a.System
		if a.Tracker != nil {
			_, _, _, costUSD := a.Tracker.GetMetrics()
			systemPrompt += fmt.Sprintf("\\n\\n[System] Current Session Cost: $%.4f", costUSD)
		}"""

old = """		systemPrompt := a.System
		if a.Tracker != nil {
			_, _, _, costUSD := a.Tracker.GetMetrics()
			systemPrompt += fmt.Sprintf("\\n\\n[System] Current Session Cost: $%.4f", costUSD)

"""

content = content.replace(old, fixed)

with open('srcs/server/agents/builtin/loop.go', 'w') as f:
    f.write(content)
