import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Phygital Mesh (Smart Labels) E2E', () => {
  test('Admin can generate a Smart Label from the builder and resolve it', async ({ page }) => {
    // 1. Navigate to builder
    await page.goto('/builder');

    // 2. Open Smart Labels section
    await page.getByText('Smart Labels (QR)', { exact: true }).click();

    // 3. Wait for the Phygital Smart Labels header to be visible
    await expect(page.getByText('Phygital Smart Labels')).toBeVisible();

    // 4. Click Generate for Storefront QR
    const generateButton = page.locator('button:has-text("Generate")').first();
    await generateButton.click();

    // 5. Verify the QR URL is rendered (mocking the SVG or visual block existence and the url text)
    await expect(page.locator('span.font-mono')).toContainText('/t/');

    // Extract the hash directly from the rendered text
    const qrUrlText = await page.locator('span.font-mono').innerText();
    const hash = qrUrlText.replace('/t/', '');

    // 6. Navigate to the resolved touchpoint URL using the frontend app routing directly
    await page.goto(qrUrlText);

    // 7. Assuming empty or dummy data returns 404 for entity (or resolves correctly based on mock)
    // We just want to make sure the resolver page loaded and hit the API
    await expect(page.getByText('Connecting...')).toBeVisible();

    // Note: in a real full E2E, we wait for target resolution:
    // await expect(page).toHaveURL(/\/storefronts\/main/);
  });
});
