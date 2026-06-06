import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: success_milestones', async ({ page, request }) => { await currentAppSmoke(page, request, 'success_milestones'); });
