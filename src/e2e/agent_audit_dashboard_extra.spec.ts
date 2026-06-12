<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('agent_audit_dashboard_extra', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'agent_audit_dashboard_extra');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('agent_audit_dashboard_extra');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
