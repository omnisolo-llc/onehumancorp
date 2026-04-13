import sys

def replace_in_file(filepath, search, replace):
    with open(filepath, 'r') as f:
        content = f.read()
    content = content.replace(search, replace)
    with open(filepath, 'w') as f:
        f.write(content)

replace_in_file("srcs/server/orchestration/service.go", "client := NewMinimaxClient(h.MinimaxAPIKey())", "client := NewCachedMinimaxClient(NewMinimaxClient(h.MinimaxAPIKey()), h.db, h.redisClient)")
replace_in_file("srcs/server/orchestration/service.go", "client := NewMinimaxClient(minimaxKey)", "client := NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), tm.db, tm.redisClient)")
replace_in_file("srcs/server/orchestration/service.go", "client := NewMinimaxClient(s.hub.MinimaxAPIKey())", "client := NewCachedMinimaxClient(NewMinimaxClient(s.hub.MinimaxAPIKey()), s.hub.SIPDB().Provider(), s.hub.RedisClient())")
replace_in_file("srcs/server/dashboard/autodream.go", "client = orchestration.NewMinimaxClient(minimaxKey)", "client = orchestration.NewCachedMinimaxClient(orchestration.NewMinimaxClient(minimaxKey), nil, nil)")

print("Done")
