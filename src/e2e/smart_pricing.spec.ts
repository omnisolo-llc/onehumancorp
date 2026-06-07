import { test, expect } from '@playwright/test';
import { memberPage, testTenantId } from './fixtures';
import { v4 as uuidv4 } from 'uuid';
import { Pool } from 'pg';

test.describe('Smart Pricing Feature CUJ', () => {
    let pool: Pool;

    test.beforeAll(() => {
        pool = new Pool({
            connectionString: process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc',
        });
    });

    test.afterAll(async () => {
        await pool.end();
    });

    test('should apply smart pricing', async ({ memberPage: page }) => {
        await page.goto('/dashboard');
        await expect(page.getByText('Proposals (0)')).toBeVisible();

        const productId = uuidv4();
        const policyId = uuidv4();

        await pool.query(
            "INSERT INTO products (id, tenant_id, title, description, type, price, inventory_count) VALUES ($1, $2, 'Winter Coat', 'Warm coat', 'physical', 100, 10)",
            [productId, testTenantId]
        );

        await pool.query(
            "INSERT INTO smart_pricing_policies (id, tenant_id, product_id, min_margin_percent, auto_discount_trigger_days_stagnant, max_discount_percent) VALUES ($1, $2, $3, 0.2, 30, 0.5)",
            [policyId, testTenantId, productId]
        );

        const approvalId = uuidv4();
        await pool.query(
            "INSERT INTO agent_approvals (id, tenant_id, department, action_type, description, payload, status, created_at, updated_at) VALUES ($1, $2, 'BUSINESS_ADVISORY', 'SMART_PRICING_SUGGESTION', 'Smart Price Suggestion: Would you like to apply a 25% discount to clear inventory?', $3, 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
            [approvalId, testTenantId, JSON.stringify({"type": "smart_pricing_suggestion", "policy_id": policyId, "product_id": productId, "suggested_discount": 0.25})]
        );

        await page.reload();
        await expect(page.getByText('Proposals (1)')).toBeVisible();
        await expect(page.getByText('Smart Price Suggestion')).toBeVisible();

        await page.getByRole('button', { name: 'Approve proposal' }).click();

        await expect(page.getByText('Proposals (0)')).toBeVisible();

        const activeDiscounts = await pool.query(
            "SELECT * FROM active_discounts WHERE policy_id = $1",
            [policyId]
        );
        expect(activeDiscounts.rows.length).toBe(1);
        expect(activeDiscounts.rows[0].discount_amount).toBe(0.25);
    });
});
