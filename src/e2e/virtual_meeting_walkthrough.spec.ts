import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: virtual_meeting_walkthrough', async ({ page, request }) => { await currentAppSmoke(page, request, 'virtual_meeting_walkthrough'); });
