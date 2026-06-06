import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: department_tasks', async ({ page, request }) => { await currentAppSmoke(page, request, 'department_tasks'); });
