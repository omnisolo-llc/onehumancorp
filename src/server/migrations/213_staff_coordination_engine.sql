-- Migration: 213_staff_coordination_engine.sql
-- Description: Agentic Staff Coordination Engine

-- Shift Table
CREATE TABLE IF NOT EXISTS public.shifts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    location_id UUID,
    staff_id UUID NOT NULL,
    clock_in_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    clock_out_time TIMESTAMPTZ,
    status TEXT NOT NULL CHECK (status IN ('active', 'completed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS for shifts
ALTER TABLE public.shifts ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_shifts ON public.shifts
    AS PERMISSIVE FOR ALL
    TO public
    USING (tenant_id = (current_setting('app.current_tenant_id'::text))::uuid)
    WITH CHECK (tenant_id = (current_setting('app.current_tenant_id'::text))::uuid);

-- Tasks Table
CREATE TABLE IF NOT EXISTS public.tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    location_id UUID,
    shift_id UUID,
    assigned_to UUID,
    title TEXT NOT NULL,
    description TEXT,
    priority TEXT NOT NULL DEFAULT 'medium' CHECK (priority IN ('low', 'medium', 'high', 'urgent')),
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'in_progress', 'completed', 'escalated')),
    due_date TIMESTAMPTZ,
    source TEXT NOT NULL DEFAULT 'manual' CHECK (source IN ('manual', 'agent_operations', 'agent_ambassador', 'system')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS for tasks
ALTER TABLE public.tasks ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_tasks ON public.tasks
    AS PERMISSIVE FOR ALL
    TO public
    USING (tenant_id = (current_setting('app.current_tenant_id'::text))::uuid)
    WITH CHECK (tenant_id = (current_setting('app.current_tenant_id'::text))::uuid);

-- Escalations Table
CREATE TABLE IF NOT EXISTS public.escalations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    location_id UUID,
    related_task_id UUID,
    summary TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'acknowledged', 'resolved')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS for escalations
ALTER TABLE public.escalations ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_escalations ON public.escalations
    AS PERMISSIVE FOR ALL
    TO public
    USING (tenant_id = (current_setting('app.current_tenant_id'::text))::uuid)
    WITH CHECK (tenant_id = (current_setting('app.current_tenant_id'::text))::uuid);

-- Trigger to update updated_at timestamps
CREATE TRIGGER set_updated_at_shifts
BEFORE UPDATE ON public.shifts
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_updated_at_tasks
BEFORE UPDATE ON public.tasks
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_updated_at_escalations
BEFORE UPDATE ON public.escalations
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
