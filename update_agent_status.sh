TIMESTAMP=$(date +%s)
cat << 'YML' > .agent-task/status/${TIMESTAMP}.yml
agent: Link
role: Principal Interoperability Engineer
status: IDLE
health: HEALTHY
metrics:
  mesh_health_checks_implemented: 1
YML
