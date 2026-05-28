ALTER TABLE tasks ADD COLUMN auto_dreamed BOOLEAN DEFAULT FALSE;

-- Ensure agent_memories conforms to design doc (id, organization_id, task_id, raw_content, summary_embedding)
-- The table was created with tenant_id, content, embedding. We'll rename them.
ALTER TABLE agent_memories RENAME COLUMN tenant_id TO organization_id;
ALTER TABLE agent_memories RENAME COLUMN content TO raw_content;
ALTER TABLE agent_memories RENAME COLUMN embedding TO summary_embedding;

-- task_id should be UUID since tasks.id is a UUID (from review: tasks.id is a UUID)
ALTER TABLE agent_memories ADD COLUMN task_id UUID;

-- Update the row level security policy for the renamed column
DROP POLICY IF EXISTS tenant_isolation_agent_memories ON agent_memories;
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (organization_id::text = current_setting('app.current_tenant', true));
