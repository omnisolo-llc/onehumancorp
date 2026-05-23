import { test, expect } from '@playwright/test';

test.describe('Agent Approvals Workflow E2E', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the Team page directly (assuming unauthenticated access is allowed for e2e tests or handled by mock)
    await page.goto('http://localhost:3000/team');
  });

  test('should display the Team page with all departments', async ({ page }) => {
    await expect(page.locator('text=Your Team')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Invisible specialized AI teams')).toBeVisible();

    // Verify all 7 departments are listed
    await expect(page.locator('text=The Manager')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=The Promoter')).toBeVisible();
    await expect(page.locator('text=The Salesperson')).toBeVisible();
    await expect(page.locator('text=The Ambassador')).toBeVisible();
    await expect(page.locator('text=The Accountant')).toBeVisible();
    await expect(page.locator('text=The Protector')).toBeVisible();
    await expect(page.locator('text=The Advisor')).toBeVisible();
  });

  test('should navigate to a department inbox and view pending requests', async ({ page }) => {
    // We'll mock the API response for pending approvals
    await page.route('/api/agents/approvals', async route => {
      const json = {
        pending_approvals: [
          {
            id: 'req-123',
            tenant_id: 'org1',
            department: 'Marketing',
            description: 'Drafted social media post for new product launch.',
            status: 'Pending',
            action_risk: 'High',
            feature_type: 'social_calendar'
          }
        ],
        next_cursor: null
      };
      await route.fulfill({ json });
    });

    await page.goto('http://localhost:3000/team');

    // Wait for the approvals to load and the badge to appear
    const promoterCard = page.locator('button:has-text("The Promoter")');
    await expect(promoterCard.locator('text=1 item awaiting approval')).toBeVisible({ timeout: 10000 });

    // Click on the department
    await promoterCard.click();

    // Verify inbox UI
    await expect(page.locator('text=Approval Inbox')).toBeVisible();
    await expect(page.locator('text=Review drafted actions for The Promoter.')).toBeVisible();

    // Verify the specific approval request is visible
    await expect(page.locator('text=Drafted social media post for new product launch.')).toBeVisible();
    await expect(page.locator('text=7-Day Social Calendar Generated')).toBeVisible(); // specific feature UI
  });

  test('should approve an action request successfully', async ({ page }) => {
    await page.route('/api/agents/approvals', async route => {
      const json = {
        pending_approvals: [
          {
            id: 'req-123',
            tenant_id: 'org1',
            department: 'Marketing',
            description: 'Drafted social media post',
            status: 'Pending',
            action_risk: 'High'
          }
        ],
        next_cursor: null
      };
      await route.fulfill({ json });
    });

    // Mock the POST approval endpoint
    let approveCalled = false;
    await page.route('/api/agents/approvals/req-123', async route => {
      if (route.request().method() === 'POST') {
        const postData = route.request().postDataJSON();
        if (postData && postData.approved === true) {
          approveCalled = true;
          await route.fulfill({ json: { success: true } });
          return;
        }
      }
      await route.fallback();
    });

    await page.goto('http://localhost:3000/team');
    await page.locator('button:has-text("The Promoter")').click();

    // Click Approve
    await page.locator('button:has-text("Approve")').click({ timeout: 10000 });

    // The request should disappear
    await expect(page.locator('text=Drafted social media post')).not.toBeVisible();
    expect(approveCalled).toBeTruthy();
  });

  test('should reject an action request successfully', async ({ page }) => {
    await page.route('/api/agents/approvals', async route => {
      const json = {
        pending_approvals: [
          {
            id: 'req-123',
            tenant_id: 'org1',
            department: 'Marketing',
            description: 'Drafted social media post',
            status: 'Pending',
            action_risk: 'High'
          }
        ],
        next_cursor: null
      };
      await route.fulfill({ json });
    });

    // Mock the POST rejection endpoint
    let rejectCalled = false;
    await page.route('/api/agents/approvals/req-123', async route => {
      if (route.request().method() === 'POST') {
        const postData = route.request().postDataJSON();
        if (postData && postData.approved === false) {
          rejectCalled = true;
          await route.fulfill({ json: { success: true } });
          return;
        }
      }
      await route.fallback();
    });

    await page.goto('http://localhost:3000/team');
    await page.locator('button:has-text("The Promoter")').click();

    // Click Reject
    await page.locator('button:has-text("Reject / Edit")').click({ timeout: 10000 });

    // The request should disappear
    await expect(page.locator('text=Drafted social media post')).not.toBeVisible();
    expect(rejectCalled).toBeTruthy();
  });

  test('should navigate back to the main team page from the inbox', async ({ page }) => {
     await page.route('/api/agents/approvals', async route => {
      const json = {
        pending_approvals: [],
        next_cursor: null
      };
      await route.fulfill({ json });
    });

    await page.goto('http://localhost:3000/team');
    await page.locator('button:has-text("The Promoter")').click();

    // Check we are in inbox
    await expect(page.locator('text=Approval Inbox')).toBeVisible();

    // Click back button (the SVG button)
    await page.locator('button:has(svg)').first().click();

    // Check we are back on Team page
    await expect(page.locator('text=Your Team')).toBeVisible({ timeout: 10000 });
  });
});
