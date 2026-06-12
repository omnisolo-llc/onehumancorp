<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_storefront', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_storefront');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('viral_storefront');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
