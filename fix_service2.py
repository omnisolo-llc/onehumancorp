import sys

def replace_in_file(filepath, search, replace):
    with open(filepath, 'r') as f:
        content = f.read()
    content = content.replace(search, replace)
    with open(filepath, 'w') as f:
        f.write(content)

replace_in_file("srcs/server/orchestration/service.go", "client := NewCachedMinimaxClient(NewMinimaxClient(h.MinimaxAPIKey()), h.db, h.redisClient)", "client := NewCachedMinimaxClient(NewMinimaxClient(h.MinimaxAPIKey()), h.SIPDB().Provider(), nil)")
replace_in_file("srcs/server/orchestration/service.go", "client := NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), tm.db, tm.redisClient)", "client := NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), h.SIPDB().Provider(), nil)")
replace_in_file("srcs/server/orchestration/service.go", "client := NewCachedMinimaxClient(NewMinimaxClient(s.hub.MinimaxAPIKey()), s.hub.SIPDB().Provider(), s.hub.RedisClient())", "client := NewCachedMinimaxClient(NewMinimaxClient(s.hub.MinimaxAPIKey()), s.hub.SIPDB().Provider(), nil)")

print("Done")
