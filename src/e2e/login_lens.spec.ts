<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('login_lens', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'login_lens');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('login_lens');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
