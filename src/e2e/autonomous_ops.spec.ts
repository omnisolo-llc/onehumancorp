import { test, expect } from '@playwright/test';
import { execSync } from 'child_process';

test.describe('Autonomous Operations CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // 1. Start from the home page after user login
    await page.goto('/');
    // Assuming the app has a mock login or auto-login for E2E
    await expect(page).toHaveTitle(/OneHuman/);
  });

  test('should process business events and show drafting helper in dashboard', async ({ page }) => {
    // 2. Simulate a Business Event (Customer Message)
    // We use a SQL command to insert a task into department_tasks table
    // This assumes the environment has a database named 'ohc' as per playwright_test.sh
    const sql = `
      INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
      VALUES ('e2e-task-1', 'default_org', 'customer_success', 'CustomerMessageReceived', '{"message": "E2E Test Message"}', 'PENDING');
    `;
    try {
        execSync(`docker exec e2e_postgres psql -U ohc -d ohc -c "${sql}"`);
    } catch (e) {
        console.warn("Failed to insert task via docker, falling back to local psql if available");
        // Fallback for non-docker environments if any
        try {
            execSync(`psql -U ohc -d ohc -c "${sql}"`);
        } catch (e2) {
             console.error("Could not insert test task into DB");
        }
    }

    // Give the background worker a moment to process the task
    await page.waitForTimeout(6000);

    // 3. Reload dashboard to see the drafted task
    await page.reload();

    // 4. Verify the drafted task appears with "The Ambassador" label
    const approvalCard = page.locator('text=Draft Reply');
    await expect(approvalCard).toBeVisible();
    await expect(page.locator('text=The Ambassador')).toBeVisible();
    await expect(page.locator('text=E2E Test Message')).toBeVisible();

    // 5. Approve the task with 1-tap
    const approveBtn = page.locator('button:has-text("Approve & Send")').first();
    await approveBtn.click();

    // 6. Verify the task is removed from UI
    await expect(approvalCard).not.toBeVisible();
  });

  test('should process OrderPlaced events and flag low stock', async ({ page }) => {
     // 1. Prepare product with low stock
     const productSql = `
       INSERT INTO products (id, organization_id, name, inventory_count, fulfillment_strategy)
       VALUES ('e2e-prod-low', 'default_org', 'Low Stock Product', 2, 'physical')
       ON CONFLICT (id) DO UPDATE SET inventory_count = 2;
     `;
     const eventSql = `
       INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
       VALUES ('e2e-task-ops-1', 'default_org', 'operations', 'OrderPlaced', '{"items": [{"product_id": "e2e-prod-low", "quantity": 1}]}', 'PENDING');
     `;

     try {
         execSync(`docker exec e2e_postgres psql -U ohc -d ohc -c "${productSql}"`);
         execSync(`docker exec e2e_postgres psql -U ohc -d ohc -c "${eventSql}"`);
     } catch (e) {
          // Fallback
     }

     await page.waitForTimeout(6000);
     await page.reload();

     // Verify "The Manager" flagged it
     await expect(page.locator('text=Restock Item: e2e-prod-low')).toBeVisible();
     await expect(page.locator('text=The Manager')).toBeVisible();

     // Approve
     await page.locator('button:has-text("Approve & Send")').first().click();
     await expect(page.locator('text=Restock Item: e2e-prod-low')).not.toBeVisible();
  });

  test('should handle multiple pending approvals from different departments', async ({ page }) => {
    const sqlOps = `
      INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status, helper_name)
      VALUES ('e2e-mult-ops', 'default_org', 'Restock Milk', 'Running low', 'PENDING', 'P1', 'LOW', 'PENDING', 'The Manager')
      ON CONFLICT DO NOTHING;
    `;
    const sqlCS = `
      INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status, proposed_content, helper_name)
      VALUES ('e2e-mult-cs', 'default_org', 'Draft Reply', 'Customer asked about cakes', 'PENDING', 'P1', 'HIGH', 'PENDING', 'Yes we do!', 'The Ambassador')
      ON CONFLICT DO NOTHING;
    `;
    // Note: the helper_name column might not exist yet, let's use the title mapping logic for now to be safe
    const sqlOpsSafe = `
      INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status)
      VALUES ('e2e-mult-ops', 'default_org', 'Restock Milk', 'Running low', 'PENDING', 'P1', 'LOW', 'PENDING')
      ON CONFLICT DO NOTHING;
    `;
    const sqlCSSafe = `
      INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status, proposed_content)
      VALUES ('e2e-mult-cs', 'default_org', 'Draft Reply to Fatima', 'Customer asked about pick-up', 'PENDING', 'P1', 'HIGH', 'PENDING', 'Sure, Fatima!')
      ON CONFLICT DO NOTHING;
    `;

    try {
        execSync(`docker exec e2e_postgres psql -U ohc -d ohc -c "${sqlOpsSafe}"`);
        execSync(`docker exec e2e_postgres psql -U ohc -d ohc -c "${sqlCSSafe}"`);
    } catch (e) {}

    await page.reload();

    await expect(page.locator('text=The Manager')).toBeVisible();
    await expect(page.locator('text=The Ambassador')).toBeVisible();
    await expect(page.locator('text=Restock Milk')).toBeVisible();
    await expect(page.locator('text=Draft Reply to Fatima')).toBeVisible();

    // Approve CS task
    await page.locator('div:has-text("Draft Reply to Fatima")').locator('button:has-text("Approve & Send")').click();
    await expect(page.locator('text=Draft Reply to Fatima')).not.toBeVisible();
    await expect(page.locator('text=Restock Milk')).toBeVisible();
  });

  test('should show empty state message when no approvals are pending', async ({ page }) => {
    // Clear all pending approvals for this org
    const sql = "DELETE FROM shared_tasks WHERE organization_id = 'default_org' AND approval_status = 'PENDING';";
    try {
        execSync(`docker exec e2e_postgres psql -U ohc -d ohc -c "${sql}"`);
    } catch (e) {}

    await page.reload();
    await expect(page.locator('text=Needs Your Approval')).not.toBeVisible();
  });

  test('should verify plain language activity feed in dashboard header', async ({ page }) => {
    // This test ensures the dashboard header reflects proactive helper status
    await expect(page.locator('text=My Business')).toBeVisible();
    await expect(page.locator('text=System Status')).toBeVisible();
    // Helper counts from memory
    await expect(page.locator('text=Team Members')).toBeVisible();
  });
});
