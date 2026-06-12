<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('business_setup_2', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'business_setup_2');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('business_setup_2');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
