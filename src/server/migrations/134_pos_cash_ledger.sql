-- POS Cash Ledger and Shift Reconciliation
CREATE TABLE IF NOT EXISTS pos_cash_ledger_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    entry_type TEXT NOT NULL, -- 'SALE', 'CASH_IN', 'CASH_OUT', 'DROP', 'PAYOUT'
    amount_cents BIGINT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'usd',
    reason TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS
ALTER TABLE pos_cash_ledger_entries ENABLE ROW LEVEL SECURITY;

-- Add shift reconciliation fields to terminal sessions
ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS opening_balance_cents BIGINT DEFAULT 0;
ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS closing_balance_cents BIGINT;
ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS closed_at TIMESTAMP WITH TIME ZONE;

-- Operations Agent view of cash ledger
CREATE INDEX IF NOT EXISTS idx_pos_ledger_session ON pos_cash_ledger_entries(session_id, tenant_id);
