import { test, expect } from '@playwright/test';

test('Order placement triggers Operations and Customer Success AI agents', async ({ page }) => {
    // Navigate to the login page
    await page.goto('/login');

    // Login
    await page.getByPlaceholder('Email or Username').first().fill( 'test@example.com');
    await page.locator('input[type="password"]').first().fill( 'password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for the Dashboard
    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    // Simulate placing an order
    await page.click('button:has-text("Simulate Order")');

    // Check if the dashboard feed or agent activity panel is visible
    // Wait for the feed to load
    await expect(page.locator('text="Agent Activity"')).toBeVisible();

    // Verify state transition output is visible
    await expect(page.locator('text="Operations processed OrderReceived"')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text="Customer Success drafted confirmation"')).toBeVisible({ timeout: 5000 });
});

test('My Team view shows all departments and toggles', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill( 'test@example.com');
    await page.locator('input[type="password"]').first().fill( 'password123');
    await page.locator('button:has-text("Login")').first().click();
    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    await page.click('button:has-text("My Team")');

    await expect(page.locator('text="View the status and activity of your 7 AI Departments."')).toBeVisible();
    await expect(page.locator('text="Operations ("The Manager")"')).toBeVisible();
    await expect(page.locator('text="Marketing & Advertising ("The Promoter")"')).toBeVisible();
    await expect(page.locator('text="Sales & Acquisition ("The Salesperson")"')).toBeVisible();
    await expect(page.locator('text="Customer Success ("The Ambassador")"')).toBeVisible();
    await expect(page.locator('text="Finance & Payments ("The Accountant")"')).toBeVisible();
    await expect(page.locator('text="Legal & Compliance ("The Protector")"')).toBeVisible();
    await expect(page.locator('text="Business Advisory ("The Advisor")"')).toBeVisible();

    // Check toggles
    await page.locator('input[type="checkbox"]').first().click();
});
