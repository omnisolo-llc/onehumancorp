import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test dashboard_mesh_ui', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'dashboard_mesh_ui');
});
