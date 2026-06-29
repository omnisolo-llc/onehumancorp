import { test, expect } from './fixtures';

test.describe('Proactive Operations Task Feed', () => {
  test('Persona: Jun the Location Manager opens app and interacts with proactive ops tasks', async ({ page, context }) => {
    await page.goto('/dashboard');

    await page.waitForTimeout(2000);

    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible({ timeout: 15000 });

    const proposalsTab = page.locator('button#tab-proposals');
    if (await proposalsTab.isVisible()) {
      await proposalsTab.click();
    }

    const checklistCard = page.locator('div[data-testid^="triage-card-"]', { hasText: 'Review Daily Prep Checklist' });
    await expect(checklistCard).toBeVisible({ timeout: 15000 });

    const supplierCard = page.locator('div[data-testid^="triage-card-"]', { hasText: 'Follow up on delayed supplier delivery from yesterday' });
    await expect(supplierCard).toBeVisible();

    const staffingCard = page.locator('div[data-testid^="triage-card-"]', { hasText: 'Staffing alert: Only 1 person scheduled for closing shift.' });
    await expect(staffingCard).toBeVisible();

    await checklistCard.locator('button', { hasText: 'Review Checklist' }).click();
    await supplierCard.locator('button', { hasText: 'Assign to Staff' }).click();
    await staffingCard.locator('button', { hasText: 'Draft Schedule Request' }).click();

    await expect(checklistCard).not.toBeVisible();
    await expect(supplierCard).not.toBeVisible();
    await expect(staffingCard).not.toBeVisible();
  });
});
