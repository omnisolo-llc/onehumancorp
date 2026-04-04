with open('srcs/server/dashboard/server_test.go', 'r') as f:
    content = f.read()

# the unused variables are server and rr
content = content.replace('server := &Server{} // Mock server', '_ = &Server{} // Mock server')
content = content.replace('rr := httptest.NewRecorder()', '_ = httptest.NewRecorder()')

with open('srcs/server/dashboard/server_test.go', 'w') as f:
    f.write(content)
