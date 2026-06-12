<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('branding_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'branding_loop');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('branding_loop');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
