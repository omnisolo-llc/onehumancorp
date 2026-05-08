-- 074_sub_agent_jobs_org_id.sql
-- Add organization_id to sub_agent_jobs to support multi-tenancy.

ALTER TABLE sub_agent_jobs ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
