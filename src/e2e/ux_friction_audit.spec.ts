import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('ux_friction_audit smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'ux_friction_audit'); });
