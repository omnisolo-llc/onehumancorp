import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test('smoke', async ({ page, request }) => {
  await test('smoke', async ({ page, request }) => {
  await currentAppSmoke(page, request, page, request, 'agent_audit_dashboard_extra');
});
});
