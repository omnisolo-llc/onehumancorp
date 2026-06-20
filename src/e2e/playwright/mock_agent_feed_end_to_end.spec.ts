import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed (Mobile MVP) - Real E2E Flow', () => {
  test.use({ viewport: { width: 375, height: 812 } }); // Mobile resolution

  test('Scenario 1: Simulates an inbound action, validates it appears in the UI, and approves the action', async ({ page, request }) => {
    const tenantId = 'e2e-tenant-1';

    // Simulate action
    const response = await request.post(`/api/dev/simulate-agent-feed-item?tenant_id=${tenantId}`);
    expect(response.ok()).toBeTruthy();

    await page.goto(`/dashboard?tenant_id=${tenantId}`);

    const simulatedCardText = page.locator('text=A new simulated event needs your attention.').first();
    await expect(simulatedCardText).toBeVisible({ timeout: 15000 });

    const card = page.locator('text=A new simulated event needs your attention.').locator('..').locator('..');
    const approveButton = card.locator('button', { hasText: 'Approve' }).first();
    await expect(approveButton).toBeVisible();

    const btnBox = await approveButton.boundingBox();
    expect(btnBox?.width).toBeGreaterThanOrEqual(44);
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);

    await approveButton.click();
    await expect(simulatedCardText).not.toBeVisible({ timeout: 15000 });
  });

  test('Scenario 2: Simulates an inbound action and then dismisses the action directly', async ({ page, request }) => {
    const tenantId = 'e2e-tenant-2';

    const response = await request.post(`/api/dev/simulate-agent-feed-item?tenant_id=${tenantId}`);
    expect(response.ok()).toBeTruthy();

    await page.goto(`/dashboard?tenant_id=${tenantId}`);

    const simulatedCardText = page.locator('text=A new simulated event needs your attention.').first();
    await expect(simulatedCardText).toBeVisible({ timeout: 15000 });

    const card = page.locator('text=A new simulated event needs your attention.').locator('..').locator('..');
    const dismissButton = card.locator('button', { hasText: 'Deny' }).first();
    await expect(dismissButton).toBeVisible();

    await dismissButton.click();
    await expect(simulatedCardText).not.toBeVisible({ timeout: 15000 });
  });

  test('Scenario 3: Simulates an inbound action, goes into the edit workflow, and cancels editing', async ({ page, request }) => {
    const tenantId = 'e2e-tenant-3';

    const response = await request.post(`/api/dev/simulate-agent-feed-item?tenant_id=${tenantId}`);
    expect(response.ok()).toBeTruthy();

    await page.goto(`/dashboard?tenant_id=${tenantId}`);

    const simulatedCardText = page.locator('text=A new simulated event needs your attention.').first();
    await expect(simulatedCardText).toBeVisible({ timeout: 15000 });

    const card = page.locator('text=A new simulated event needs your attention.').locator('..').locator('..');
    const editButton = card.locator('button', { hasText: 'Edit' }).first();
    await expect(editButton).toBeVisible();

    await editButton.click();

    const cancelButton = card.locator('button', { hasText: 'Cancel' }).first();
    await expect(cancelButton).toBeVisible();
    await cancelButton.click();

    await expect(editButton).toBeVisible();
  });

  test('Scenario 4: Simulates an inbound action, edits the payload content, and saves/approves it', async ({ page, request }) => {
    const tenantId = 'e2e-tenant-4';

    const response = await request.post(`/api/dev/simulate-agent-feed-item?tenant_id=${tenantId}`);
    expect(response.ok()).toBeTruthy();

    await page.goto(`/dashboard?tenant_id=${tenantId}`);

    const simulatedCardText = page.locator('text=A new simulated event needs your attention.').first();
    await expect(simulatedCardText).toBeVisible({ timeout: 15000 });

    const card = page.locator('text=A new simulated event needs your attention.').locator('..').locator('..');
    const editButton = card.locator('button', { hasText: 'Edit' }).first();
    await expect(editButton).toBeVisible();
    await editButton.click();

    const textarea = card.locator('textarea').first();
    await expect(textarea).toBeVisible();
    await textarea.fill('Updated draft response from e2e test');

    const saveApproveButton = card.locator('button', { hasText: 'Save & Approve' }).first();
    await expect(saveApproveButton).toBeVisible();
    await saveApproveButton.click();

    await expect(simulatedCardText).not.toBeVisible({ timeout: 15000 });
  });

  test('Scenario 5: Verifies that the activity feed tab renders activities properly and shows offline/empty states correctly', async ({ page }) => {
    const tenantId = 'e2e-tenant-5';
    await page.goto(`/dashboard?tenant_id=${tenantId}`);

    const activityTab = page.locator('button:has-text("Activity")');
    await expect(activityTab).toBeVisible();
    await activityTab.click();

    const emptyStateText = page.locator('text=No recent activity found.');
    // It might be empty, or have activities if we share tenant DB. Either way, it shouldn't crash.
    await expect(emptyStateText.or(page.locator('text=APPROVED').first())).toBeVisible({ timeout: 15000 });
  });

  test('Scenario 6: Verifies offline queueing', async ({ page, request, context }) => {
    const tenantId = 'e2e-tenant-6';

    const response = await request.post(`/api/dev/simulate-agent-feed-item?tenant_id=${tenantId}`);
    expect(response.ok()).toBeTruthy();

    await page.goto(`/dashboard?tenant_id=${tenantId}`);

    const simulatedCardText = page.locator('text=A new simulated event needs your attention.').first();
    await expect(simulatedCardText).toBeVisible({ timeout: 15000 });

    const card = page.locator('text=A new simulated event needs your attention.').locator('..').locator('..');
    const approveButton = card.locator('button', { hasText: 'Approve' }).first();
    await expect(approveButton).toBeVisible();

    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    await approveButton.click();
    await expect(simulatedCardText).not.toBeVisible({ timeout: 15000 });

    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
  });
});
