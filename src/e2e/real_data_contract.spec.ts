import { expect, test } from '@playwright/test';
import fs from 'node:fs';
import path from 'node:path';

const repoRoot = path.resolve(__dirname, '../..');
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

function walkFiles(dir: string): string[] {
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  return entries.flatMap((entry) => {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) return walkFiles(fullPath);
    return entry.isFile() ? [fullPath] : [];
  });
}

function isProductionSource(file: string) {
  if (!/\.(ts|tsx|rs)$/.test(file)) return false;
  if (ignoredPathFragments.some((fragment) => file.includes(fragment))) return false;
  const relative = path.relative(repoRoot, file);
  if (explicitAllowlist.has(relative)) return false;
  return true;
}

function patternsForFile(file: string) {
  const isNextRouteHandler = /src\/ui\/next\/src\/app\/api\/.*\/route\.tsx?$/.test(path.relative(repoRoot, file));
  return isNextRouteHandler ? [...fakeDataPatterns, ...routeHandlerOnlyPatterns] : fakeDataPatterns;
}

test.describe('real data contract', () => {
  test.skip(
    !fs.existsSync(path.join(repoRoot, 'src/server/lib.rs')),
    'Production source files are not available in this Bazel Playwright runfiles tree.',
  );

  test('Rust server does not own browser application pages', async () => {
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
    files.add(path.join(repoRoot, 'src/server/lib.rs'));

    for (const file of files) {
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
