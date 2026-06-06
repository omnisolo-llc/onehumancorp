import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: autonomous_ops', async ({ page, request }) => { await currentAppSmoke(page, request, 'autonomous_ops'); });
