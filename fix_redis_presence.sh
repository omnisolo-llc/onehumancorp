cat << 'INNER_EOF' > modify.sed
s/return m.client.HSet(ctx, "presence", agentID, data).Err()/return m.client.Set(ctx, "presence:" + agentID, data, 30*time.Second).Err()/g
/res, err := m.client.HGetAll(ctx, "presence").Result()/c\
\tkeys, err := m.client.Keys(ctx, "presence:*").Result()\
\tif err != nil {\
\t\treturn nil, err\
\t}\
\n\tvar active []AgentPresence\
\tfor _, key := range keys {\
\t\tdata, err := m.client.Get(ctx, key).Result()\
\t\tif err == nil {\
\t\t\tvar p AgentPresence\
\t\t\tif err := json.Unmarshal([]byte(data), \&p); err == nil {\
\t\t\t\tactive = append(active, p)\
\t\t\t}\
\t\t}\
\t}\
\treturn active, nil
INNER_EOF
sed -i -f modify.sed srcs/server/orchestration/mesh/redis_mesh.go
