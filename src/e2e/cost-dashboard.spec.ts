import { test, expect } from './fixtures';

test.describe('Cost Dashboard E2E', () => {

  test('Persona: Business Owner can view the cost transparency metrics and insights', async ({ page }) => {
    // 1. Owner Logs In
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('maya@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // 2. Navigate to Cost Dashboard
    await page.goto('/cost-dashboard');

    // 3. Verify Page Title
    await expect(page.getByRole('heading', { name: /Business Advisory Dashboard/i })).toBeVisible();

    // 4. Verify Advisory Summary
    await expect(page.getByRole('heading', { name: /Advisory Summary/i })).toBeVisible();
    await expect(page.getByText(/Recommendation:/i)).toBeVisible();

    // 5. Verify Cost Transparency section
    await expect(page.getByRole('heading', { name: /Cost Transparency/i })).toBeVisible();
    await expect(page.getByText('Total Costs')).toBeVisible();
    await expect(page.getByText('Total Revenue')).toBeVisible();
    await expect(page.getByText(/Period:/i)).toBeVisible();

    // 6. Verify Cost Breakdown section
    await expect(page.getByRole('heading', { name: /Cost Breakdown/i })).toBeVisible();
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.getByText('Storage', { exact: true })).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();
    await expect(page.getByText('Network & Bandwidth')).toBeVisible();
    await expect(page.getByText('Bandwidth Savings')).toBeVisible();

    // 7. Verify back button
    const backBtn = page.getByRole('button', { name: /Back to My Plan/i });
    await expect(backBtn).toBeVisible();
    await backBtn.click();
    await expect(page).toHaveURL(/\/plan/);
  });
});
