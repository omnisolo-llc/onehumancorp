<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('dashboard_nav', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'dashboard_nav');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('dashboard_nav');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
