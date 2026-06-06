import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: dashboard_ux', async ({ page, request }) => { await currentAppSmoke(page, request, 'dashboard_ux'); });
