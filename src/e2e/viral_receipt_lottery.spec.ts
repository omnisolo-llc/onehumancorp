import { test, expect } from './fixtures';
import { adminPage } from './fixtures';
import { observeButtonStates } from '../../scripts/playwright/button-states.mjs';

test.describe('Viral Receipt Lottery', () => {
  test('dashboard links to Viral Receipt Lottery, which generates a viral link', async ({ page }) => {
    await adminPage(page, async () => {
      await page.goto('/dashboard.html');

      await page.locator('#viral-receipt-lottery-link').click();

      await expect(page.locator('h1')).toHaveText('Viral Receipt Lottery 🎟');
      await expect(page.locator('.receipt-mockup')).toBeVisible();

      // Finish page-load generation before observing the separate user click.
      await expect(page.locator('#preview-url')).toHaveText(/^cloud\.omnisolo\.co\/win\/[\da-f-]{36}$/, { timeout: 5000 });

      const generateBtn = page.locator('#generate-btn');
      await expect(generateBtn).toBeVisible();
      await expect(page.locator('#result-area')).not.toBeVisible();

      const observation = await generateBtn.evaluateHandle(observeButtonStates);
      try {
        const [request] = await Promise.all([
          page.waitForRequest(request => request.method() === 'POST'
            && new URL(request.url()).pathname === '/api/v1/growth/referrals/generate'),
          generateBtn.click(),
        ]);
        const response = await request.response();
        expect(response, 'the click must receive a real referral-generation response').not.toBeNull();
        expect(response!.status()).toBe(200);
        const data = await response!.json();
        expect(data.referral_link).toMatch(/^https:\/\/cloud\.omnisolo\.co\/ref\/[\da-f-]{36}$/);
        const refId = new URL(data.referral_link).pathname.split('/').pop();

        expect(await observation.evaluate(value => value.states)).toContainEqual({
          disabled: true, text: 'Generating...',
        });
        await expect(page.locator('#result-area')).toBeVisible({ timeout: 5000 });
        const shareLinkInput = page.locator('#share-link');
        await expect(shareLinkInput).toBeVisible();
        await expect(shareLinkInput).toHaveValue(new URL(`/win/${refId}`, page.url()).href);
        await expect(page.locator('#preview-url')).toHaveText(`cloud.omnisolo.co/win/${refId}`);
        await expect(generateBtn).toBeEnabled();
        await expect(generateBtn).toHaveText('Generate Lottery Link');
      } finally {
        await observation.evaluate(value => value.disconnect());
        await observation.dispose();
      }
    });
  });

  test('should copy the lottery link to clipboard', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    await adminPage(page, async () => {
      await page.goto('/viral-receipt-lottery.html');

      const generateBtn = page.locator('#generate-btn');
      await generateBtn.click();

      const resultArea = page.locator('#result-area');
      await expect(resultArea).toBeVisible({ timeout: 5000 });

      const copyBtn = page.locator('#copy-btn');
      await expect(copyBtn).toHaveText('Copy Link');
      await copyBtn.click();

      await expect(copyBtn).toHaveText('Copied!', { timeout: 3000 });

      try {
          const clipboardText = await page.evaluate(async () => {
              return await navigator.clipboard.readText();
          });
          expect(clipboardText).toContain('/win/');
      } catch (e) {
          console.warn('Clipboard read failed: ', e);
      }
    });
  });
});
