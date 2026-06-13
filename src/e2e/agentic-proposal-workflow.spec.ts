import { test, expect } from '@playwright/test';

test.describe('Agentic Proposal Workflow', () => {
  const tenantId = `tenant-${Math.random().toString(36).substring(7)}`;

  test('Owner reviews drafted quote, simulates client acceptance, and verifies project creation', async ({ page, request }) => {
    // Setup tenant and user
    await request.post('http://127.0.0.1:8081/api/onboarding/start', {
      data: {
        organization_id: tenantId,
        business_type: 'Service',
        company_name: 'Nora Agency'
      }
    });

    const quoteId = `quote-${Math.random().toString(36).substring(7)}`;

    // Ensure there's a quote in the database
    // We will do this via the /api/v1/quotes endpoint
    const createRuleRes = await request.post('http://127.0.0.1:8081/api/v1/quotes', {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': 'admin'
      },
      data: {
        customer_id: '00000000-0000-0000-0000-000000000000',
        status: 'DRAFT',
        line_items: [
            { description: 'Website design', unit_price_cents: 500000, quantity: 1, is_optional: false }
        ]
      }
    });
    const quoteData = await createRuleRes.json();
    const createdQuoteId = quoteData.id;

    // Login via UI
    await page.goto('http://127.0.0.1:3000/login');
    await page.fill('input[type="email"]', `${tenantId}@example.com`);
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');

    // Make sure we are on dashboard
    await expect(page).toHaveURL(/.*\/dashboard.*/);

    // Navigate to quote page
    await page.goto(`http://127.0.0.1:18789/api/ui/quote.html?id=${createdQuoteId}&tenant=${tenantId}&mode=customer`);

    // Verify client accept button is visible
    const btnSimulateAccept = page.locator('#btn-simulate-client-accept');
    await expect(btnSimulateAccept).toBeVisible();

    // Setup dialog listener for the alert
    page.on('dialog', dialog => dialog.accept());

    // Click the simulate client accept button
    await btnSimulateAccept.click();

    // Verify redirect to dashboard
    await expect(page).toHaveURL(/.*dashboard\.html.*/, { timeout: 10000 });

    // Verify project appears in the active projects section
    const projectsSection = page.locator('#projects-section');
    await expect(projectsSection).toBeVisible({ timeout: 10000 });

    const projectCard = page.getByTestId(/project-card-/);
    await expect(projectCard).toBeVisible();
    await expect(projectCard).toContainText('Active');
  });
});
