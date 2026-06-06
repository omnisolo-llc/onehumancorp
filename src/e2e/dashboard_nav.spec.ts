import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: dashboard_nav', async ({ page, request }) => { await currentAppSmoke(page, request, 'dashboard_nav'); });
