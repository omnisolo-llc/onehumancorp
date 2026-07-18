import { test, expect } from '../../../../e2e/fixtures';

test.describe('Autonomous Subscription Box Lifecycle', () => {

  test('Maya creates and manages a monthly cake subscription', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: /Sign in|Login|Log In/i }).first().click();

    await expect(page).toHaveURL('/dashboard');

    await page.goto('/products/new');

    await page.click('button:has-text("Subscription Box")');

    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.click('label:has-text("Take a photo or upload")');
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles('e2e/fixtures/test_img.png');

    await page.waitForSelector('input[value="Vegan Cake"]', { state: 'visible', timeout: 10000 }).catch(() => {});

    await expect(page.locator('select')).toBeVisible();
    await page.selectOption('select', 'monthly');
    // Discount field
    await page.fill('input[type="number"]', '10');

    await page.click('button:has-text("Looks Good")');

    await expect(page.locator('text=Product Published!')).toBeVisible();
    await page.click('text=Return to Dashboard');

    await expect(page).toHaveURL('/dashboard');

    await page.click('h3:has-text("Subscriptions & Fulfillments")');
    await expect(page).toHaveURL('/subscriptions');

    await expect(page.locator('text=Active Plans')).toBeVisible();
    await expect(page.locator('text=Subscribers')).toBeVisible();
    await expect(page.locator('text=Upcoming Fulfillments')).toBeVisible();

    page.on('dialog', dialog => dialog.accept());
    await page.click('button:has-text("Print Labels")');
  });
});
