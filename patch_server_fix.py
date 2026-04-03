import re

with open('./srcs/server/dashboard/server.go', 'r') as f:
    content = f.read()

content = content.replace("func (s *DashboardServer)", "func (s *Server)")

with open('./srcs/server/dashboard/server.go', 'w') as f:
    f.write(content)
