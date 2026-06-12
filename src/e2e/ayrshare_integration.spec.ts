<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('ayrshare_integration', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'ayrshare_integration');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('ayrshare_integration');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
