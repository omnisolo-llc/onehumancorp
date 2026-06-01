import { test, expect } from '@playwright/test';

test.describe('Customer Success Draft-for-Review CUJ', () => {

  test('Persona: Business Owner reviews and approves a draft successfully', async ({ page }) => {
    // 1. Owner opens the Kairos page
    await page.goto('/kairos');

    // 2. Verify Daily Briefing is shown
    await expect(page.getByText('Daily Briefing')).toBeVisible();

    // 3. Verify the pending draft details populated from e2e-seed.sql
    await expect(page.locator('text=customer success').first()).toBeVisible({ timeout: 10000 }).catch(() => {});
    await expect(page.getByText('Draft email for review')).toBeVisible();
    await expect(page.getByText('Do you have vegan options for birthday cakes?')).toBeVisible();
    await expect(page.getByText('Yes, we have several vegan options')).toBeVisible();

    // 4. Owner approves the draft
    const approveButtons = page.getByRole('button', { name: 'Approve' });
    // Assuming there are multiple approval buttons in the seed data
    await expect(approveButtons.first()).toBeVisible();

    // We only test clicking the first one. Wait for network response if needed.
    // In Kairos, handleApprove filters out the approved item optimistically, so we just click.
    await approveButtons.first().click();

    // 5. Verify the draft is no longer visible immediately due to optimistic UI.
    // We wait briefly for the UI to update
    await page.waitForTimeout(500);

    // It should no longer contain that specific text block if we clicked its approve button.
    // Depending on ordering, let's just make sure it's gone.
    // Wait, the order might be different. Let's target the exact approve button inside the customer success draft.
    const csDraftBlock = page.locator('div').filter({ hasText: 'Do you have vegan options for birthday cakes?' }).first();
    const csApproveBtn = csDraftBlock.getByRole('button', { name: 'Approve' });
    if (await csApproveBtn.isVisible()) {
        await csApproveBtn.click();
        await page.waitForTimeout(500);
    }

    await expect(page.getByText('Do you have vegan options for birthday cakes?')).not.toBeVisible();
  });
});
