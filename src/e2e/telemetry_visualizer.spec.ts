<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('telemetry_visualizer', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'telemetry_visualizer');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('telemetry_visualizer');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
