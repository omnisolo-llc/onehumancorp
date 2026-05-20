import { test, expect } from './fixtures';

test('dashboard order milestone is visible after marking an order ready', async ({ page }) => {
  await page.goto('/dashboard');
  await page.getByRole('button', { name: 'Mark Order Ready' }).click();

  await expect(page.locator('#milestone-card')).toBeVisible();
  await expect(page.locator('#milestone-card')).toContainText('First Sale!');
});

test('draft-to-approval flow for AI Agent Departments', async ({ page, request }) => {
  // Wait a bit to ensure the server is ready, just in case
  await page.waitForTimeout(1000);

  // 1. Fetch pending approvals to verify the seeded request exists
  const getPendingRes = await request.get('/api/agents/approvals');
  expect(getPendingRes.ok()).toBeTruthy();
  const getPendingJson = await getPendingRes.json();

  // Find our seeded approval request
  const pendingApproval = getPendingJson.pending_approvals?.find((a: any) => a.id === 'e2e-approval-1');
  expect(pendingApproval).toBeDefined();
  expect(pendingApproval.status).toBe('Pending');
  expect(pendingApproval.department).toBe('CustomerSuccess');

  // 2. Approve the request via the approval endpoint
  const approveRes = await request.post('/api/agents/approvals/e2e-approval-1', {
    data: { approved: true }
  });
  expect(approveRes.ok()).toBeTruthy();
  const approveJson = await approveRes.json();
  expect(approveJson.success).toBe(true);

  // 3. Verify it is no longer pending
  const getAfterRes = await request.get('/api/agents/approvals');
  expect(getAfterRes.ok()).toBeTruthy();
  const getAfterJson = await getAfterRes.json();
  const stillPending = getAfterJson.pending_approvals?.find((a: any) => a.id === 'e2e-approval-1');
  expect(stillPending).toBeUndefined();
});

test('UI: Navigates to team page and displays all AI Agent Departments', async ({ page }) => {
  await page.route('**/api/agents/approvals', async route => {
    await route.fulfill({ json: { pending_approvals: [] } });
  });

  await page.goto('/team');
  await expect(page.locator('h1')).toContainText('Your Team');
  await expect(page.getByText('The Manager')).toBeVisible();
  await expect(page.getByText('The Promoter')).toBeVisible();
  await expect(page.getByText('The Salesperson')).toBeVisible();
  await expect(page.getByText('The Ambassador')).toBeVisible();
  await expect(page.getByText('The Accountant')).toBeVisible();
  await expect(page.getByText('The Protector')).toBeVisible();
  await expect(page.getByText('The Advisor')).toBeVisible();
});

test('UI: Department card shows pending approval and opens ApprovalInbox', async ({ page }) => {
  await page.route('**/api/agents/approvals', async route => {
    await route.fulfill({ json: { pending_approvals: [
      { id: 'mock-1', department: 'CustomerSuccess', description: 'Test request', status: 'Pending', action_risk: 'High' }
    ] } });
  });

  await page.goto('/team');

  const ambassadorCard = page.locator('button', { hasText: 'The Ambassador' });
  await expect(ambassadorCard).toContainText('1 item awaiting approval');

  await ambassadorCard.click();

  await expect(page.locator('h1')).toContainText('The Ambassador');
  await expect(page.getByText('Test request')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Approve' })).toBeVisible();
});

test('UI: Approving a request updates the UI to All Caught Up', async ({ page }) => {
  let postCalled = false;

  await page.route('**/api/agents/approvals', async (route, request) => {
    if (request.method() === 'GET') {
      await route.fulfill({ json: { pending_approvals: [
        { id: 'mock-1', department: 'CustomerSuccess', description: 'Test request', status: 'Pending', action_risk: 'High' }
      ] } });
    } else {
      await route.fallback();
    }
  });

  await page.route('**/api/agents/approvals/mock-1', async route => {
    postCalled = true;
    await route.fulfill({ json: { success: true } });
  });

  await page.goto('/team');
  await page.locator('button', { hasText: 'The Ambassador' }).click();

  await page.getByRole('button', { name: 'Approve' }).click();

  await expect(page.getByText('All Caught Up!')).toBeVisible();
  expect(postCalled).toBe(true);
});

test('UI: Rejecting a request updates the UI to All Caught Up', async ({ page }) => {
  let postCalled = false;

  await page.route('**/api/agents/approvals', async (route, request) => {
    if (request.method() === 'GET') {
      await route.fulfill({ json: { pending_approvals: [
        { id: 'mock-2', department: 'Operations', description: 'Another request', status: 'Pending', action_risk: 'Low' }
      ] } });
    } else {
      await route.fallback();
    }
  });

  await page.route('**/api/agents/approvals/mock-2', async route => {
    postCalled = true;
    await route.fulfill({ json: { success: true } });
  });

  await page.goto('/team');
  await page.locator('button', { hasText: 'The Manager' }).click();

  await page.getByRole('button', { name: 'Reject / Edit' }).click();

  await expect(page.getByText('All Caught Up!')).toBeVisible();
  expect(postCalled).toBe(true);
});

test('UI: Department with no approvals shows All Caught Up directly', async ({ page }) => {
  await page.route('**/api/agents/approvals', async route => {
    await route.fulfill({ json: { pending_approvals: [] } });
  });

  await page.goto('/team');

  await page.locator('button', { hasText: 'The Accountant' }).click();

  await expect(page.locator('h1')).toContainText('The Accountant');
  await expect(page.getByText('All Caught Up!')).toBeVisible();
  await expect(page.getByText('There are no pending actions requiring your review.')).toBeVisible();
});
