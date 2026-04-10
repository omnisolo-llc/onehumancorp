#!/bin/bash
sed -i '/ALTER TABLE telemetry_buffer ADD COLUMN organization_id/d' srcs/server/db/migrations/013_tenant_isolation_sip.sql
cat << 'SQL_END' >> srcs/server/db/migrations/026_telemetry_buffer.sql
ALTER TABLE telemetry_buffer ADD COLUMN organization_id TEXT DEFAULT 'system';
SQL_END
