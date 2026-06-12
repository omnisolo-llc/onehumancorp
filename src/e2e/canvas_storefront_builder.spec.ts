<<<<<<< HEAD
import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('canvas_storefront_builder', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'canvas_storefront_builder');
});
=======
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('canvas_storefront_builder');
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
