-- Explicit publication receipts are also durable jobs. Draft edits and legacy
-- published_at timestamps do not grant anonymous access to this table's data.
ALTER TABLE builder_sites ADD COLUMN IF NOT EXISTS publication_generation BIGINT NOT NULL DEFAULT 0;
ALTER TABLE builder_sites ADD COLUMN IF NOT EXISTS current_publication_id UUID;
-- Existing UUID-only rows have no provable raw-tenant binding. Explicitly
-- reviewed legacy content must be saved as a newly bound publication site.
ALTER TABLE builder_sites ADD COLUMN IF NOT EXISTS publication_tenant_id TEXT;
CREATE UNIQUE INDEX IF NOT EXISTS idx_builder_sites_publication_tenant ON builder_sites(id, publication_tenant_id);
ALTER TABLE builder_sites ADD CONSTRAINT builder_publication_tenant_exists
FOREIGN KEY (publication_tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;

CREATE TABLE IF NOT EXISTS builder_publications (
    publication_id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    owner_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    operation_id UUID NOT NULL,
    site_id UUID NOT NULL,
    requested_site_id UUID,
    site_version BIGINT NOT NULL CHECK (site_version > 0),
    snapshot JSONB NOT NULL,
    snapshot_sha256 TEXT NOT NULL CHECK (snapshot_sha256 ~ '^[0-9a-f]{64}$'),
    product_ids TEXT[] NOT NULL DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','processing','published','failed','revoked')),
    lease_token UUID,
    lease_until TIMESTAMPTZ,
    rendered_pages JSONB,
    rendered_sha256 TEXT,
    error_code TEXT,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, owner_id, operation_id),
    UNIQUE (publication_id, tenant_id),
    UNIQUE (publication_id, tenant_id, site_id),
    UNIQUE (site_id, site_version),
    UNIQUE (site_id, publication_id),
    FOREIGN KEY (site_id,tenant_id) REFERENCES builder_sites(id,publication_tenant_id) ON DELETE CASCADE,
    CHECK (status <> 'processing' OR (lease_token IS NOT NULL AND lease_until IS NOT NULL)),
    CHECK (status <> 'published' OR (rendered_pages IS NOT NULL AND rendered_sha256 IS NOT NULL))
);
ALTER TABLE builder_sites ADD CONSTRAINT builder_current_publication_belongs_to_site
FOREIGN KEY (id,current_publication_id) REFERENCES builder_publications(site_id,publication_id)
ON DELETE SET NULL (current_publication_id);

CREATE INDEX IF NOT EXISTS idx_builder_publications_pending ON builder_publications(status, lease_until, created_at);
ALTER TABLE builder_publications ENABLE ROW LEVEL SECURITY;
ALTER TABLE builder_publications FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_builder_publications ON builder_publications;
CREATE POLICY tenant_isolation_builder_publications ON builder_publications
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE OR REPLACE FUNCTION protect_builder_publication_snapshot() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    IF OLD.status = 'published' AND NEW.status NOT IN ('published','revoked')
    THEN RAISE EXCEPTION 'Published receipt cannot be reopened'; END IF;
    IF OLD.status = 'revoked' AND ROW(NEW.status,NEW.revoked_at) IS DISTINCT FROM ROW(OLD.status,OLD.revoked_at)
    THEN RAISE EXCEPTION 'Publication revocation is final'; END IF;
    IF ROW(NEW.publication_id,NEW.tenant_id,NEW.owner_id,NEW.operation_id,NEW.site_id,NEW.requested_site_id,NEW.site_version,NEW.snapshot,NEW.snapshot_sha256,NEW.product_ids,NEW.created_at)
       IS DISTINCT FROM
       ROW(OLD.publication_id,OLD.tenant_id,OLD.owner_id,OLD.operation_id,OLD.site_id,OLD.requested_site_id,OLD.site_version,OLD.snapshot,OLD.snapshot_sha256,OLD.product_ids,OLD.created_at)
    THEN RAISE EXCEPTION 'Publication snapshot identity is immutable'; END IF;
    IF OLD.status IN ('published','revoked') AND ROW(NEW.rendered_pages,NEW.rendered_sha256) IS DISTINCT FROM ROW(OLD.rendered_pages,OLD.rendered_sha256)
    THEN RAISE EXCEPTION 'Published rendering is immutable'; END IF;
    RETURN NEW;
END;
$$;
DROP TRIGGER IF EXISTS protect_builder_publication_snapshot ON builder_publications;
CREATE TRIGGER protect_builder_publication_snapshot BEFORE UPDATE ON builder_publications
FOR EACH ROW EXECUTE FUNCTION protect_builder_publication_snapshot();

CREATE OR REPLACE FUNCTION protect_builder_publication_tenant() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.publication_tenant_id IS DISTINCT FROM OLD.publication_tenant_id
       OR (OLD.publication_tenant_id IS NOT NULL AND NEW.tenant_id IS DISTINCT FROM OLD.tenant_id)
    THEN RAISE EXCEPTION 'Publication site tenant binding is immutable'; END IF;
    RETURN NEW;
END;
$$;
DROP TRIGGER IF EXISTS protect_builder_publication_tenant ON builder_sites;
CREATE TRIGGER protect_builder_publication_tenant BEFORE UPDATE ON builder_sites
FOR EACH ROW EXECUTE FUNCTION protect_builder_publication_tenant();

-- Internal routing metadata contains no snapshot or user data. Discovery never
-- establishes authority; the worker rechecks the tenant-scoped receipt and user.
CREATE TABLE IF NOT EXISTS builder_publication_work (
    publication_id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    site_id UUID NOT NULL,
    queued BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (publication_id,tenant_id,site_id) REFERENCES builder_publications(publication_id,tenant_id,site_id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_builder_publication_work_site ON builder_publication_work(site_id);
CREATE INDEX IF NOT EXISTS idx_builder_publication_work_queued ON builder_publication_work(publication_id) WHERE queued;
CREATE OR REPLACE FUNCTION protect_builder_publication_routing() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    IF ROW(NEW.publication_id,NEW.tenant_id,NEW.site_id) IS DISTINCT FROM ROW(OLD.publication_id,OLD.tenant_id,OLD.site_id)
    THEN RAISE EXCEPTION 'Publication routing identity is immutable'; END IF;
    RETURN NEW;
END;
$$;
CREATE TRIGGER protect_builder_publication_routing BEFORE UPDATE ON builder_publication_work
FOR EACH ROW EXECUTE FUNCTION protect_builder_publication_routing();
REVOKE ALL ON builder_publication_work FROM PUBLIC;
ALTER TABLE builder_publication_work ENABLE ROW LEVEL SECURITY;
ALTER TABLE builder_publication_work FORCE ROW LEVEL SECURITY;
CREATE POLICY publication_work_discovery ON builder_publication_work FOR SELECT USING (true);
CREATE POLICY publication_work_insert ON builder_publication_work FOR INSERT WITH CHECK (tenant_id=current_setting('app.current_tenant',true));
CREATE POLICY publication_work_delete ON builder_publication_work FOR DELETE USING (tenant_id=current_setting('app.current_tenant',true));

CREATE POLICY publication_work_retire ON builder_publication_work FOR UPDATE USING (tenant_id=current_setting('app.current_tenant',true)) WITH CHECK (tenant_id=current_setting('app.current_tenant',true));
