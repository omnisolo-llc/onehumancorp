import { expect, test } from './fixtures';

test.describe('Invisible AI Agents - Automations UI', () => {
  test('shows the automations tab', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
    await page.getByRole('button', { name: 'Automations' }).first().click();
    await expect(page.getByText('Approve & Post')).toBeVisible();
  });
});
