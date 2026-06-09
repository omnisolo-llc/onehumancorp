import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test agent_audit_dashboard_extra', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'agent_audit_dashboard_extra');
});
