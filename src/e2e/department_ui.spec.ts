import { test, expect } from './fixtures';

test.describe('Department UI and Agent Architecture', () => {
  test('should display 7 personas and toggle autonomy levels', async ({ page }) => {
    // Mock the settings GET request
    await page.route('/api/agents/settings', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ settings: { 'The Protector': false } })
      });
    });

    // Mock settings POST request
    await page.route('/api/agents/settings', async (route) => {
      if (route.request().method() === 'POST') {
        const postData = route.request().postDataJSON();
        expect(postData.department).toBe('Legal');
        expect(postData.autoExecute).toBe(true);
        await route.fulfill({ status: 200, body: JSON.stringify({ success: true }) });
      } else {
        await route.continue();
      }
    });

    await page.goto('/team');

    // Verify 7 personas are visible
    await expect(page.locator('text=The Manager')).toBeVisible();
    await expect(page.locator('text=The Promoter')).toBeVisible();
    await expect(page.locator('text=The Salesperson')).toBeVisible();
    await expect(page.locator('text=The Ambassador')).toBeVisible();
    await expect(page.locator('text=The Accountant')).toBeVisible();
    await expect(page.locator('text=The Protector')).toBeVisible();
    await expect(page.locator('text=The Advisor')).toBeVisible();

    // The Protector should have default state 'Draft for Review'
    const protectorCard = page.locator('div', { hasText: 'The Protector' }).first();
    await expect(protectorCard.locator('text=Draft for Review')).toBeVisible();

    // Toggle autonomy level to 'Auto-Execute'
    const [request] = await Promise.all([
      page.waitForRequest(req => req.url().includes('/api/agents/settings') && req.method() === 'POST'),
      protectorCard.getByRole('button', { name: 'Toggle Auto-Execute' }).click()
    ]);

    expect(request.method()).toBe('POST');
    await expect(protectorCard.locator('text=Auto-Execute')).toBeVisible();
    await expect(protectorCard.locator('text=Draft for Review')).not.toBeVisible();
  });

  test('should allow editing and approving a drafted action from the approval queue', async ({ page }) => {
    // Add mock approval API response for 'Legal'
    await page.route('/api/agents/approvals', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          pending_approvals: [
            {
              id: 'test-req-1',
              tenant_id: 'e2e-tenant',
              department: 'Legal',
              description: 'Drafted Privacy Policy Update for EU compliance',
              status: 'pending',
              action_risk: 'High',
              feature_type: 'legal_compliance'
            }
          ]
        })
      });
    });

    await page.goto('/team');

    // Check if Protector card shows pending items
    const protectorCard = page.locator('div', { hasText: 'The Protector' }).first();
    await expect(protectorCard.locator('text=1 item awaiting approval')).toBeVisible();

    // Navigate to approval inbox
    await protectorCard.locator('button', { hasText: 'The Protector' }).click();

    // Inside ApprovalInbox
    await expect(page.getByRole('heading', { name: 'The Protector' })).toBeVisible();
    await expect(page.locator('text=Drafted Privacy Policy Update for EU compliance')).toBeVisible();

    // Verify Glassmorphism Persona Badge
    await expect(page.locator('p.text-xs.font-semibold', { hasText: 'The Protector' })).toBeVisible();

    // Click Edit
    await page.getByRole('button', { name: 'Edit' }).click();

    // Fill text area
    const textarea = page.locator('textarea');
    await expect(textarea).toBeVisible();
    await textarea.fill('Drafted Privacy Policy Update for EU and US compliance');

    // Click Save
    await page.getByRole('button', { name: 'Save' }).click();

    // Verify text updated
    await expect(page.locator('text=Drafted Privacy Policy Update for EU and US compliance')).toBeVisible();

    // Intercept approval POST request
    await page.route('/api/agents/approvals/test-req-1', async (route) => {
      if (route.request().method() === 'POST') {
        const postData = route.request().postDataJSON();
        expect(postData.approved).toBe(true);
        expect(postData.description).toBe('Drafted Privacy Policy Update for EU and US compliance');
        await route.fulfill({ status: 200, body: JSON.stringify({ success: true }) });
      } else {
        await route.continue();
      }
    });

    // Approve the drafted action
    await page.getByRole('button', { name: 'Approve' }).click();

    // Feed should now be empty
    await expect(page.locator('text=All Caught Up!')).toBeVisible();
    await expect(page.locator('text=There are no pending actions requiring your review.')).toBeVisible();
  });
});
