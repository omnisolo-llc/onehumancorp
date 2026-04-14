with open("srcs/server/dashboard/server.go", "r") as f:
    code = f.read()

if "/api/growth/viral-bridge" not in code:
    code = code.replace(
        'mux.HandleFunc("/api/growth/waitlist", server.handleWaitlist)',
        'mux.HandleFunc("/api/growth/waitlist", server.handleWaitlist)\n\tmux.HandleFunc("/api/growth/viral-bridge", server.handleSovereignToCloudInvite)'
    )

with open("srcs/server/dashboard/server.go", "w") as f:
    f.write(code)
