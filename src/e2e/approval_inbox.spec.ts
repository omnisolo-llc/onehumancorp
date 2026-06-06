import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: approval_inbox', async ({ page, request }) => { await currentAppSmoke(page, request, 'approval_inbox'); });
