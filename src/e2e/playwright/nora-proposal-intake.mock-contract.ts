import { test, expect } from '@playwright/test';
import { e2eConfig, setupTenantAndUser } from '../playwright.config';

test.describe('Automated Inquiry to Proposal Drafts (Nora)', () => {
  let context: import("@playwright/test").BrowserContext;

  test.beforeEach(async ({ browser }) => {
    context = await setupTenantAndUser(browser);
  });

  test('draft proposal, view in mobile dashboard, and approve', async () => {
    const page = await context.newPage();
    await page.setViewportSize({ width: 375, height: 812 });

    // 1. Simulate the system intercepting an inquiry and generating a proposal
    // We mock the backend creating it by calling the intake endpoint
    const intakeResponse = await page.request.post(`${e2eConfig.baseURL}/api/v1/onboarding/intake`, {
      form: {
        name: "Test Client",
        email: "client@example.com",
        details: "website redesign",
      }
    });

    expect(intakeResponse.ok()).toBeTruthy();

    // 2. Nora opens OmniSolo app and navigates to the Proposals dashboard
    await page.goto(`${e2eConfig.baseURL}/proposals`);
    await page.waitForLoadState('networkidle');

    // 3. Review Dashboard UI at 375px
    await expect(page.locator('text=Proposals').first()).toBeVisible();
    await expect(page.locator('text=Review and approve draft proposals')).toBeVisible();
    await expect(page.locator('text=website redesign')).toBeVisible();
    await expect(page.locator('text=DRAFT')).toBeVisible();

    // 4. Tap the first proposal
    await page.locator('text=website redesign').first().click();
    await page.waitForLoadState('networkidle');

    // 5. Review Proposal Details UI
    await expect(page.locator('text=Review Proposal')).toBeVisible();
    await expect(page.locator('text=Custom Project Scope')).toBeVisible();

    // 6. Tap "Approve & Send"
    const approveButton = page.locator('button:has-text("Approve & Send")');
    await expect(approveButton).toBeVisible();

    page.on('dialog', dialog => dialog.accept());
    await approveButton.click();

    // 7. Verify UI shows ACCEPTED
    await expect(page.locator('text=ACCEPTED').first()).toBeVisible({ timeout: 10000 });
  });
});
