import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: ux_friction_audit', async ({ page, request }) => { await currentAppSmoke(page, request, 'ux_friction_audit'); });
