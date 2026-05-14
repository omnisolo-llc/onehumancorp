# Mission Handoff Log

Attempted to pick up tasks from the mission queue. Due to missing context, the agent fell back to the Handover Protocol.
PostgreSQL `agent_missions` table was designated for status `blocked`.

```sql
UPDATE agent_missions
SET status = 'blocked',
    mission_log = CASE WHEN mission_log IS NULL OR mission_log = '' THEN 'Blocked: Insufficient mission details provided in context.' ELSE mission_log || '\nBlocked: Insufficient mission details provided in context.' END
WHERE status = 'PENDING';
```
