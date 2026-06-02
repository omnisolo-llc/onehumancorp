-- Update existing PENDING_APPROVAL statuses to DRAFT
UPDATE agent_approvals SET status = 'DRAFT' WHERE status = 'PENDING_APPROVAL';
