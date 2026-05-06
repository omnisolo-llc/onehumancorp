import { test, expect } from '@playwright/test';

test.describe('Pricing UI UX Friction Plain Language', () => {
  test('should display plain language product limit message after adding too many products', async ({ page }) => {
    await page.goto('/');

    const loginEmailInput = page.getByPlaceholder(/email/i).first();
    const loginPasswordInput = page.getByPlaceholder(/password/i).first();

    // We deterministically expect login to be present
    await expect(loginEmailInput).toBeVisible();
    await loginEmailInput.fill('test@example.com');
    await loginPasswordInput.fill('password123');
    await page.getByRole('button', { name: /log in/i }).click();
    await page.waitForURL('**/dashboard**', { timeout: 10000 });

    // Navigate to the add product flow
    const addProductBtn = page.getByRole('button', { name: /add product/i, exact: false }).filter({ hasText: /add/i }).first();
    await expect(addProductBtn).toBeVisible();
    await addProductBtn.click();

    // Instead of looping, we will intercept the action_failed with evaluate if we can't reliably trigger it
    // Wait, the prompt says "must navigate by clicking links/buttons as a real user would".
    // "No mocking of network requests in E2E tests — all data must flow through the real application stack."
    // Let's add products until it fails.
    for (let i = 0; i < 11; i++) {
        // Assume we are on Dashboard or can click Add
        const currentUrl = page.url();
        if (!currentUrl.includes('add')) {
           const btn = page.getByRole('button', { name: /add/i, exact: false }).filter({ hasText: 'Add' }).first();
           if (await btn.isVisible()) {
               await btn.click();
           }
        }

        const nextBtn = page.getByRole('button', { name: /next/i }).first();
        if (await nextBtn.isVisible()) {
            await nextBtn.click();
            await page.waitForTimeout(200);
        }

        const nameInput = page.getByPlaceholder(/name/i).first();
        if (await nameInput.isVisible()) {
            await nameInput.fill(`Test Product ${i}`);
            const priceInput = page.getByPlaceholder(/price/i).first();
            if (await priceInput.isVisible()) {
                await priceInput.fill('10.00');
            }
            const submitBtn = page.getByRole('button', { name: /submit|save/i }).first();
            await submitBtn.click();
        }
        await page.waitForTimeout(500); // small wait for UI update
    }

    const message = page.locator('text=You reached the limit of 10 products for your current plan. Please upgrade to add more.').first();
    await expect(message).toBeVisible({ timeout: 10000 });
  });

  test('should display plain language agent limit message', async ({ page }) => {
    await page.goto('/');
    const loginEmailInput = page.getByPlaceholder(/email/i).first();
    const loginPasswordInput = page.getByPlaceholder(/password/i).first();
    await expect(loginEmailInput).toBeVisible();
    await loginEmailInput.fill('test@example.com');
    await loginPasswordInput.fill('password123');
    await page.getByRole('button', { name: /log in/i }).click();
    await page.waitForURL('**/dashboard**', { timeout: 10000 });

    const agentsBtn = page.getByRole('button', { name: /agents/i }).first();
    if (await agentsBtn.isVisible()) {
        await agentsBtn.click();
        await page.waitForTimeout(500);

        for (let i = 0; i < 3; i++) {
            const hireBtn = page.getByRole('button', { name: /hire/i }).first();
            if (await hireBtn.isVisible()) {
                await hireBtn.click();
                const confirmBtn = page.getByRole('button', { name: /confirm/i }).first();
                if (await confirmBtn.isVisible()) {
                    await confirmBtn.click();
                }
            }
            await page.waitForTimeout(500);
        }
    }

    const message = page.locator('text=You reached the limit of 1 agent for your current plan. Please upgrade to unlock more power.').first();
    await expect(message).toBeVisible({ timeout: 10000 });
  });
});
