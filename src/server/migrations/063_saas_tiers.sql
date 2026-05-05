-- 063_saas_tiers.sql
-- Explicitly formalize tier ENUM constraints for SaaS architecture

-- Ensure the plan_tier on organizations is standardized
UPDATE organizations SET plan_tier = 'Free' WHERE plan_tier IS NULL OR plan_tier = '';

ALTER TABLE organizations ADD CONSTRAINT chk_organizations_plan_tier
    CHECK (plan_tier IN ('Free', 'Starter', 'Pro', 'Business'));

-- If the tenants table has a tier column, standardize that as well
UPDATE tenants SET tier = 'Free' WHERE tier IS NULL OR tier = '';

ALTER TABLE tenants ADD CONSTRAINT chk_tenants_tier
    CHECK (tier IN ('Free', 'Starter', 'Pro', 'Business'));
