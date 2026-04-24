/to\.mu\.TryLock\(\)/ {
    print "\t\tif !to.mu.TryLock() {"
    print "\t\t\ttelemetry.RecordPostgresLockContention(ctx, \"tasks_db\")"
    print "\t\t\tto.mu.Lock()"
    print "\t\t}"
    next
}
/to\.mu\.Lock\(\)/ { next }
/telemetry\.RecordPostgresLockContention/ { next }
{ print }
