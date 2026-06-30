import { test, expect } from '@playwright/test';

test.describe('Intelligent Owner Triage Inbox (Mobile First)', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // Mobile viewport

  test('should load the triage inbox and display empty state', async ({ page }) => {
    const tenantId = 'triage-test-tenant-' + Date.now();
    await page.goto(`/api/ui/triage.html?tenant_id=${tenantId}`);

    // Verify app shell and empty state
    await expect(page.locator('text=Work Triage')).toBeVisible();
    await expect(page.getByTestId('triage-feed-empty')).toBeVisible();
    await expect(page.locator('text=All caught up! You\'re a hero.')).toBeVisible();
  });

  test('should handle a new triage item (approve)', async ({ page, request }) => {
    const tenantId = 'triage-test-tenant-approve-' + Date.now();

    // Seed database with an item by hitting an API endpoint (e.g. webhook) or we can just mock it in DB directly or use dev endpoint if exists.
    // Wait, the API `load_ui_triage_from_db` looks in `triage_items` / `unified_triage_actions`.
    // We can use the simulate endpoint or similar if it populates the correct table.

    // Let's use the builder seeder exec to insert directly:
    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status)
          VALUES ('triage-item-1', '${tenantId}', 'Cust 1', 'Proactive Context Agent', 'urgent', 'Needs attention right away', 'pending')
          ON CONFLICT DO NOTHING;
          INSERT INTO triage_proposed_actions (id, tenant_id, triage_item_id, action_type, payload, status)
          VALUES ('triage-action-1', '${tenantId}', 'triage-item-1', 'Draft Reply', 'Here is a draft', 'pending')
          ON CONFLICT DO NOTHING;
        `
      }
    });

    await page.goto(`/api/ui/triage.html?tenant_id=${tenantId}`);

    // Wait for item to appear
    const itemCard = page.getByTestId('triage-card-triage-item-1');
    await expect(itemCard).toBeVisible({ timeout: 15000 });

    // Click the card header to expand detail view
    const cardHeader = page.getByTestId('triage-card-header-triage-item-1');
    await expect(cardHeader).toBeVisible();
    await cardHeader.click();

    // Verify glassmorphism style or contents
    await expect(itemCard.locator('text=Needs attention right away')).toBeVisible();
    await expect(itemCard.locator('text=Draft Reply')).toBeVisible();

    // Test the button target size
    const approveButton = page.getByTestId('triage-approve-triage-item-1');
    await expect(approveButton).toBeVisible();
    const box = await approveButton.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.width).toBeGreaterThanOrEqual(44);
      expect(box.height).toBeGreaterThanOrEqual(44);
    }

    // Click Approve
    await approveButton.click();

    // Optimistic update should hide it
    await expect(itemCard).not.toBeVisible({ timeout: 5000 });
  });

  test('should handle a new triage item (edit and approve)', async ({ page, request }) => {
    const tenantId = 'triage-test-tenant-edit-' + Date.now();

    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status)
          VALUES ('triage-item-3', '${tenantId}', 'Cust 3', 'Proactive Context Agent', 'urgent', 'Needs edit right away', 'pending')
          ON CONFLICT DO NOTHING;
          INSERT INTO triage_proposed_actions (id, tenant_id, triage_item_id, action_type, payload, status)
          VALUES ('triage-action-3', '${tenantId}', 'triage-item-3', 'Draft Reply', 'Original draft payload', 'pending')
          ON CONFLICT DO NOTHING;
        `
      }
    });

    await page.goto(`/api/ui/triage.html?tenant_id=${tenantId}`);

    const itemCard = page.getByTestId('triage-card-triage-item-3');
    await expect(itemCard).toBeVisible({ timeout: 15000 });

    // Click the card header to expand detail view
    const cardHeader = page.getByTestId('triage-card-header-triage-item-3');
    await expect(cardHeader).toBeVisible();
    await cardHeader.click();

    const reviewButton = page.getByTestId('triage-review-btn-triage-item-3');
    await expect(reviewButton).toBeVisible();

    await reviewButton.click();

    const textarea = page.getByTestId('triage-edit-textarea-triage-item-3');
    await expect(textarea).toBeVisible();
    await expect(textarea).toHaveValue('Original draft payload');

    await textarea.fill('Edited draft payload');

    const saveButton = page.getByTestId('triage-save-btn-triage-item-3');
    await expect(saveButton).toBeVisible();
    await saveButton.click();

    await expect(itemCard).not.toBeVisible({ timeout: 5000 });
  });

  test('should handle a new triage item (dismiss)', async ({ page, request }) => {
    const tenantId = 'triage-test-tenant-dismiss-' + Date.now();

    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status)
          VALUES ('triage-item-2', '${tenantId}', 'Cust 2', 'Proactive Context Agent', 'low', 'Just an FYI', 'pending')
          ON CONFLICT DO NOTHING;
        `
      }
    });

    await page.goto(`/api/ui/triage.html?tenant_id=${tenantId}`);

    const itemCard = page.getByTestId('triage-card-triage-item-2');
    await expect(itemCard).toBeVisible({ timeout: 15000 });

    // Click the card header to expand detail view
    const cardHeader = page.getByTestId('triage-card-header-triage-item-2');
    await expect(cardHeader).toBeVisible();
    await cardHeader.click();

    const dismissButton = page.getByTestId('triage-dismiss-triage-item-2');
    await expect(dismissButton).toBeVisible();

    await dismissButton.click();
    await expect(itemCard).not.toBeVisible({ timeout: 5000 });
  });
});
