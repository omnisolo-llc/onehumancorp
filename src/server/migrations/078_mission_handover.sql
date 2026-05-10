-- Mission Handover protocol executed by General Mission Worker & Taskmaster (L5)
UPDATE agent_missions
SET status = 'blocked',
    mission_log = COALESCE(mission_log, '') || CASE WHEN COALESCE(mission_log, '') = '' THEN '' ELSE '
' END || 'Handover: Blocker - Unable to finish OHC product missions within current execution context constraints.'
WHERE status IN ('PENDING', 'pending');
