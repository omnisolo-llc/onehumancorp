import { expect, test } from './fixtures';

test.describe('Invisible AI Agents - Automations UI', () => {
  test('shows the automations tab', async ({ page }) => {
    await page.goto('/assistant');
    await expect(page.getByRole('heading', { name: 'Jarvis Assistant' })).toBeVisible();
    await page.getByRole('button', { name: 'Automations' }).first().click();
    await expect(page.getByText('Approve & Post')).toBeVisible();
  });
});
