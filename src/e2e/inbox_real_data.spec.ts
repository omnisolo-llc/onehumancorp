import { expect, test } from './fixtures';

test.describe('Inbox real-data behavior', () => {
  test('loads conversations from the database-backed inbox endpoint', async ({ page }) => {
    test.setTimeout(60000);

    const inboxResponse = page.waitForResponse((response) =>
      response.url().includes('/api/v1/ui/inbox/messages') && response.request().method() === 'GET',
    );

    await page.goto('/inbox');
    const response = await inboxResponse;

    expect(response.status(), '/api/v1/ui/inbox/messages must be reachable').toBeLessThan(500);
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();
    await expect(page.getByText('Loaded from `/api/v1/ui/inbox/messages`.')).toBeVisible();
  });

  test('does not expose simulated inbox controls or silent send-message no-ops', async ({ page }) => {
    await page.goto('/inbox');

    await expect(page.getByRole('button', { name: /simulate incoming message/i })).toHaveCount(0);
    await expect(page.getByText(/AI Replied/i)).toHaveCount(0);

    const sendButtons = page.getByRole('button', { name: /send message/i });
    const sendButtonCount = await sendButtons.count();

    for (let index = 0; index < sendButtonCount; index += 1) {
      const button = sendButtons.nth(index);
      const before = await page.locator('body').innerText();
      let responseSeen = false;

      const responsePromise = page.waitForResponse((response) => {
        const method = response.request().method();
        return ['POST', 'PUT', 'PATCH'].includes(method) && response.status() < 500;
      }, { timeout: 5000 }).then(() => {
        responseSeen = true;
      }).catch(() => undefined);

      await button.click();
      await responsePromise;
      await page.waitForTimeout(250);

      const after = await page.locator('body').innerText();
      expect(responseSeen || after !== before, 'Send message must produce a real response or visible state change').toBeTruthy();
    }
  });
});
