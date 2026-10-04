INSERT INTO agent_feed_decisions(tenant_id,action_id,origin,decision_state,actor_id,token_id,job_id,dispatch_payload,dispatch_status,attempted_at,dispatch_returned_at)
VALUES('synthetic','attempt','canonical','APPROVED','owner','token','job','{}','DISPATCH_RETURNED',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);
DO $$
DECLARE rejected BOOLEAN := FALSE;
BEGIN
 BEGIN
  UPDATE agent_feed_decisions SET attempted_at=NULL,dispatch_status='PENDING' WHERE action_id='attempt';
 EXCEPTION WHEN check_violation THEN rejected := TRUE;
 END;
 IF NOT rejected THEN RAISE EXCEPTION 'Expected irreversible dispatch fence rejection'; END IF;
END $$;
DO $$
DECLARE rejected BOOLEAN := FALSE;
BEGIN
 BEGIN
  DELETE FROM agent_feed_decisions WHERE action_id='attempt';
 EXCEPTION WHEN check_violation THEN rejected := TRUE;
 END;
 IF NOT rejected THEN RAISE EXCEPTION 'Expected durable attempt deletion rejection'; END IF;
END $$;
