<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('referrals', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'referrals');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('referrals');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
