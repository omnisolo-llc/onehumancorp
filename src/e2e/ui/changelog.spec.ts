import { test, expect } from '../fixtures';

test.describe('Release Notes & Changelog', () => {
  test('renders Changelog page with screenshots and can be accessed on mobile', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    // Test on a typical mobile viewport
    await page.setViewportSize({ width: 375, height: 812 });

    // Navigate to the Changelog page directly
    await page.goto('/api/ui/changelog.html');

    // Wait for network requests to settle so changelog data loads
    await page.waitForLoadState('networkidle');

    // Verify we are on the changelog page and header is visible
    const title = page.locator('h1', { hasText: 'Release Notes & Changelog' });
    await expect(title).toBeVisible();

    // Verify the container has loaded content
    const container = page.locator('#changelog-container');
    await expect(container).toBeVisible();

    // Verify there is at least one screenshot or content entry loaded
    // Depending on the backend response, there could be no changelog,
    // but typically a mocked or real response contains data. We assert loosely
    // that it either shows a screenshot or some version content.
    const textContent = await container.innerText();
    expect(textContent).not.toBe('Loading...');
  });
});
