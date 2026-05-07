-- We must bypass RLS for token management as it doesn't have an organization ID and is global. Or we should ensure that the queries bypass RLS.
CREATE POLICY bypass_revoked_tokens ON revoked_tokens USING (true);
