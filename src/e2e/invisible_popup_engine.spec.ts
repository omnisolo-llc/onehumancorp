import { test, expect } from './fixtures';

test.describe('Invisible Dynamic Pop-Up Engine', () => {
    test('should launch a pop-up store, select inventory, and simulate pos mode', async ({ page }) => {
        await page.goto('/dashboard');

        await page.click('text="Pop-Up Store"');

        await expect(page).toHaveURL(/.*pop-up.*/);
        await expect(page.locator('h1')).toContainText('Pop-Up Storefront');

        await page.click('text="Vegan Cupcakes (Dozen)"');

        // Wait for the API route mock to settle during click
        await Promise.all([
          page.waitForResponse(resp => resp.url().includes('/api/v1/popup') && resp.status() === 200),
          page.click('text="Launch Pop-Up Node"')
        ]);

        await expect(page.locator('h2')).toContainText('Pop-Up Live!');
        await expect(page.locator('text="Return to Dashboard"')).toBeVisible();

        await page.click('text="Return to Dashboard"');
        await expect(page).toHaveURL(/.*dashboard.*/);
    });

    test('should disable button if no inventory is selected', async ({ page }) => {
      await page.goto('/pop-up');
      const launchBtn = page.getByRole('button', { name: 'Launch Pop-Up Node' });
      await expect(launchBtn).toBeDisabled();

      await page.click('text="Vegan Cupcakes (Dozen)"');
      await expect(launchBtn).toBeEnabled();

      await page.click('text="Vegan Cupcakes (Dozen)"');
      await expect(launchBtn).toBeDisabled();
    });

    test('should send correct inventory payload to backend API', async ({ page }) => {
      await page.goto('/pop-up');

      await page.click('text="Vegan Cupcakes (Dozen)"');
      await page.click('text="Custom Wedding Cake Tier"');

      const requestPromise = page.waitForRequest(req => req.url().includes('/api/v1/popup') && req.method() === 'POST');
      await page.click('text="Launch Pop-Up Node"');
      const request = await requestPromise;

      const postData = JSON.parse(request.postData() || '{}');
      expect(postData.items).toContain('1');
      expect(postData.items).toContain('2');
      expect(postData.items).not.toContain('3');
    });

    test('should be accessible on mobile viewport', async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/pop-up');

      await expect(page.locator('h1')).toContainText('Pop-Up Storefront');
      await expect(page.locator('text="Select Inventory to Split"')).toBeVisible();

      // Ensure the button is full width (mobile friendly)
      const btn = page.getByRole('button', { name: 'Launch Pop-Up Node' });
      const boundingBox = await btn.boundingBox();
      expect(boundingBox?.width).toBeGreaterThan(300);
    });

    test('should ensure Grandmother test is passed (no jargon)', async ({ page }) => {
      await page.goto('/pop-up');

      const textContent = await page.content();
      expect(textContent).not.toContain('SQL');
      expect(textContent).not.toContain('Kubernetes');
      expect(textContent).not.toContain('API');
      expect(textContent).not.toContain('Payload');

      await expect(page.getByText('Choose the items you are bringing to this pop-up location.')).toBeVisible();
    });
});
