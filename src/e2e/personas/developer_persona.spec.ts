import { test, expect } from '@playwright/test';

test.describe('Developer Persona Interaction', () => {
  test('developer can view referrals and click copy link', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('developer@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).first().click();
    await page.waitForURL('**/dashboard');

    const referralsBtn = page.locator('button:has-text("Referrals")').first();
    await expect(referralsBtn).toBeVisible();
    await referralsBtn.click();

    const copyBtn = page.locator('button:has-text("Copy")').first();
    await expect(copyBtn).toBeVisible();
    await copyBtn.click();

    // Simulate some realistic workflow steps
    await page.waitForTimeout(200);
    const historyBtn = page.locator('button:has-text("📜 View History")');
    await expect(historyBtn).toBeVisible();
    await historyBtn.click();
  });

  test('developer can connect social media successfully', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('developer@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).first().click();
    await page.waitForURL('**/dashboard');

    const growBtn = page.locator('button:has-text("Grow Business")').first();
    await expect(growBtn).toBeVisible();
    await growBtn.click();

    const igBtn = page.locator('button:has-text("Connect Instagram")');
    await expect(igBtn).toBeVisible();
    await igBtn.click();

    await expect(page.locator('text=📸 Connect Instagram').first()).toBeVisible();
  });

  test('developer triggers free tier modal', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('developer@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).first().click();
    await page.waitForURL('**/dashboard');

    const agentsBtn = page.locator('button:has-text("Manage Agents")').first();
    await expect(agentsBtn).toBeVisible();
    await agentsBtn.click();

    const hireBtn = page.locator('button:has-text("Hire Agent")').first();
    await expect(hireBtn).toBeVisible();
    await hireBtn.click();

    await expect(page.locator('text=Scale Up Your Team')).toBeVisible();
  });

  test('developer sees viral storefront link', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('developer@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).first().click();
    await page.waitForURL('**/dashboard');

    await page.goto('/website-builder');
    for (let i = 0; i < 4; i++) {
       const nextBtn = page.getByRole('button', { name: /next|continue/i }).first();
       await expect(nextBtn).toBeVisible();
       await nextBtn.click();
    }

    const footerLink = page.getByText(/Built with OHC.*Start your free business/i).first();
    await expect(footerLink).toBeVisible();
  });
});
