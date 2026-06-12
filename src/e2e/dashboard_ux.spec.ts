<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('dashboard_ux', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'dashboard_ux');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('dashboard_ux');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
