import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test.describe("Smoke", () => { currentAppSmoke('virtual_meeting_walkthrough'); });
