import { test, expect } from './fixtures';

test.describe('Agentic Work Triage Feed', () => {
  test('Owner can review and approve AI-drafted replies', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const response = await page.request.post(`/api/dev/simulate-triage-item?tenant_id=default`);
    expect(response.status()).toBe(200);
    const json = await response.json();
    const triageItemId = json.id;

    await page.goto('/dashboard');

    const feed = page.locator('[data-testid^="triage-card-"]').first();
    await expect(feed).toBeVisible({ timeout: 10000 });

    const card = page.locator(`[data-testid="triage-card-${triageItemId}"]`);
    await expect(card).toBeVisible({ timeout: 10000 });

    // Verify translucent glass container style
    await expect(card).toHaveCSS('backdrop-filter', 'blur(30px) saturate(210%)');

    const approveButton = card.locator(`[data-testid*="triage-approve"]`);
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    await expect(card).not.toBeVisible({ timeout: 5000 });
  });

  test('Owner can reject AI-drafted replies', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const response = await page.request.post(`/api/dev/simulate-triage-item?tenant_id=default`);
    const json = await response.json();
    const triageItemId = json.id;

    await page.goto('/dashboard');

    const card = page.locator(`[data-testid="triage-card-${triageItemId}"]`);
    await expect(card).toBeVisible({ timeout: 10000 });

    const rejectButton = card.locator(`[data-testid*="triage-reject"]`);
    await expect(rejectButton).toBeVisible();
    await rejectButton.click();

    await expect(card).not.toBeVisible({ timeout: 5000 });
  });

  test('Owner can edit AI-drafted replies', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const response = await page.request.post(`/api/dev/simulate-triage-item?tenant_id=default`);
    const json = await response.json();
    const triageItemId = json.id;

    await page.goto('/dashboard');

    const card = page.locator(`[data-testid="triage-card-${triageItemId}"]`);
    await expect(card).toBeVisible({ timeout: 10000 });

    const editButton = card.locator(`[data-testid*="triage-edit"]`);
    await expect(editButton).toBeVisible();

    // We expect edit to open a modal or do something, for now just check it exists and is clickable
    await editButton.click();
  });

  test('Triage feed layout is responsive at 375px', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.setViewportSize({ width: 375, height: 812 });

    const response = await page.request.post(`/api/dev/simulate-triage-item?tenant_id=default`);
    const json = await response.json();
    const triageItemId = json.id;

    await page.goto('/dashboard');

    const card = page.locator(`[data-testid="triage-card-${triageItemId}"]`);
    await expect(card).toBeVisible({ timeout: 10000 });

    const box = await card.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);

    // Check horizontal scroll doesn't exist
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(375);
  });

  test('System correctly ingests webhook and shows in feed', async ({ page, loginAs, adminUser }) => {
     await loginAs(page, adminUser);
     const res = await page.request.post('/api/v1/omnichannel/webhook', {
        data: {
            tenant_id: 'default',
            source: 'Instagram DM',
            sender_id: 'john_doe',
            message: 'Can you fix my sink on Tuesday?',
        }
     });
     expect(res.status()).toBe(200);

     await page.goto('/dashboard');
     const card = page.locator('[data-testid^="triage-card-"]', { hasText: 'Sink' }).first();

     // Due to background job, we wait a bit
     await expect(card).toBeVisible({ timeout: 15000 });
     await expect(card).toHaveCSS('backdrop-filter', 'blur(30px) saturate(210%)');
  });
});
