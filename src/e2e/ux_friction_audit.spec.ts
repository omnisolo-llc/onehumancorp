import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('ux_friction_audit', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'ux_friction_audit');
});
