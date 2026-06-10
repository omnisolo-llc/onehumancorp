import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('dashboard_nav smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'dashboard_nav'); });
