import { expect, test } from '../../../../e2e/fixtures';

test.describe('Native Rust Omnichannel Chat System', () => {
  test('Owner unified inbox view on mobile', async ({ page }) => {
    test.setTimeout(60000);

    // Set viewport to mobile 375px
    await page.setViewportSize({ width: 375, height: 812 });

    // Wait for real-data backend messages call
    const inboxResponse = page.waitForResponse((response) =>
      response.url().includes('/api/v1/ui/inbox/messages') && response.request().method() === 'GET',
    );

    // Act like Maya logging in from her phone
    await page.goto('/inbox');
    const response = await inboxResponse;
    expect(response.status()).toBeLessThan(500);

    // Translucent glass styling container is verified implicitly by its presence
    await expect(page.locator('.glassmorphism').first()).toBeVisible();

    // Draft reply section is conditionally rendered if there is a draft
    const draftSection = page.getByText(/Draft Reply/i).first();
    if (await draftSection.isVisible()) {
      await expect(page.getByRole('button', { name: /Approve & Send/i }).first()).toBeVisible();
    }
  });
});
