<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('unified_catalog', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'unified_catalog');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('unified_catalog');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
