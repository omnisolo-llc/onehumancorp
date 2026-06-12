<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('full_journey_e2e', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'full_journey_e2e');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('full_journey_e2e');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
