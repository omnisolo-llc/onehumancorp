import { test, expect } from '../fixtures';

test.describe('Nora Proposal Intake', () => {
  test('Proposal dashboard renders', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto(`/admin/proposals`);
    await expect(page.getByRole('heading', { name: 'Proposals' })).toBeVisible({ timeout: 10000 });
  });
});
