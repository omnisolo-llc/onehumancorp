import { test, expect } from './fixtures';
import { currentAppSmokeAsync } from './current_app_smoke';

test.describe('Viral Cart Recovery Growth Loop', () => {
  test('should display cart recovery widget on dashboard, allow configuration, and generate AI draft', async ({ page, request, loginAs, unlimitedAdminUser }) => {
    // Navigate to dashboard using our e2e fixtures logic to login
    await loginAs(page, unlimitedAdminUser);

    // We are running our standard currentAppSmokeAsync logic as part of the flow to align with other tests.
    // However, the focus of this test is on the Cart Recovery.
    await currentAppSmokeAsync(page, request, 'viral_cart_recovery');

    // Navigate to dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // 1. Verify the widget is visible
    const widgetHeading = page.getByRole('heading', { name: /Cart Recovery Agent/i });
    await expect(widgetHeading).toBeVisible();

    // 2. Click the configure agent link
    const configureLink = page.getByRole('link', { name: /Configure Agent/i });
    await expect(configureLink).toBeVisible();
    await configureLink.click();

    // 3. Verify navigation to the cart recovery page
    await expect(page).toHaveURL(/.*\/cart-recovery.*/);

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // Verify page content
    await expect(page.getByRole('heading', { name: /Cart Recovery Agent/i })).toBeVisible();

    // 4. Fill the configuration form
    const customerInput = page.getByLabel('Customer Name (Optional)');
    await expect(customerInput).toBeVisible();
    await customerInput.fill('John Doe');

    const valueInput = page.getByLabel('Cart Value (Optional)');
    await expect(valueInput).toBeVisible();
    await valueInput.fill('$125');

    // 5. Generate AI draft
    const generateBtn = page.getByRole('button', { name: 'Generate Recovery Email' });
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 6. Verify the drafted email
    // It takes a second for the mock to return
    const draftTextarea = page.locator('textarea').last();
    await expect(draftTextarea).toBeVisible({ timeout: 5000 });
    const draftContent = await draftTextarea.inputValue();

    expect(draftContent).toContain('John Doe');
    expect(draftContent).toContain('$125');
  });
});
