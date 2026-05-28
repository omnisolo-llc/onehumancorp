ALTER TABLE tasks ADD COLUMN auto_dreamed BOOLEAN DEFAULT FALSE;

ALTER TABLE agent_memories RENAME COLUMN tenant_id TO organization_id;
ALTER TABLE agent_memories RENAME COLUMN content TO raw_content;
ALTER TABLE agent_memories RENAME COLUMN embedding TO summary_embedding;
ALTER TABLE agent_memories ADD COLUMN task_id TEXT;

DROP POLICY IF EXISTS tenant_isolation_agent_memories ON agent_memories;
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (organization_id::text = current_setting('app.current_tenant', true));
