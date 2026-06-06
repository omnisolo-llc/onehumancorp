import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: milestone_loop', async ({ page, request }) => { await currentAppSmoke(page, request, 'milestone_loop'); });
