/sharedTasksMu\.TryLock\(\)/ {
    print "\t\tif !sharedTasksMu.TryLock() {"
    print "\t\t\ttelemetry.RecordPostgresLockContention(ctx, \"shared_tasks_decomposition\")"
    print "\t\t\tsharedTasksMu.Lock()"
    print "\t\t}"
    next
}
/sharedTasksMu\.Lock\(\)/ { next }
/telemetry\.RecordPostgresLockContention/ { next }
{ print }
