import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('department_orchestration', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'department_orchestration');
});
