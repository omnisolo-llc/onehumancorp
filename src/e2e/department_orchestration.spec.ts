import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test department_orchestration', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'department_orchestration');
});
