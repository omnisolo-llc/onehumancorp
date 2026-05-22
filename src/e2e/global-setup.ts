import { chromium, type FullConfig } from '@playwright/test';
import { execFileSync, spawnSync } from 'child_process';
import * as fs from 'fs';
import * as path from 'path';

const ROOT = path.resolve(__dirname, '../..');
const SEED_SQL = path.join(__dirname, 'e2e-seed.sql');
const AUTH_DIR = process.env.E2E_AUTH_DIR
  ?? (process.env.TEST_TMPDIR
    ? path.join(process.env.TEST_TMPDIR, 'playwright-auth')
    : path.join(ROOT, 'test-results', '.auth'));

const USERS = [
  { email: 'test@example.com', password: 'password123', file: 'admin.json' },
  { email: 'member@example.com', password: 'MemberPass123!', file: 'member.json' },
];

async function waitForApp(baseURL: string) {
  for (let attempt = 0; attempt < 60; attempt += 1) {
    try {
      const response = await fetch(new URL('/readyz', baseURL));
      if (response.ok) return;
    } catch {
      // App is still booting.
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
  throw new Error(`App did not become ready at ${baseURL}`);
}

function seedDatabase() {
  const databaseUrl = process.env.DATABASE_URL ?? 'postgres://ohc:ohc@localhost:5432/ohc';
  const composeProject = process.env.E2E_COMPOSE_PROJECT;

  if (composeProject) {
    execFileSync(
      'docker',
      [
        'compose',
        '-p',
        composeProject,
        '-f',
        'deploy/docker-compose.e2e.yml',
        'exec',
        '-T',
        'postgres',
        'psql',
        '-U',
        'ohc',
        '-d',
        'ohc',
        '-v',
        'ON_ERROR_STOP=1',
        '-f',
        '-',
      ],
      {
        cwd: ROOT,
        input: fs.readFileSync(SEED_SQL),
        stdio: ['pipe', 'inherit', 'inherit'],
        env: process.env,
      },
    );
    return;
  }

  const psqlCheck = spawnSync('psql', ['--version'], { stdio: 'ignore' });

  if (psqlCheck.status === 0) {
    execFileSync('psql', [databaseUrl, '-v', 'ON_ERROR_STOP=1', '-f', SEED_SQL], {
      cwd: ROOT,
      stdio: 'inherit',
      env: process.env,
    });
    return;
  }

  if (process.env.E2E_POSTGRES_CONTAINER) {
    execFileSync(
      'docker',
      ['exec', '-i', process.env.E2E_POSTGRES_CONTAINER, 'psql', '-U', 'ohc', '-d', 'ohc', '-v', 'ON_ERROR_STOP=1'],
      {
        cwd: ROOT,
        input: fs.readFileSync(SEED_SQL),
        stdio: ['pipe', 'inherit', 'inherit'],
        env: process.env,
      },
    );
    return;
  }

  execFileSync(
    'docker',
    ['compose', '-f', 'deploy/docker-compose.e2e.yml', 'exec', '-T', 'postgres', 'psql', '-U', 'ohc', '-d', 'ohc', '-v', 'ON_ERROR_STOP=1', '-f', '-'],
    {
      cwd: ROOT,
      input: fs.readFileSync(SEED_SQL),
      stdio: ['pipe', 'inherit', 'inherit'],
      env: process.env,
    },
  );
}

async function loginThroughUi(baseURL: string, user: (typeof USERS)[number]) {
  const browser = await chromium.launch(
    process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE
      ? { executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE }
      : undefined,
  );
  try {
    const page = await browser.newPage({ baseURL });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill(user.email);
    await page.locator('input[type="password"]').first().fill(user.password);
    await page.locator('button:has-text("Login")').first().click();
    await page.getByRole('heading', { name: 'Dashboard' }).waitFor({ state: 'visible', timeout: 15000 });
    await page.context().storageState({ path: path.join(AUTH_DIR, user.file) });
  } finally {
    await browser.close();
  }
}

export default async function globalSetup(config: FullConfig) {
  const baseURL = config.projects[0]?.use?.baseURL as string | undefined;
  if (!baseURL) {
    throw new Error('Playwright baseURL is required for e2e global setup.');
  }

  await waitForApp(baseURL);
  seedDatabase();
  fs.mkdirSync(AUTH_DIR, { recursive: true });

  for (const user of USERS) {
    await loginThroughUi(baseURL, user);
  }
}
