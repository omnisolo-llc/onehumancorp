import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test echo_ux_friction', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'echo_ux_friction');
});
