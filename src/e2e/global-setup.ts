import { chromium, type FullConfig } from '@playwright/test';
import { Client } from 'pg';
import * as fs from 'fs';
import * as path from 'path';

export default async function globalSetup(config: FullConfig) {
  const baseURL = config.projects[0]?.use?.baseURL as string | undefined;
  if (!baseURL) {
    throw new Error('Playwright baseURL is required for e2e global setup.');
  }

  const databaseUrl = process.env.DATABASE_URL;
  if (!databaseUrl) {
    throw new Error('DATABASE_URL is missing. Playwright tests must use the Bazel-provided test database.');
  }

  // Seed the database
  const seedSqlPath = path.join(__dirname, 'e2e-seed.sql');
  const seedSql = fs.readFileSync(seedSqlPath, 'utf8');

  const client = new Client({ connectionString: databaseUrl });
  try {
    await client.connect();
    await client.query(seedSql);
  } catch (error) {
    console.error('Failed to seed database:', error);
    throw error;
  } finally {
    await client.end();
  }

  // wait for app to be ready
  for (let attempt = 0; attempt < 60; attempt += 1) {
    try {
      const response = await fetch(new URL('/', baseURL));
      if (response.ok) return;
    } catch {
      // App is still booting.
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
}
