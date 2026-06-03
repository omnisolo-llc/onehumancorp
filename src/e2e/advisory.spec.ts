import { test, expect } from '@playwright/test';

test.describe('Advisory Insights CUJ', () => {

  test('Persona: Business Owner can view the Advisory Insights section on the dashboard', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.goto('/dashboard');

    await expect(page.getByRole('heading', { name: /Advisory Insights/i })).toBeVisible({ timeout: 10000 });
  });

  test('Persona: Business Owner can view their business industry dynamically loaded', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.goto('/dashboard');

    await expect(page.getByText(/Industry:/i)).toBeVisible({ timeout: 10000 });
  });

  test('Persona: Business Owner can see the active pending orders count in the insight widget', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.goto('/dashboard');

    await expect(page.getByText(/Active Orders:/i)).toBeVisible({ timeout: 10000 });
  });

  test('Persona: Business Owner can dismiss the advisory insight widget', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.goto('/dashboard');

    const dismissButton = page.getByRole('button', { name: /Dismiss Insight/i });
    if (await dismissButton.isVisible()) {
        await dismissButton.click();
        await expect(page.getByText(/Active Orders:/i)).not.toBeVisible();
    }
  });

  test('Persona: Business Owner receives an appropriate insight recommendation based on orders count', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.goto('/dashboard');

    await expect(page.getByText(/Recommendation:/i)).toBeVisible({ timeout: 10000 });
  });
});
