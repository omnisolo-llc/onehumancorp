import { expect, test } from './fixtures';

test.describe('Autonomous Client Intake Questionnaire Engine', () => {
  let templateId: string;

  test.beforeAll(async ({ request }) => {
    // 1. Create a questionnaire template via API
    const response = await request.post('/api/v1/intake/templates', {
      headers: {
        'x-spiffe-id': 'spiffe://ohc.app/test',
        'x-org-id': 'my-business',
      },
      data: {
        title: "Web Design Consultation",
        questions: [
          {
            type_name: "multiple_choice",
            text: "What type of website do you need?",
            is_required: true,
            options: ["E-commerce", "Portfolio", "Landing Page"]
          },
          {
            type_name: "text",
            text: "Do you have any specific feature requests?",
            is_required: false,
            options: null
          }
        ]
      }
    });
    expect(response.ok()).toBeTruthy();
    const data = await response.json();
    templateId = data.template_id;
    expect(templateId).toBeDefined();
  });

  test('Customer fills out intake form and generates a draft quote', async ({ browser, adminPage }) => {
    // 2. Customer navigates to the mobile-first form
    const customerContext = await browser.newContext({ viewport: { width: 375, height: 812 } });
    const customerPage = await customerContext.newPage();

    await customerPage.goto(`/intake/${templateId}`);

    // Name step
    await expect(customerPage.locator('h2')).toContainText('What is your name?');
    await customerPage.fill('input[type="text"]', 'Alice Custom');
    await customerPage.click('button:has-text("OK")');

    // Email step
    await expect(customerPage.locator('h2')).toContainText('What is your email address?');
    await customerPage.fill('input[type="email"]', 'alice@example.com');
    await customerPage.click('button:has-text("OK")');

    // Q1: Multiple choice
    await expect(customerPage.locator('h2')).toContainText('What type of website do you need?');
    await customerPage.click('button:has-text("E-commerce")');
    // Multiple choice auto-advances after 300ms

    // Q2: Text
    await expect(customerPage.locator('h2')).toContainText('Do you have any specific feature requests?');
    await customerPage.fill('input[type="text"]', 'Need Stripe integration.');
    await customerPage.click('button:has-text("Submit")');

    // Success screen
    await expect(customerPage.locator('h1')).toContainText('Request Sent!');

    // 3. Merchant reviews the generated quote in the Agent Feed
    await adminPage.goto('/dashboard');

    // Switch to Approvals tab if not default
    const approvalsTab = adminPage.locator('button:has-text("Proposals")').first();
    await expect(approvalsTab).toBeVisible();

    // The AI orchestrator should have created a proposal draft
    const newApprovalCard = adminPage.locator('.glassmorphism').filter({ hasText: 'New Intake: Alice Custom Branding. Proposal Drafted.' }).first();
    await expect(newApprovalCard).toBeVisible({ timeout: 15000 });

    // Verify the simulated AI payload details are visible
    await expect(newApprovalCard).toContainText('Alice Custom wants a logo refresh and 3-page site');
    await expect(newApprovalCard).toContainText('Custom Branding & Web Design');

    // 4. Merchant approves the drafted proposal
    await newApprovalCard.getByTestId('approve-proposal').click();

    // Verify card is removed after approval
    await expect(newApprovalCard).not.toBeVisible();
  });
});
