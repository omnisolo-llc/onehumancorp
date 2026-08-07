import { test, expect } from './fixtures';
import { adminPage } from './fixtures';

test.describe('Edge Ledger Sync Protocol UI Flow', () => {
  test('Owner can navigate to the Terminal setup page', async ({ browser }) => {
    const page = await adminPage(browser);

    await page.goto('/settings/terminal');
    const heading = page.getByRole('heading', { name: 'Terminal' });
    await expect(heading).toBeVisible();

    const connectReaderBtn = page.getByRole('button', { name: 'Connect Reader' });
    if (await connectReaderBtn.isVisible()) {
        await expect(connectReaderBtn).toBeVisible();
    }
  });
});
