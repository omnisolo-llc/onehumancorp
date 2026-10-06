-- Metadata/attempt links on the receipt database; no second execution queue.
CREATE UNIQUE INDEX IF NOT EXISTS assistant_receipt_identity ON tenant_workflow_receipts(id,tenant_id,actor_id);
CREATE TABLE IF NOT EXISTS assistant_execution_tasks (
 id TEXT PRIMARY KEY NOT NULL,
 tenant_id TEXT NOT NULL,
 actor_id TEXT NOT NULL,
 root_request_id TEXT NOT NULL,
 current_receipt_id TEXT NOT NULL,
 workspace TEXT NOT NULL CHECK(length(btrim(workspace)) BETWEEN 1 AND 80),
 title TEXT NOT NULL CHECK(length(btrim(title)) BETWEEN 1 AND 160),
 archived INTEGER NOT NULL DEFAULT 0 CHECK(archived IN (0,1)),
 created_at BIGINT NOT NULL CHECK(created_at>0),
 updated_at BIGINT NOT NULL CHECK(updated_at>=created_at),
 UNIQUE(id,tenant_id,actor_id),
 UNIQUE(tenant_id,actor_id,root_request_id),
 FOREIGN KEY(id,tenant_id,actor_id) REFERENCES tenant_workflow_receipts(id,tenant_id,actor_id) ON DELETE CASCADE,
 FOREIGN KEY(current_receipt_id,tenant_id,actor_id) REFERENCES tenant_workflow_receipts(id,tenant_id,actor_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS assistant_execution_attempts (
 receipt_id TEXT PRIMARY KEY NOT NULL,
 task_id TEXT NOT NULL,
 tenant_id TEXT NOT NULL,
 actor_id TEXT NOT NULL,
 source_receipt_id TEXT,
 UNIQUE(task_id,source_receipt_id),
 FOREIGN KEY(task_id,tenant_id,actor_id) REFERENCES assistant_execution_tasks(id,tenant_id,actor_id) ON DELETE CASCADE,
 FOREIGN KEY(receipt_id,tenant_id,actor_id) REFERENCES tenant_workflow_receipts(id,tenant_id,actor_id) ON DELETE CASCADE,
 FOREIGN KEY(source_receipt_id,tenant_id,actor_id) REFERENCES tenant_workflow_receipts(id,tenant_id,actor_id) ON DELETE CASCADE
);

ALTER TABLE assistant_execution_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE assistant_execution_tasks FORCE ROW LEVEL SECURITY;
CREATE POLICY assistant_task_scope ON assistant_execution_tasks
 USING(tenant_id=current_setting('app.current_tenant',true))
 WITH CHECK(tenant_id=current_setting('app.current_tenant',true));
ALTER TABLE assistant_execution_attempts ENABLE ROW LEVEL SECURITY;
ALTER TABLE assistant_execution_attempts FORCE ROW LEVEL SECURITY;
CREATE POLICY assistant_attempt_scope ON assistant_execution_attempts
 USING(tenant_id=current_setting('app.current_tenant',true))
 WITH CHECK(tenant_id=current_setting('app.current_tenant',true));
CREATE FUNCTION assistant_task_admission() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF NEW.id<>NEW.current_receipt_id OR NOT EXISTS (
  SELECT 1 FROM tenant_workflow_receipts r WHERE r.id=NEW.id AND r.tenant_id=NEW.tenant_id
  AND r.actor_id=NEW.actor_id AND r.request_id=NEW.root_request_id AND r.phase IN ('queued','cancelled')) THEN
  RAISE EXCEPTION 'assistant_requires_admitted_receipt';
 END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER assistant_task_admission BEFORE INSERT ON assistant_execution_tasks
 FOR EACH ROW EXECUTE FUNCTION assistant_task_admission();
CREATE FUNCTION assistant_task_update() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF ROW(NEW.id,NEW.tenant_id,NEW.actor_id,NEW.root_request_id,NEW.created_at)
  IS DISTINCT FROM ROW(OLD.id,OLD.tenant_id,OLD.actor_id,OLD.root_request_id,OLD.created_at) THEN
  RAISE EXCEPTION 'assistant_task_identity_is_immutable';
 END IF;
 IF NEW.current_receipt_id<>OLD.current_receipt_id AND NOT EXISTS (
  SELECT 1 FROM assistant_execution_attempts a JOIN tenant_workflow_receipts source ON source.id=a.source_receipt_id
  JOIN tenant_workflow_receipts target ON target.id=a.receipt_id
  WHERE a.task_id=OLD.id AND a.receipt_id=NEW.current_receipt_id AND a.source_receipt_id=OLD.current_receipt_id
  AND a.tenant_id=OLD.tenant_id AND a.actor_id=OLD.actor_id AND source.phase='cancelled' AND target.phase IN ('queued','cancelled')) THEN
  RAISE EXCEPTION 'assistant_attempt_pointer_requires_safe_association';
 END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER assistant_task_update BEFORE UPDATE ON assistant_execution_tasks
 FOR EACH ROW EXECUTE FUNCTION assistant_task_update();
CREATE FUNCTION assistant_attempt_admission() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF NOT EXISTS (
  SELECT 1 FROM assistant_execution_tasks t JOIN tenant_workflow_receipts r ON r.id=NEW.receipt_id
  WHERE t.id=NEW.task_id AND t.tenant_id=NEW.tenant_id AND t.actor_id=NEW.actor_id
  AND r.tenant_id=NEW.tenant_id AND r.actor_id=NEW.actor_id
  AND ((NEW.source_receipt_id IS NULL AND NEW.receipt_id=t.id AND r.phase IN ('queued','cancelled'))
  OR (NEW.source_receipt_id=t.current_receipt_id AND NEW.receipt_id<>t.id AND r.phase IN ('queued','cancelled')
  AND EXISTS (SELECT 1 FROM tenant_workflow_receipts source WHERE source.id=NEW.source_receipt_id
  AND source.tenant_id=NEW.tenant_id AND source.actor_id=NEW.actor_id AND source.phase='cancelled')))) THEN
  RAISE EXCEPTION 'assistant_attempt_requires_cancelled_source';
 END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER assistant_attempt_admission BEFORE INSERT ON assistant_execution_attempts
 FOR EACH ROW EXECUTE FUNCTION assistant_attempt_admission();
CREATE FUNCTION assistant_execution_retention() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF TG_OP='UPDATE' OR EXISTS(SELECT 1 FROM users WHERE id=OLD.actor_id AND tenant_id=OLD.tenant_id) THEN
  RAISE EXCEPTION 'assistant_execution_history_is_immutable';
 END IF;
 RETURN OLD;
END $$;
CREATE TRIGGER assistant_attempt_immutable BEFORE UPDATE OR DELETE ON assistant_execution_attempts
 FOR EACH ROW EXECUTE FUNCTION assistant_execution_retention();
CREATE TRIGGER assistant_task_retention BEFORE DELETE ON assistant_execution_tasks
 FOR EACH ROW EXECUTE FUNCTION assistant_execution_retention();
DO $$ DECLARE function_name TEXT; BEGIN
 FOREACH function_name IN ARRAY ARRAY['assistant_task_admission','assistant_task_update','assistant_attempt_admission','assistant_execution_retention'] LOOP
  EXECUTE format('ALTER FUNCTION %I.%I() SET search_path TO pg_catalog,%I,pg_temp',current_schema(),function_name,current_schema());
 END LOOP;
END $$;
