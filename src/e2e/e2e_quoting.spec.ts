import { test as base, expect } from './fixtures';

const test = base.extend({
  page: async ({ page }, use) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await use(page);
  }
});

test.describe('Quoting UI e2e', () => {
  test('owner can navigate to quoting page, view a real quote from the backend, and approve it', async ({ adminUser, loginAs, page }) => {
    await loginAs(page, adminUser);
    // Navigate to the quoting page with a quote that we seeded in the db
    await page.goto('/quoting?id=00000000-0000-0000-0000-000000000001');

    await expect(page.getByText('Project Proposal')).toBeVisible({ timeout: 15000 });

    const approveBtn = page.getByRole('button', { name: 'Accept Proposal' });
    await expect(approveBtn).toBeVisible();

    await approveBtn.click();
    await expect(page.getByText('Proposal Accepted')).toBeVisible();
  });
});
