import { test, expect } from './fixtures';

test.describe('Autonomous Quoting (Auto-Quote & Book)', () => {
  test('CUJ: Owner enables auto-quote, customer requests, owner sees result on dashboard', async ({ page, request }) => {
    // 1. Owner navigates to Settings -> Sales & Acquisition
    await page.goto('/settings');
    await page.waitForLoadState('networkidle');

    await page.getByRole('link', { name: 'Configure', exact: false }).filter({ hasText: 'Configure' }).click();
    await page.waitForURL('**/settings/sales-acquisition');
    await expect(page.getByRole('heading', { name: 'Sales & Acquisition' })).toBeVisible();

    // 2. Toggles "Autonomous Quoting" ON and inputs pricing rules
    await page.getByText('Enable Autonomous Quoting').click();

    const baseRateInput = page.getByLabel('Base Hourly Rate ($)');
    await baseRateInput.fill('75');

    const rulesInput = page.getByPlaceholder('e.g. Plus materials, $20 travel fee...');
    await rulesInput.fill('Plus materials, $20 travel fee');

    // Save settings
    await page.getByRole('button', { name: 'Save Settings' }).click();
    await expect(page.getByText('Settings saved successfully.')).toBeVisible();

    // 3. Customer visits storefront/booking, fills out service request
    // We simulate the customer by navigating directly to /booking
    await page.goto('/booking');
    await page.waitForLoadState('networkidle');

    await page.getByPlaceholder('e.g. I have a leaky faucet in the kitchen that needs fixing.').fill('I have a broken pipe in the kitchen that needs emergency fixing.');

    // 4. Submit
    await page.getByRole('button', { name: 'Get a Quote' }).click();

    // 5. System generates quote and calendar link (success message visible)
    await expect(page.getByRole('heading', { name: 'Request Sent!' })).toBeVisible();
    await expect(page.getByText('We\'ve received your inquiry. We\'ll review it and send over a custom quote and available timeslots shortly.')).toBeVisible();

    // Wait a moment for backend to settle DB writes
    await page.waitForTimeout(2000);

    // 6. Owner sees newly booked job in their dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // The activity feed should show the automatically quoted action
    await expect(page.getByText('Automatically quoted 150 for service request: \'I have a broken pipe in the ki\'')).toBeVisible();
  });
});
