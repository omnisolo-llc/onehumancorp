import { test, expect } from '@playwright/test';

test.describe('Onboarding Zero-Touch CUJ', () => {

  test('Persona: Business Owner completes zero-touch setup successfully', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    await expect(page.getByRole('heading', { name: /Welcome/i })).toBeVisible({ timeout: 15000 });
    await page.getByRole('link', { name: /Start Onboarding/i }).click();

    await expect(page.getByText('Build Your Business')).toBeVisible();

    const descInput = page.getByPlaceholder(/e.g., I sell custom vegan cakes in Seattle./i);
    await descInput.fill('I sell custom vegan cakes in Seattle under the name Maya Bakery.');

    const generateBtn = page.getByRole('button', { name: /Generate My Business/i });
    await generateBtn.click();

    await expect(page.getByText('Agents at work...')).toBeVisible({ timeout: 5000 });

    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('Business Name')).toBeVisible();
    await expect(page.getByText('Business Type')).toBeVisible();

    await page.getByRole('button', { name: /Approve & Go Live/i }).click();

    await expect(page.getByText("Welcome")).toBeVisible({ timeout: 15000 });
  });

  test('Persona: Business Owner fails validation on short description', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.getByRole('link', { name: /Start Onboarding/i }).click();

    const descInput = page.getByPlaceholder(/e.g., I sell custom vegan cakes in Seattle./i);
    await descInput.fill('No');
    await page.getByRole('button', { name: /Generate My Business/i }).click();

    await expect(page.getByText('Please provide a valid description (at least 5 characters).')).toBeVisible();
  });

});
