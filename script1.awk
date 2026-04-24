/r\.mu\.TryLock\(\)/ {
    print "\t\tif !r.mu.TryLock() {"
    print "\t\t\ttelemetry.RecordPostgresLockContention(ctx, \"shared_task_list_tasks\")"
    print "\t\t\tr.mu.Lock()"
    print "\t\t}"
    next
}
/r\.mu\.Lock\(\)/ { next }
/telemetry\.RecordPostgresLockContention/ { next }
{ print }
