CREATE TABLE IF NOT EXISTS growth_referrals (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    inviter_id TEXT NOT NULL,
    invitee_email TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
