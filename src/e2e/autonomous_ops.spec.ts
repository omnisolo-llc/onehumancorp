<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('autonomous_ops', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'autonomous_ops');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('autonomous_ops');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
