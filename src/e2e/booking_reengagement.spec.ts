import { test, expect } from './fixtures';
import { db } from './db_utils';
import { randomUUID } from 'crypto';

test('booking reengagement approval flow', async ({ page, loginAs, adminUser }) => {
  await loginAs(page, adminUser);

  // Create a dormant customer and shared task to simulate the backend worker having already run
  // This avoids us waiting for the queue worker to pick it up during an E2E test.

  const customerId = randomUUID();
  const customerName = 'Reengagement E2E Customer';
  const taskId = randomUUID();

  // 1. Setup the customer in DB
  await db.query(
    'INSERT INTO customers (id, tenant_id, name) VALUES ($1, $2, $3)',
    [customerId, adminUser.tenant_id, customerName]
  );

  // 2. Setup a drafted shared_task for approval
  const draftedMessage = `Hi ${customerName}, I noticed we haven't had a session in a while! Hope everything is going great with your progress. Would you like to jump back in this week? I have some slots available. Here is a quick booking link: [Link]`;
  const description = `AI detected that ${customerName} is a returning customer who hasn't booked in 14 days. This follow-up helps maintain momentum.`;
  const title = `Approve Re-engagement for ${customerName}`;

  await db.query(
    `INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status, proposed_content)
     VALUES ($1, $2, $3, $4, 'PENDING', 'P1', 'LOW', 'PENDING', $5)`,
    [taskId, adminUser.tenant_id, title, description, draftedMessage]
  );

  // Navigate to the agent feed
  await page.goto('/agent-feed');

  // Wait for the card to be visible
  await expect(page.locator(`text=${title}`)).toBeVisible();

  // Check the message draft
  await expect(page.locator('text=haven\'t had a session in a while')).toBeVisible();

  // Check the button is present (assuming there is an approve button)
  const approveButton = page.locator(`button:has-text("Approve")`);

  // Wait for the Approve button to appear inside the card context or generally
  // Note: OHC usually has a specific Action Card layout, so we expect it to be clickable.
  await expect(approveButton.first()).toBeVisible();
  await approveButton.first().click();

  // Depending on exact UI logic, it might transition state or hide.
  // We'll assert the task is eventually removed from the immediate view or shows success.
  await expect(page.locator(`text=${title}`)).not.toBeVisible();
});
