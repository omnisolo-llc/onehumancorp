import { test, expect } from '@playwright/test';
import { db } from '../db_utils';

test.describe('AI Agentic Inventory & Prep Forecasting Engine', () => {
  const tenantId = 'fatima-tenant';
  let predictionId: string;
  let productId: string;

  test.beforeAll(async () => {
    // We mock the database connection if it's not available in the test environment.
    // In CI, DATABASE_URL will be set. Locally, it might not be.
    // E2E resilience constraint requires test to run against the real stack if available.

    // Setup test data
    await db.query("BEGIN");
    try {

      // Setup tenant
      await db.query(`
        INSERT INTO tenants (id, name, subdomain)
        VALUES ($1, 'Fatimas Food Cart', 'fatima')
        ON CONFLICT (id) DO NOTHING
      `, [tenantId]);

      // Set context
      await db.query("SELECT set_config('app.current_tenant', $1, false)", [tenantId]);

      // Create product
      productId = 'prod-falafel-' + Date.now();
      await db.query(`
        INSERT INTO products (id, tenant_id, name, description, price, currency)
        VALUES ($1, $2, 'Falafel Wrap', 'Delicious wrap', 8.99, 'USD')
      `, [productId, tenantId]);

      // Create inventory prediction
      predictionId = 'pred-' + Date.now();
      const tomorrow = new Date();
      tomorrow.setDate(tomorrow.getDate() + 1);

      await db.query(`
        INSERT INTO inventory_predictions (id, tenant_id, product_id, predicted_stockout_date, confidence_score, suggested_reorder_quantity)
        VALUES ($1, $2, $3, $4, 0.92, 50)
      `, [predictionId, tenantId, productId, tomorrow.toISOString()]);

      await db.query("COMMIT");
    } catch (e) {
      await db.query("ROLLBACK");
      throw e;
    }
  });

  test.afterAll(async () => {
    await db.query(`DELETE FROM tenants WHERE id = $1`, [tenantId]);
  });

  test('Food cart operator can review and approve AI prep forecast', async ({ page }) => {
    // Set tenant
    await page.addInitScript((tenant) => {
      localStorage.setItem('tenant_id', tenant);
    }, tenantId);

    // Navigate to prep forecast page
    await page.goto('/prep-forecast');

    // Wait for the prediction to load
    const card = page.locator(`[data-testid^="prep-card-"]`).first();
    await expect(card).toBeVisible();

    // Check content
    await expect(card.locator('text=Falafel Wrap')).toBeVisible();
    await expect(card.locator('text=92%')).toBeVisible();
    await expect(card.locator('text=50')).toBeVisible();

    // Adjust quantity
    await card.locator('button:has-text("+")').click();
    await expect(card.locator('text=51')).toBeVisible();

    await card.locator('button:has-text("-")').click();
    await expect(card.locator('text=50')).toBeVisible();

    // Approve
    await card.locator('button:has-text("Approve")').click();

    // Verify it disappears from the list
    await expect(card).not.toBeVisible();

    // Verify empty state if no more items
    const emptyState = page.locator('text=No prep items required for today.');
    await expect(emptyState).toBeVisible();

    // Verify job was created in DB
    const res = await db.query(`
      SELECT * FROM ohc_job_queue
      WHERE tenant_id = $1 AND job_type = 'create_prep_task'
      ORDER BY created_at DESC LIMIT 1
    `, [tenantId]);

    expect(res.rows.length).toBeGreaterThan(0);
    expect(res.rows[0].payload.product_id).toBe(productId);
    expect(res.rows[0].payload.action).toBe('approve_prep_plan');
  });
});
