import { test, expect } from '@playwright/test';

test.describe('Echo UX First-Time Tour & Navigation', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('Test 1: Navigates to Dashboard from login', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.click('button:has-text("Login")');
    await page.waitForURL('**/*');

    // Quick Actions hint should be hidden initially
    const hint = page.locator('#quick-actions-hint');
    await expect(hint).toBeHidden();
  });

  test('Test 2: First-Time User Tour hint toggle works', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.click('button:has-text("Login")');
    await page.waitForURL('**/*');

    // Click the ? icon next to Quick Actions
    const questionMarkBtn = page.locator('text="Quick Actions"').locator('..').locator('button:has-text("?")');
    await expect(questionMarkBtn).toBeVisible();
    await questionMarkBtn.click();

    // Hint should now be visible
    const hint = page.locator('#quick-actions-hint');
    await expect(hint).toBeVisible();
    await expect(hint).toContainText('These buttons are shortcuts to your most common daily tasks.');
  });

  test('Test 3: Navigation elements have correct text labels', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.click('button:has-text("Login")');
    await page.waitForURL('**/*');

    // Check navigation labels
    await expect(page.locator('nav').locator('text="Add"')).toBeVisible();
    await expect(page.locator('nav').locator('text="Orders"')).toBeVisible();
    await expect(page.locator('nav').locator('text="Messages"')).toBeVisible();
    await expect(page.locator('nav').locator('text="Analytics"')).toBeVisible();
    await expect(page.locator('nav').locator('text="Share"')).toBeVisible();
  });

  test('Test 4: Navigation Add button functions properly', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.click('button:has-text("Login")');
    await page.waitForURL('**/*');

    let logged = false;
    page.on('console', msg => {
      if (msg.text() === 'action_add_product') logged = true;
    });

    const addBtn = page.locator('nav').locator('text="Add"');
    await expect(addBtn).toBeVisible();
    await addBtn.click();

    await expect.poll(() => logged).toBe(true);
  });

  test('Test 5: Navigation Messages button routes to Inbox', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.click('button:has-text("Login")');
    await page.waitForURL('**/*');

    const msgsBtn = page.locator('nav').locator('text="Messages"');
    await expect(msgsBtn).toBeVisible();
    await msgsBtn.click();

    // In our app, showScreen('inbox-screen') displays inbox
    await expect(page.locator('#inbox-screen')).toBeVisible();
  });
});
