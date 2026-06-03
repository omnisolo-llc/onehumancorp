import { test, expect } from './fixtures';

test.describe('Mobile Inbox UX', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('verifies mobile-first buttons and simulation flow', async ({ page }) => {
    // 1. Start from home page as required
    await page.goto('/');

    // 2. Navigate to Inbox
    await page.click('text=Inbox');
    await expect(page).toHaveURL(/.*\/inbox/);

    // 3. Verify the Customer Inbox header is present
    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();

    // 4. Verify the new mobile-first back button
    const backBtn = page.getByLabel('Back to dashboard');
    await expect(backBtn).toBeVisible();

    // 5. Verify the new mobile-first simulate button
    const simulateBtn = page.getByLabel('Simulate Incoming Message');
    await expect(simulateBtn).toBeVisible();

    // 6. Simulate an incoming message
    await simulateBtn.click();

    // 7. Verify the simulated message appeared
    await expect(page.getByText('Are you open today?')).toBeVisible();

    // 8. Verify the AI drafted a reply
    const aiBadge = page.getByText('AI Replied');
    await expect(aiBadge).toBeVisible({ timeout: 10000 });

    // 9. Verify the specific draft reply text
    await expect(page.getByText('Hi! Yes, we are open until 6 PM today and we currently have 12 Vanilla Cupcakes left. Shall I set one aside for you?')).toBeVisible();

    // 10. Click the back button to return to dashboard
    await backBtn.click();
    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});
