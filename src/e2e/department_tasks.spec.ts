import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('department_tasks smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'department_tasks'); });
