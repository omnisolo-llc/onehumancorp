import { test, expect } from '../../../../../src/e2e/fixtures';

test.describe('Dashboard Triage Edit', () => {
  test('Triage config renders', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto(`/settings/triage`);
    await expect(page.getByRole('heading', { name: 'Triage' })).toBeVisible({ timeout: 10000 });
  });
});
