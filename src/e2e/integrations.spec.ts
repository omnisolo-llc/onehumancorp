<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('integrations', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'integrations');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('integrations');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
