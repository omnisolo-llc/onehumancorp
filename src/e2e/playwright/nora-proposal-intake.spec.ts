import { test, expect } from '../fixtures';

test.describe('Nora Autonomous Proposal Intake Flow', () => {
  let proposalId: string;
  let tenantId = 'agency-1';
  let customerId = 'cust-1';

  test('Client intake creates proposal automatically', async ({ request, page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
  });
});
