import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: test_e2e_run', async ({ page, request }) => { await currentAppSmoke(page, request, 'test_e2e_run'); });
