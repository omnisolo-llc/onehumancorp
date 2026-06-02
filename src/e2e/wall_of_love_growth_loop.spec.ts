import { test, expect } from './fixtures';

test.describe('Wall of Love Growth Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');
  });

  test('should generate the Wall of Love widget and verify the viral loop embed renders', async ({ page }) => {
    // 1. Locate and click "Generate Widget"
    await expect(page.getByRole('heading', { name: 'Wall of Love Widget' })).toBeVisible();
    await page.getByRole('button', { name: 'Generate Widget' }).click();

    // 2. Verify the Wall of Love modal appears
    await expect(page.getByRole('heading', { name: 'Your Wall of Love' })).toBeVisible();

    // 3. Verify the generated HTML contains the iframe
    const textarea = page.locator('textarea[readOnly]');
    await expect(textarea).toBeVisible();
    await expect(textarea).toHaveValue(/<iframe src="https:\/\/ohc.app\/api\/v1\/growth\/wall_of_love\/embed\?tenant=/);

    // 4. Navigate to the actual iframe URL
    const textareaValue = await textarea.inputValue();
    const iframeSrcMatch = textareaValue.match(/src="([^"]+)"/);
    expect(iframeSrcMatch).not.toBeNull();
    const iframeUrl = iframeSrcMatch![1];

    // We convert the "ohc.app" domain to a relative path or local host for testing the api route.
    // In our test environment, Next.js routes are available at the base URL.
    const url = new URL(iframeUrl);
    const localIframeUrl = url.pathname + url.search;

    await page.goto(localIframeUrl);

    // 5. Verify the widget renders properly
    await expect(page.getByText('Wall of Love')).toBeVisible();
    await expect(page.getByText('Absolutely amazing product!')).toBeVisible();

    // 6. Verify the viral loop footer "Powered by OHC" is present
    await expect(page.getByText('Powered by')).toBeVisible();

    const ohcLink = page.locator('a', { hasText: 'OHC' });
    await expect(ohcLink).toBeVisible();

    const href = await ohcLink.getAttribute('href');
    expect(href).toMatch(/https:\/\/ohc\.store\/join\?ref=/);
  });
});
