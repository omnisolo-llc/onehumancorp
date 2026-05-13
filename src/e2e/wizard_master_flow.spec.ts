
import { test, expect } from '@playwright/test';

test.describe('Real Onboarding Wizard Flow E2E', () => {
    test('should traverse the full setup wizard successfully with progressive disclosure', async ({ page }) => {



        await page.goto('/wizard');
        await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
        await page.locator('button:has-text("Launch My Business")').click();

        await expect(page.locator('text=What kind of business are you building?')).toBeVisible();
        await page.locator('button:has-text("Online Store")').click();
        await page.locator('button:has-text("Next")').click();

        await expect(page.locator('text=What is your business called?')).toBeVisible();
        await page.locator('input[placeholder="e.g. Maya\'s Cakes"]').fill('Cakes');
        await page.locator('button:has-text("Next")').click();

        await expect(page.locator('text=What do you sell?')).toBeVisible();
        await page.locator('label:has-text("Physical products")').click();
        await page.locator('button:has-text("Next")').click();

        await expect(page.locator('text=How do you want to receive payments?')).toBeVisible();
        await page.locator('button:has-text("Online only")').click();
        await page.locator('button:has-text("Next")').click();

        await expect(page.locator('text=Administrator account')).toBeVisible();
        await page.locator('input[placeholder="Name"]').fill('Admin');
        await page.locator('input[placeholder="you@email.com"]').fill('a@b.com');
        await page.locator('input[placeholder="Password"]').fill('pass');
        await page.locator('button:has-text("Next")').click();

        await expect(page.locator('text=Template gallery')).toBeVisible();
        await page.locator('h3:has-text("Modern")').click();
        await page.locator('button:has-text("Next")').click();

        await expect(page.locator('text=Brand colors & logo')).toBeVisible();
        await page.locator('button:has-text("Next")').click();

        await expect(page.locator('text=Add your first product or service')).toBeVisible();
        await page.locator('button:has-text("Next")').click();

        await expect(page.locator('text=Connect a domain')).toBeVisible();
        await page.locator('button:has-text("Use a free OHC subdomain")').click();
        await page.locator('button:has-text("Next")').click();

        await expect(page.locator('text=Manage my AI team')).toBeVisible();
        await page.locator('span:has-text("Customer Support")').first().click();
        await page.locator('button:has-text("Next")').click();

        await expect(page.locator('text=Schedule / frequency')).toBeVisible();
        await page.locator('button:has-text("Next")').click();

        await expect(page.locator('text=Tune your agent')).toBeVisible();
        await page.locator('button:has-text("Next")').click();

        await expect(page.locator('text=Review & Launch')).toBeVisible();
        await expect(page.locator('text=Cakes')).toBeVisible();
        await expect(page.locator('text=Online Store')).toBeVisible();

        // Progressive disclosure test
        await page.locator('text=Advanced Mode').click();
        await expect(page.locator('text=Raw Config JSON (Advanced)')).toBeVisible();
        await expect(page.locator('text=CLI Deploy:')).toBeVisible();

        await page.locator('button:has-text("Launch My Business")').click();
        await expect(page.locator('text=Onboarding Complete!')).toBeVisible();
        await expect(page.locator('text=Dashboard URL:')).toBeVisible();
    });
});
