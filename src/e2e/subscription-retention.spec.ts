import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import { AIJudge } from './ai-judge';
import { Pool } from 'pg';

test.describe('Agentic Subscription Retention & Churn Prediction System', () => {
  test('Worker identifies at-risk subscriber and drafts win-back message', async ({ adminPage: page }) => {
    // 1. Manually trigger the worker logic by inserting seed data into Postgres directly.
    const pool = new Pool({
      connectionString: process.env.DATABASE_URL,
    });

    // Using default E2E tenant
    const tenantIdRes = await pool.query("SELECT id FROM tenants WHERE slug = 'e2e-tenant' LIMIT 1");
    const tenantId = tenantIdRes.rows.length > 0 ? tenantIdRes.rows[0].id : null;

    if (tenantId) {
        const customerId = '00000000-0000-0000-0000-000000000002'; // Dummy customer

        // Directly insert the draft job as if the `subscription_retention_job` ran
        await pool.query(`
          INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status)
          VALUES (gen_random_uuid(), $1, 'subscription_retention', $2, 'PENDING')
        `, [
          tenantId,
          JSON.stringify({ subscription_id: 'sub_test123', customer_id: customerId })
        ]);

        // Wait a moment for worker to process
        await new Promise(r => setTimeout(r, 6000));
    }
    await pool.end();

    // 2. Navigate to feed
    await page.goto('/feed');
    await expect(page).toHaveURL(/\/feed/);

    // 3. We expect the worker to have processed the at-risk customer and generated a card
    const approveButton = page.locator('button:has-text("Approve & Send")').first();
    await approveButton.waitFor({ state: 'visible', timeout: 5000 });

    const textContent = await page.locator('.text-sm.text-\\[\\#1D1D1F\\]\\/80').first().textContent();
    expect(textContent).toContain('discount');

    const reasoningText = await page.locator('text=Reasoning:').first().textContent();
    expect(reasoningText).toContain('Health Score dropped');

    await approveButton.click();
    await expect(approveButton).toHaveCount(0);
  });
});
