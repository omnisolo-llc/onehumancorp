import sys

def replace_in_file(filepath, search, replace):
    with open(filepath, 'r') as f:
        content = f.read()
    content = content.replace(search, replace)
    with open(filepath, 'w') as f:
        f.write(content)

# SIPDB() might be nil or not returning the correct thing here. Let's look closely at `h` and `s.hub`.
# Wait, for `Publish` method, `h.SIPDB()` panic.
# Let's revert all these to just NewMinimaxClient if we don't have safe access to the db pool.
# Or better, we need to check if h.SIPDB() is nil.
# The code was originally: client := NewMinimaxClient(minimaxKey)

replace_in_file("srcs/server/orchestration/service.go", "client := NewCachedMinimaxClient(NewMinimaxClient(h.MinimaxAPIKey()), h.SIPDB().Provider(), nil)", "client := NewMinimaxClient(h.MinimaxAPIKey())")
replace_in_file("srcs/server/orchestration/service.go", "client := NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), h.SIPDB().Provider(), nil)", "client := NewMinimaxClient(minimaxKey)")
replace_in_file("srcs/server/orchestration/service.go", "client := NewCachedMinimaxClient(NewMinimaxClient(s.hub.MinimaxAPIKey()), s.hub.SIPDB().Provider(), nil)", "client := NewMinimaxClient(s.hub.MinimaxAPIKey())")
replace_in_file("srcs/server/dashboard/autodream.go", "client = orchestration.NewCachedMinimaxClient(orchestration.NewMinimaxClient(minimaxKey), nil, nil)", "client = orchestration.NewMinimaxClient(minimaxKey)")

print("Done")
