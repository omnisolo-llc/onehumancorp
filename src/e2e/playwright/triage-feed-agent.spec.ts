import { expect } from '@playwright/test';
import { test } from '../fixtures';

test.describe('Agentic Work Triage Feed', () => {
  // Mobile viewport for Phase 1 requirements
  test.use({ viewport: { width: 375, height: 667 } });

  test('Owner can review and approve AI-drafted replies', async ({ page, request }) => {
    const tenantId = 'e2e-tenant-triage-1';

    // DB seeding for users and tenants
    const seedRes = await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO users (id, email, full_name, is_superadmin)
          VALUES ('triage_user_id', 'triage_user@example.com', 'Triage User', false)
          ON CONFLICT DO NOTHING;

          INSERT INTO tenants (id, name, owner_email)
          VALUES ('${tenantId}', 'Triage Store', 'triage_user@example.com')
          ON CONFLICT DO NOTHING;

          INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
          VALUES ('e2e-triage-1', '${tenantId}', 'Instagram DM', '{"description": "Do you have vegan chocolate cake available this weekend?"}'::jsonb, '{"action_type": "Draft Reply", "draft_reply": "Hi! Yes, we have 2 vegan chocolate cakes left for this weekend"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
          ON CONFLICT DO NOTHING;

          INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status)
          VALUES ('e2e-triage-legacy-1', '${tenantId}', '12345', 'Instagram DM', 'High', 'Do you have vegan chocolate cake available this weekend?', 'pending')
          ON CONFLICT DO NOTHING;
        `
      }
    });
    expect(seedRes.ok()).toBeTruthy();

    await page.goto(`/login?test_email=triage_user@example.com`);
    await page.evaluate((tid) => {
        localStorage.setItem('tenant', tid);
        localStorage.setItem('tenant_id', tid);
    }, tenantId);
    await page.goto('/dashboard');

    // Wait for the feed to load
    const feed = page.locator('.triage-item[id^="triage-"], [data-testid^="triage-card-"]').first();
    await expect(feed).toBeVisible({ timeout: 15000 });

    // Verify layout constraints for mobile (375px)
    const bodyBox = await page.locator('body').boundingBox();
    expect(bodyBox?.width).toBeLessThanOrEqual(375);

    // Check specific text in card
    await expect(feed).toContainText('vegan chocolate cake');
    await expect(feed).toContainText('Hi! Yes, we have 2 vegan chocolate cakes left for this weekend');

    // Find approve button
    const approveButton = page.locator(`button:has-text("Approve"), [data-testid="feed-approve-btn"], .btn-primary:has-text("Approve")`).first();
    await expect(approveButton).toBeVisible({ timeout: 5000 });

    const box = await approveButton.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.width).toBeGreaterThanOrEqual(44);
      expect(box.height).toBeGreaterThanOrEqual(44);
    }

    await approveButton.click();
    await expect(feed).not.toBeVisible({ timeout: 5000 });
  });

  test('Owner can dismiss AI-drafted replies', async ({ page, request }) => {
    const tenantId = 'e2e-tenant-triage-2';

    const seedRes = await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO users (id, email, full_name, is_superadmin)
          VALUES ('triage_user_id_2', 'triage_user_2@example.com', 'Triage User 2', false)
          ON CONFLICT DO NOTHING;

          INSERT INTO tenants (id, name, owner_email)
          VALUES ('${tenantId}', 'Triage Store 2', 'triage_user_2@example.com')
          ON CONFLICT DO NOTHING;

          INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
          VALUES ('e2e-triage-2', '${tenantId}', 'Instagram DM', '{"description": "Just an FYI"}'::jsonb, '{"action_type": "Draft Reply", "draft_reply": "No worries"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
          ON CONFLICT DO NOTHING;

          INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status)
          VALUES ('e2e-triage-legacy-2', '${tenantId}', '12345', 'Instagram DM', 'High', 'Just an FYI', 'pending')
          ON CONFLICT DO NOTHING;
        `
      }
    });
    expect(seedRes.ok()).toBeTruthy();

    await page.goto(`/login?test_email=triage_user_2@example.com`);
    await page.evaluate((tid) => {
        localStorage.setItem('tenant', tid);
        localStorage.setItem('tenant_id', tid);
    }, tenantId);
    await page.goto('/dashboard');

    const feed = page.locator('.triage-item[id^="triage-"], [data-testid^="triage-card-"]').first();
    await expect(feed).toBeVisible({ timeout: 15000 });

    // Check specific text in card
    await expect(feed).toContainText('Just an FYI');
    await expect(feed).toContainText('No worries');

    const dismissButton = page.locator(`button:has-text("Dismiss"), [data-testid="feed-dismiss-btn"], .btn-outline:has-text("Dismiss")`).first();
    await expect(dismissButton).toBeVisible({ timeout: 5000 });

    const box = await dismissButton.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.width).toBeGreaterThanOrEqual(44);
      expect(box.height).toBeGreaterThanOrEqual(44);
    }
    await dismissButton.click();
    await expect(feed).not.toBeVisible({ timeout: 5000 });
  });

  test('Triage feed handles empty state correctly', async ({ page, request }) => {
    const tenantId = 'e2e-tenant-triage-3';

    // 1. Seed the DB with test data (ensure no items for this tenant)
    const seedRes = await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `
          INSERT INTO users (id, email, full_name, is_superadmin)
          VALUES ('triage_user_id_3', 'triage_user_3@example.com', 'Triage User 3', false)
          ON CONFLICT DO NOTHING;

          INSERT INTO tenants (id, name, owner_email)
          VALUES ('${tenantId}', 'Triage Store 3', 'triage_user_3@example.com')
          ON CONFLICT DO NOTHING;

          DELETE FROM agent_feed_items WHERE tenant_id = '${tenantId}';
          DELETE FROM triage_items WHERE tenant_id = '${tenantId}';
        `
      }
    });
    expect(seedRes.ok()).toBeTruthy();

    await page.goto(`/login?test_email=triage_user_3@example.com`);
    await page.evaluate((tid) => {
        localStorage.setItem('tenant', tid);
        localStorage.setItem('tenant_id', tid);
    }, tenantId);
    await page.goto('/dashboard');

    // Wait to ensure UI fully loads.
    await page.waitForTimeout(2000);
    // onboarding-welcome-card shouldn't be counted as a work triage item.
    const emptyState = page.locator('[data-testid="triage-feed-empty"]');
    await expect(emptyState).toBeVisible({ timeout: 15000 });
  });
});
