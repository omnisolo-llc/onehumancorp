import { test, expect } from './fixtures';

test.describe('Viral Loyalty Program Generator', () => {
  test('should display loyalty program generator, allow settings updates, and simulate launching', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // From dashboard, navigate to the Loyalty Program page
    const dashboard = page.locator('#dashboard-screen');
    await expect(dashboard).toBeVisible();
    await page.locator('#loyalty-nav-link').click();

    // Verify Loyalty Program page loads
    await expect(page.getByRole('heading', { name: 'Loyalty Program Generator' })).toBeVisible();

    // Simulate Maya the Baker inputting her program details
    await page.fill('input[placeholder="e.g. Maya\'s Sweets"]', 'Maya The Baker');
    await page.fill('input[placeholder="e.g. Gold, VIP, Superfan"]', 'Cupcake VIP');
    await page.fill('input[placeholder="e.g. $10, 15%"]', '$15');

    // Update preview
    await page.getByRole('button', { name: 'Update Preview' }).click();

    // Verify live preview reflects the changes
    await expect(page.getByText('Maya The Baker')).toBeVisible();
    await expect(page.getByText('Cupcake VIP Status Unlocked')).toBeVisible();
    await expect(page.getByText('Give $15, Get $15 when you refer friends!')).toBeVisible();

    // Simulate Launching Campaign (Copy Message)
    const sendButton = page.getByRole('button', { name: 'Email to Customers' });
    await sendButton.click();
    // After click it should transition
    await expect(page.getByRole('button', { name: 'Emails Sent!' })).toBeVisible();
  });
});
