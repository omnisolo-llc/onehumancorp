import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: team_chat', async ({ page, request }) => { await currentAppSmoke(page, request, 'team_chat'); });
