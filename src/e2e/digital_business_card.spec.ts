import { test, expect } from './fixtures';

test.describe('Digital Business Card Generator E2E', () => {
  test('should generate digital business card and handle soft paywall', async ({ page }) => {

    await test.step('Navigate and verify header', async () => {
      await page.goto('/digital-business-card');
      // Adding a small wait for the page to render fully
      await page.waitForTimeout(1000);
      await expect(page.locator('h1', { hasText: 'Digital Business Card Generator' })).toBeVisible({ timeout: 15000 });
    });

    await test.step('Fill out form and generate link', async () => {
      await page.fill('input[placeholder="e.g. Jane Doe"]', 'Carlos Repair');
      await page.fill('input[placeholder="e.g. Founder & CEO"]', 'Owner');
      await page.fill('input[placeholder="e.g. Acme Corp"]', 'Carlos Home Repair');
      await page.fill('input[placeholder="e.g. +1 (555) 123-4567"]', '+15559876543');

      await page.getByRole('button', { name: 'Generate Shareable Link' }).click();

      // Verify link is generated
      await expect(page.locator('input[readonly]')).toBeVisible();
      await expect(page.getByRole('button', { name: 'Copy' })).toBeVisible();
    });

    await test.step('Trigger soft paywall', async () => {
      // Check "Remove Powered by OHC branding" checkbox
      await page.locator('input[type="checkbox"]').click({ force: true });

      // Soft paywall should appear
      await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();
      await expect(page.getByText('Make the Digital Business Card 100% white-labeled')).toBeVisible();

      // Close soft paywall
      await page.getByRole('button', { name: 'Keep Watermark' }).click();
      await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeHidden();
    });

    await test.step('Verify generated view and viral loop', async () => {
      // Re-generate link just in case
      await page.getByRole('button', { name: 'Generate Shareable Link' }).click();

      // Get the link
      const linkInput = page.locator('input[readonly]');
      await expect(linkInput).toBeVisible();
      const generatedUrl = await linkInput.inputValue();

      // Navigate to generated URL
      await page.goto(generatedUrl);

      // Verify VCard data renders
      await expect(page.getByRole('heading', { name: 'Carlos Repair' })).toBeVisible({ timeout: 15000 });
      await expect(page.getByText('Carlos Home Repair')).toBeVisible();
      await expect(page.getByText('+15559876543')).toBeVisible();

      // Verify Save to Contacts button
      await expect(page.getByRole('button', { name: 'Save to Contacts' })).toBeVisible();

      // Verify viral loop footer
      await expect(page.getByText('Create your own free digital business card')).toBeVisible();
      await expect(page.getByText('Powered by OHC')).toBeVisible();
    });
  });
});
