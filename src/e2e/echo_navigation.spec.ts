import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: echo_navigation', async ({ page, request }) => { await currentAppSmoke(page, request, 'echo_navigation'); });
