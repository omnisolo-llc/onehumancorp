import { test, expect } from '@playwright/test';

test.describe('Agentic Service Quoting CUJ', () => {
  test('Owner enables Autonomous Quoting and Customer gets instant quote', async ({ browser }) => {
    // 1. Owner Setup in one browser context
    const ownerContext = await browser.newContext();
    const ownerPage = await ownerContext.newPage();

    // Set tenant explicitly for the test
    await ownerPage.goto('/settings/sales');
    await ownerPage.evaluate(() => localStorage.setItem('tenant', 'test-tenant-123'));
    await ownerPage.reload();

    await expect(ownerPage.getByRole('heading', { name: 'Sales & Acquisition' })).toBeVisible();

    // Enable toggle (Wait for settings to potentially load first, though it's quick)
    await ownerPage.waitForTimeout(500);

    // Make sure toggle is checked (click if not checked)
    const toggle = ownerPage.locator('input[type="checkbox"]');
    const isChecked = await toggle.isChecked();
    if (!isChecked) {
      await ownerPage.getByText('Enable Autonomous Quoting').click();
    }

    // Input pricing rules
    await ownerPage.getByPlaceholder('e.g., $50/hr base rate, plus materials. Minimum 2 hours for any job.').fill('$50/hr base, plus materials');

    // Save settings (wait for the response)
    const savePromise = ownerPage.waitForResponse(response => response.url().includes('/api/settings/sales-proxy') && response.request().method() === 'POST');
    await ownerPage.getByRole('button', { name: 'Save' }).click();
    await savePromise;

    await expect(ownerPage.getByRole('button', { name: 'Saved!' })).toBeVisible();
    await ownerContext.close();

    // 2. Customer Request in a clean incognito context to prove we aren't using localStorage
    const customerContext = await browser.newContext();
    const customerPage = await customerContext.newPage();

    // The customer must land on the storefront of the specific tenant
    // Our fake frontend passes localStorage.getItem('tenant') or defaults to 'my-store'.
    // So we need to inject the tenant ID just to route correctly in this mock frontend
    // (In reality, tenant is determined by subdomain/URL).
    await customerPage.goto('/booking');
    await customerPage.evaluate(() => localStorage.setItem('tenant', 'test-tenant-123'));
    await customerPage.reload();

    await expect(customerPage.getByRole('heading', { name: 'Request a Service' })).toBeVisible();

    // Fill the request
    await customerPage.getByPlaceholder('e.g. I have a leaky faucet in the kitchen that needs fixing.').fill('I need help fixing a leaky pipe in my kitchen sink.');

    // Submit
    await customerPage.getByRole('button', { name: 'Get a Quote' }).click();

    // 3. Verify Autonomous Response
    await expect(customerPage.getByRole('heading', { name: 'Quote Generated!' })).toBeVisible({ timeout: 10000 });
    await expect(customerPage.getByText('Estimated $150 based on your request')).toBeVisible();
    await expect(customerPage.getByRole('link', { name: 'Book Time Slot' })).toHaveAttribute('href', 'https://cal.com/ohc-test/30min');

    await customerContext.close();
  });
});
