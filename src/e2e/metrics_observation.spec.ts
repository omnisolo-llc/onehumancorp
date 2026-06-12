<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('metrics_observation', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'metrics_observation');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('metrics_observation');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
