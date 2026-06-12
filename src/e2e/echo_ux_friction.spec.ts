<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('echo_ux_friction', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'echo_ux_friction');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('echo_ux_friction');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
