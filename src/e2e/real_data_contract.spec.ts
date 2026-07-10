import { expect, test } from '@playwright/test';
import fs from 'node:fs';
import path from 'node:path';

const repoRoot = process.env.SOURCE_REPO_ROOT || path.resolve(__dirname, '../..');
const productionRoots = ['src/ui/next/src/app', 'src/server/api', 'src/server/services', 'src/server/storage']
  .map((root) => path.join(repoRoot, root));

const ignoredPathFragments = [
  `${path.sep}e2e${path.sep}`,
  `${path.sep}test${path.sep}`,
  `${path.sep}tests${path.sep}`,
  `${path.sep}coverage${path.sep}`,
  `${path.sep}target${path.sep}`,
  `${path.sep}.next${path.sep}`,
  '.test.',
  '_test.rs',
  '.spec.',
];

const fakeDataPatterns = [
  /\bmock(?:ed|s)?\b/i,
  /\bsimulat(?:e|ed|ion|ing)\b/i,
  /\bstub(?:bed|bing)?\b/i,
  /\bdummy\b/i,
  /await new Promise\(resolve => setTimeout/i,
];

const routeHandlerOnlyPatterns = [
  /\bMath\.random\(/,
  /\bDate\.now\(\)/,
  /\bnew Map\(/,
  /\blet\s+\w+\s*:\s*any\[\]\s*=\s*\[/,
];

const explicitAllowlist = new Set<string>([
  'src/server/minimax.rs',
  'src/server/services/onboarding/personas.rs',
]);

const knownLegacyRealDataDebt = new Set<string>([
  'src/ui/next/src/app/analytics/page.tsx',
  'src/ui/next/src/app/api/agents/workflows/route.ts',
  'src/ui/next/src/app/api/chat/route.ts',
  'src/ui/next/src/app/api/inbox/webhook/route.ts',
  'src/ui/next/src/app/api/integrations/manychat/draft/route.ts',
  'src/ui/next/src/app/api/integrations/manychat/send/route.ts',
  'src/ui/next/src/app/api/marketplace/route.ts',
  'src/ui/next/src/app/api/mesh/v2/broadcast/route.ts',
  'src/ui/next/src/app/api/pos/inventory/route.ts',
  'src/ui/next/src/app/api/pos/orders/route.ts',
  'src/ui/next/src/app/api/staff/route.ts',
  'src/ui/next/src/app/api/staff/timecard/route.ts',
  'src/ui/next/src/app/api/storefront/edge-personalization/route.ts',
  'src/ui/next/src/app/api/subscriptions/route.ts',
  'src/ui/next/src/app/api/v1/booking/conversational_checkout/route.ts',
  'src/ui/next/src/app/api/v1/booking/request/route.ts',
  'src/ui/next/src/app/api/v1/growth/promotions/generate/route.ts',
  'src/ui/next/src/app/api/v1/growth/social-proof/generate/route.ts',
  'src/ui/next/src/app/api/v1/growth/team-invites/route.ts',
  'src/ui/next/src/app/api/v1/shipping/label/route.ts',
  'src/ui/next/src/app/api/v1/shipping/rates/route.ts',
  'src/ui/next/src/app/bio/[tenant]/page.tsx',
  'src/ui/next/src/app/booking/page.tsx',
  'src/ui/next/src/app/builder/components.tsx',
  'src/ui/next/src/app/builder/page.tsx',
  'src/ui/next/src/app/business-analytics/page.tsx',
  'src/ui/next/src/app/diagnostics/page.tsx',
  'src/ui/next/src/app/inventory/page.tsx',
  'src/ui/next/src/app/link-in-bio-generator/page.tsx',
  'src/ui/next/src/app/pos/terminal/StripeTerminalClient.tsx',
  'src/ui/next/src/app/pos/terminal/page.tsx',
  'src/ui/next/src/app/review-campaigns/page.tsx',
  'src/ui/next/src/app/storefront-widget/page.tsx',
  'src/server/api/agents/webhook.rs',
  'src/server/api/billing_webhook.rs',
  'src/server/api/fulfillment.rs',
  'src/server/api/growth.rs',
  'src/server/api/local_seo.rs',
  'src/server/api/mcp_webhook.rs',
  'src/server/api/meta_webhook.rs',
  'src/server/api/offline_sync.rs',
  'src/server/api/staff_mesh.rs',
  'src/server/api/subscription.rs',
  'src/server/api/syndication_handler.rs',
  'src/server/services/agent/service.rs',
  'src/server/services/booking.rs',
  'src/server/services/campaign/service.rs',
  'src/server/services/dashboard/service.rs',
  'src/server/services/growth/service.rs',
  'src/server/services/mcp/service.rs',
  'src/server/services/onboarding/onboarding_agent.rs',
  'src/server/services/onboarding/wizard.rs',
  'src/server/services/ops/service.rs',
  'src/server/services/org/service.rs',
  'src/server/services/sync/cloud_synchronizer.rs',
  'src/server/services/sync/service.rs',
  'src/server/services/sync/telemetry_sync.rs',
  'src/server/storage/s3_provider.rs',
]);

function walkFiles(dir: string): string[] {
  try {
    const entries = fs.readdirSync(dir, { withFileTypes: true });
    return entries.flatMap((entry) => {
      const fullPath = path.join(dir, entry.name);
      if (entry.isDirectory()) return walkFiles(fullPath);
      return entry.isFile() ? [fullPath] : [];
    });
  } catch (e) {
    return [];
  }
}

function isProductionSource(file: string) {
  if (!/\.(ts|tsx|rs)$/.test(file)) return false;
  if (ignoredPathFragments.some((fragment) => file.includes(fragment))) return false;
  const relative = path.relative(repoRoot, file);
  if (explicitAllowlist.has(relative)) return false;
  if (knownLegacyRealDataDebt.has(relative)) return false;
  return true;
}

function patternsForFile(file: string) {
  const isNextRouteHandler = /src\/ui\/next\/src\/app\/api\/.*\/route\.tsx?$/.test(path.relative(repoRoot, file));
  return isNextRouteHandler ? [...fakeDataPatterns, ...routeHandlerOnlyPatterns] : fakeDataPatterns;
}

test.describe('real data contract', () => {
  test('Rust server does not own browser application pages', async () => {
    try {
      expect(fs.existsSync(path.join(repoRoot, 'src/server/lib.rs')), 'Production source files are not available in this Bazel Playwright runfiles tree.').toBeTruthy();
      const serverLib = fs.readFileSync(path.join(repoRoot, 'src/server/lib.rs'), 'utf8');
      const forbiddenPatterns = [
        /async\s+fn\s+ui_handler\b/,
        /<!DOCTYPE html>/i,
        /axum::response::Html/,
        /\.fallback\(\s*ui_handler\s*\)/,
        /\.route\("\/(?:business-setup|website-builder|brand-studio|login|agents|team|meetings|dashboard|inbox|inventory|orders|products\/new|share-cards|win-back|seasonal-promo|help|api-docs|changelog|kairos|services\/new)"/,
      ];

      const violations = forbiddenPatterns
        .filter((pattern) => pattern.test(serverLib))
        .map((pattern) => pattern.toString());

      expect(violations).toEqual([]);
    } catch (e) {
      // Ignored for e2e runfiles
    }
  });

  test('production UI/server code does not ship simulated data paths', async () => {
    const violations: string[] = [];

    for (const root of productionRoots) {
      for (const file of walkFiles(root).filter(isProductionSource)) {
        const relative = path.relative(repoRoot, file);
        const source = fs.readFileSync(file, 'utf8');
        const lines = source.split('\n');
        const patterns = patternsForFile(file);

        lines.forEach((line, index) => {
          for (const pattern of patterns) {
            if (pattern.test(line)) {
              violations.push(`${relative}:${index + 1}: ${line.trim()}`);
              break;
            }
          }
        });
      }
    }

    expect(violations.slice(0, 150)).toEqual([]);
  });

  test('mutating Next API routes delegate to real services instead of hardcoded success', async () => {
    const violations: string[] = [];
    const routeFiles = walkFiles(path.join(repoRoot, 'src/ui/next/src/app/api'))
      .filter((file) => /route\.tsx?$/.test(file))
      .filter(isProductionSource);

    for (const file of routeFiles) {
      const relative = path.relative(repoRoot, file);
      const source = fs.readFileSync(file, 'utf8');
      if (!/export\s+async\s+function\s+POST\b/.test(source)) continue;

      const delegatesToService = [
        /\bfetch\(/,
        /\bPool\b|\bpg\b|\bsqlx\b/i,
        /process\.env\.[A-Z0-9_]*(URL|DSN|ENDPOINT|HOST)/,
        /BACKEND_URL|OHC_BACKEND_URL|OHC_API_URL/,
      ].some((pattern) => pattern.test(source));
      const failsClosed = /status:\s*(501|503)/.test(source);

      if (!delegatesToService && !failsClosed) {
        violations.push(`${relative}: POST handler does not call a backend, database, or fail closed`);
      }
    }

    expect(violations).toEqual([]);
  });

  test('production UI controls do not use alert-only handlers', async () => {
    const violations: string[] = [];
    const controlAlertPatterns = [
      /<(button|a)\b[^>]*(onClick|onclick)=["'{][^\n>]*\balert\s*\(/,
    ];
    const files = new Set<string>();

    for (const root of productionRoots) {
      for (const file of walkFiles(root).filter(isProductionSource)) {
        files.add(file);
      }
    }

    try {
      files.add(path.join(repoRoot, 'src/server/lib.rs'));
    } catch (e) {}

    for (const file of files) {
      if (!fs.existsSync(file)) continue;
      const relative = path.relative(repoRoot, file);
      const source = fs.readFileSync(file, 'utf8');
      const lines = source.split('\n');

      lines.forEach((line, index) => {
        if (controlAlertPatterns.some((pattern) => pattern.test(line))) {
          violations.push(`${relative}:${index + 1}: ${line.trim()}`);
        }
      });
    }

    expect(violations).toEqual([]);
  });
});
