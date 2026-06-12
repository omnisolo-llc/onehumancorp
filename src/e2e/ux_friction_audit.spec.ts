<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('ux_friction_audit', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'ux_friction_audit');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('ux_friction_audit');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
