<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('free_tier', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'free_tier');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('free_tier');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
