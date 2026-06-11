import { test, expect } from '@playwright/test';

test.describe('The Ambassador Agent - Native Social Inbox Auto-Responder', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display drafted ambassador reply and approve it', async ({ page, request }) => {
    test.setTimeout(60000);

    // 1. Authenticate user
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // 2. Trigger the simulation
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || 'http://127.0.0.1:18789';
    const simulateRes = await request.post(`${apiBase}/api/agents/approvals/simulate-ambassador-reply`);
    expect(simulateRes.ok()).toBeTruthy();

    // 3. Navigate back to Dashboard to refresh state
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // 4. Wait for the feed item to appear
    const feedContainer = page.locator('div.glassmorphism', { hasText: 'Approval' }).first();
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    // 5. Locate the Ambassador Reply Card
    const ambassadorCard = page.getByTestId('ambassador-reply-card').first();
    await expect(ambassadorCard).toBeVisible({ timeout: 10000 });

    // 6. Verify the content is visible
    await expect(ambassadorCard).toContainText('Do you have vegan chocolate cake available for Saturday?');
    await expect(ambassadorCard).toContainText('Yes we do! We have 3 left for this Saturday');

    // 7. Find the buttons
    const approveBtn = page.getByTestId('approve-ambassador-reply').first();
    const editBtn = page.getByTestId('edit-ambassador-reply').first();

    await expect(approveBtn).toBeVisible();
    await expect(editBtn).toBeVisible();

    // 8. Approve the reply
    const cardParent = approveBtn.locator('xpath=./../../..');
    await approveBtn.click();

    // 9. Wait for transition state to complete
    await expect(cardParent).toHaveClass(/scale-95/);
    await expect(cardParent).not.toBeVisible({ timeout: 5000 });
  });
});
