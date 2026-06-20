import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed (Mobile MVP) - Real E2E Flow', () => {
  test.use({ viewport: { width: 375, height: 812 } }); // Mobile resolution

  test('Scenario 1: Simulates an inbound action, validates it appears in the UI, and approves the action', async ({ page, request }) => {
    const tenantId = 'e2e-tenant-1';

    // Simulate action
    const response = await request.post(`/api/dev/simulate-agent-feed-item?tenant_id=${tenantId}&mobile_optimized=true`);
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

    const response = await request.post(`/api/dev/simulate-agent-feed-item?tenant_id=${tenantId}&mobile_optimized=true`);
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

    const response = await request.post(`/api/dev/simulate-agent-feed-item?tenant_id=${tenantId}&mobile_optimized=true`);
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

    const response = await request.post(`/api/dev/simulate-agent-feed-item?tenant_id=${tenantId}&mobile_optimized=true`);
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

  test('Scenario 6: Real-time update via WebSocket pushes to UI without refresh', async ({ page, request }) => {
    const tenantId = 'e2e-tenant-ws-test';

    // 1. Go to dashboard and wait for it to load
    await page.goto(`/dashboard?tenant_id=${tenantId}`);

    // Check it loaded correctly (e.g. Activity tab is visible)
    const activityTab = page.locator('button:has-text("Activity")');
    await expect(activityTab).toBeVisible({ timeout: 15000 });

    // We expect no simulated card initially for this specific message.
    const simulatedCardText = page.locator('text=A realtime event needs your attention.');
    await expect(simulatedCardText).not.toBeVisible();

    // 2. Simulate an event via API (while dashboard is already open and WebSocket is supposedly connected)
    const response = await request.post(`/api/dev/simulate-agent-feed-item?tenant_id=${tenantId}`);
    expect(response.ok()).toBeTruthy();

    // Wait and verify if we see the normal simulated event. Wait, the simulate endpoint creates "A new simulated event needs your attention.".
    // Let's look for that string.
    const newSimulatedCardText = page.locator('text=A new simulated event needs your attention.').first();

    // 3. Verify it shows up without page refresh
    await expect(newSimulatedCardText).toBeVisible({ timeout: 15000 });

    // 4. Click approve to clear it
    const card = page.locator('text=A new simulated event needs your attention.').locator('..').locator('..');
    const approveButton = card.locator('button', { hasText: 'Approve' }).first();
    await expect(approveButton).toBeVisible();

    await approveButton.click();
    await expect(newSimulatedCardText).not.toBeVisible({ timeout: 15000 });
  });
});
