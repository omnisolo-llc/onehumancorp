import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test.describe("Smoke", () => { currentAppSmoke('one_tap_referral'); });
