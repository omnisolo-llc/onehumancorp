import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: login_lens', async ({ page, request }) => { await currentAppSmoke(page, request, 'login_lens'); });
