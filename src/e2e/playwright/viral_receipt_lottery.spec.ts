import { test, expect } from '@playwright/test';
import { observeButtonStates } from '../../../scripts/playwright/button-states.mjs';

test.describe('Viral Receipt Lottery Generator', () => {
  test('should load the generator and generate a lottery link', async ({ page }) => {
    // 2. Navigate to dashboard and click the new link
    await page.goto('/ui/dashboard.html');
    const link = page.locator('#viral-receipt-lottery-link');
    await expect(link).toBeVisible();
    await link.click();

    // 3. Wait for main elements
    await expect(page.locator('h1')).toHaveText('Viral Receipt Lottery 🎟');
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    // Finish the page-load generation before observing the separate user click.
    const previewUrl = page.locator('#preview-url');
    await expect(previewUrl).toHaveText(/^cloud\.omnisolo\.co\/win\/[\da-f-]{36}$/);
    const resultArea = page.locator('#result-area');
    await expect(resultArea).not.toBeVisible();

    // Record actual DOM transitions before clicking: a fast real response may
    // restore the button before click() returns to the test runner.
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
      await expect(resultArea).toBeVisible({ timeout: 5000 });
      await expect(page.locator('#share-link')).toHaveValue(new URL(`/win/${refId}`, page.url()).href);
      await expect(previewUrl).toHaveText(`cloud.omnisolo.co/win/${refId}`);
      await expect(generateBtn).toBeEnabled();
      await expect(generateBtn).toHaveText('Generate Lottery Link');
    } finally {
      await observation.evaluate(value => value.disconnect());
      await observation.dispose();
    }
  });
});
