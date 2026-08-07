import { test, expect } from './fixtures';

test.describe('Edge Ledger Sync Protocol UI Flow', () => {
  test('Owner can navigate to the Terminal setup page', async ({ page }) => {
    await page.goto('/settings/terminal');
    const heading = page.getByRole('heading', { name: 'Terminal' });
    await expect(heading).toBeVisible();

    const connectReaderBtn = page.getByRole('button', { name: 'Connect Reader' });
    await expect(connectReaderBtn).toBeVisible();
  });
});
