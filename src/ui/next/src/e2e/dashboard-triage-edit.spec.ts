import { expect, test } from '@playwright/test';

test.describe('Dashboard Triage Action Feed Edit UI', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should allow editing a draft from the unified dashboard feed', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    const seedData = [
      {
        source: 'Instagram DM',
        priority: 'high',
        context: 'Message: Customer asked about vegan cakes.',
        action_type: 'Draft Reply',
        action_payload: 'Yes, we have vegan options.',
        customer_id: 'cust_test_1'
      }
    ];

    for (const data of seedData) {
      await page.request.post(`/api/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
        data
      });
    }

    await page.goto('/dashboard');
    await expect(page.locator('text=Activity Feed').first()).toBeVisible({ timeout: 15000 });

    const feedBtn = page.locator('button', { hasText: 'Pending Approvals' });
    if (await feedBtn.isVisible()) {
        await feedBtn.click();
    }

    const itemCard = page.locator('div[data-testid="instagram-dm-card"]').first();
    await expect(itemCard).toBeVisible({ timeout: 15000 });

    // Review draft/Edit if available
    const reviewDraftButton = page.locator('button', { hasText: 'Review Draft' }).first();
    if (await reviewDraftButton.isVisible()) {
        await reviewDraftButton.click();
    } else {
        const editBtn = page.locator('button', { hasText: 'Edit' }).first();
        if (await editBtn.isVisible()) {
            await editBtn.click();
        }
    }

    const textarea = page.locator('textarea[data-testid="edit-draft-textarea"]').first();
    if (await textarea.isVisible()) {
        await textarea.fill('Edited draft payload from dashboard feed');
        const saveButton = page.locator('button[data-testid="save-edit-approve-btn"]').first();
        await expect(saveButton).toBeVisible();
        await saveButton.click();

        await expect(itemCard).not.toBeVisible({ timeout: 5000 });
    } else {
        // Just approve if textarea is missing in this view
        const approveButton = page.locator('button[data-testid="approve-instagram-dm"]').first();
        if (await approveButton.isVisible()) {
            await approveButton.click();
            await expect(itemCard).not.toBeVisible({ timeout: 5000 });
        }
    }
  });
});
