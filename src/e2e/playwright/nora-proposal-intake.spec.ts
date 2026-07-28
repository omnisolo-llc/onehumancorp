import { test, expect } from '@playwright/test';
import { setupTestEnv, teardownTestEnv, loginAsE2eTenant } from '../db_utils';

test.describe('Nora Autonomous Proposal Intake Flow', () => {
  let proposalId: string;
  let tenantId = 'agency-1';
  let customerId = 'cust-1';

  test('Client intake creates proposal automatically', async ({ request, page }) => {

  });
});
