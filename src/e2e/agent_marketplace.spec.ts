<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('agent-marketplace', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'agent-marketplace');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('agent-marketplace');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
