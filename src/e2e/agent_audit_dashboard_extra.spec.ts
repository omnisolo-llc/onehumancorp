import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('agent_audit_dashboard_extra smoke', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'agent_audit_dashboard_extra');
});
