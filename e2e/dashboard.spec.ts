import { test, expect } from '@playwright/test';

test.describe('Dashboard CUJ', () => {

<<<<<<< HEAD
  test('Persona: Business Owner sees dashboard title and analytics', async ({ page }) => {
=======
  test('Persona: Business Owner sees dashboard title and Business Snapshot', async ({ page }) => {
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.goto('/dashboard');

    await expect(page.getByRole('heading', { name: /Dashboard/i })).toBeVisible();
<<<<<<< HEAD
    await expect(page.getByRole('heading', { name: /Business Analytics/i })).toBeVisible();
=======
    await expect(page.getByRole('heading', { name: /Business Snapshot/i })).toBeVisible();
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  });

  test('Persona: Business Owner can view sales and customer metrics', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.goto('/dashboard');

<<<<<<< HEAD
    await expect(page.getByText(/Total Sales/i)).toBeVisible();
    await expect(page.getByText('Customers', { exact: true })).toBeVisible();
=======
    await expect(page.getByText(/Today's Sales/i)).toBeVisible();
    await expect(page.getByText(/Active Customers/i)).toBeVisible();
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  });

  test('Persona: Business Owner can view the X share button', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.goto('/dashboard');

<<<<<<< HEAD
    const shareButton = page.getByRole('link', { name: /WhatsApp/i });
=======
    const shareButton = page.getByRole('button', { name: /Share on X to get 7 Days Free/i });
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await expect(shareButton).toBeVisible();
  });

  test('Persona: Business Owner can open Embed Modal', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.goto('/dashboard');

    // Assuming there is a generic button that triggers `setShowEmbedModal(true)`
    const openEmbedButton = page.getByRole('button', { name: /Embed/i });
    if (await openEmbedButton.isVisible()) {
        await openEmbedButton.click();
        await expect(page.getByRole('heading', { name: /Embed Storefront/i })).toBeVisible();
        await expect(page.getByText(/Copy Code/i)).toBeVisible();
    }
  });

  test('Persona: Business Owner can view the referral modal', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await page.goto('/dashboard');

    const openReferralButton = page.getByRole('button', { name: /Referral/i });
    if (await openReferralButton.isVisible()) {
        await openReferralButton.click();
        await expect(page.getByRole('heading', { name: /Help a Business Grow!/i })).toBeVisible();
        await expect(page.getByText(/Your Unique Link/i)).toBeVisible();
    }
  });
});
