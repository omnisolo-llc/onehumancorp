/**
 * E2E Test Utilities
 * Replaces helpers.sh with Node.js/TypeScript implementation
 */

const BASE_URL = 'http://127.0.0.1:18080';
const MAX_RETRY_ATTEMPTS = 60;
const RETRY_DELAY_MS = 1000;

export interface TestResult {
  name: string;
  passed: boolean;
  error?: string;
  duration: number;
}

export class TestRunner {
  private passed = 0;
  private failed = 0;
  private results: TestResult[] = [];

  async runTest(
    name: string,
    testFn: () => Promise<void> | void,
  ): Promise<void> {
    const startTime = Date.now();
    try {
      await Promise.resolve(testFn());
      this.passed++;
      this.results.push({ name, passed: true, duration: Date.now() - startTime });
      console.log(`✓ ${name}`);
    } catch (error) {
      this.failed++;
      const errorMsg = error instanceof Error ? error.message : String(error);
      this.results.push({ 
        name, 
        passed: false, 
        error: errorMsg,
        duration: Date.now() - startTime 
      });
      console.error(`✗ ${name}: ${errorMsg}`);
    }
  }

  printSummary(): void {
    const total = this.passed + this.failed;
    console.log('\n================================');
    console.log('Test Summary:');
    console.log(`  Passed: ${this.passed}`);
    console.log(`  Failed: ${this.failed}`);
    console.log(`  Total:  ${total}`);
    console.log('================================\n');
  }

  hasFailed(): boolean {
    return this.failed > 0;
  }

  getResults(): TestResult[] {
    return this.results;
  }
}

/**
 * HTTP GET request
 */
export async function httpGet(
  endpoint: string,
  expectedStatus: number = 200,
): Promise<string> {
  const url = `${BASE_URL}${endpoint}`;
  const response = await fetch(url, { method: 'GET' });

  if (response.status !== expectedStatus) {
    throw new Error(
      `GET ${endpoint} - expected ${expectedStatus}, got ${response.status}`,
    );
  }

  return response.text();
}

/**
 * HTTP POST request
 */
export async function httpPost(
  endpoint: string,
  data: unknown,
  expectedStatus: number = 200,
): Promise<string> {
  const url = `${BASE_URL}${endpoint}`;
  const response = await fetch(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: typeof data === 'string' ? data : JSON.stringify(data),
  });

  if (response.status !== expectedStatus) {
    throw new Error(
      `POST ${endpoint} - expected ${expectedStatus}, got ${response.status}`,
    );
  }

  return response.text();
}

/**
 * HTTP PUT request
 */
export async function httpPut(
  endpoint: string,
  data: unknown,
  expectedStatus: number = 200,
): Promise<string> {
  const url = `${BASE_URL}${endpoint}`;
  const response = await fetch(url, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: typeof data === 'string' ? data : JSON.stringify(data),
  });

  if (response.status !== expectedStatus) {
    throw new Error(
      `PUT ${endpoint} - expected ${expectedStatus}, got ${response.status}`,
    );
  }

  return response.text();
}

/**
 * HTTP DELETE request
 */
export async function httpDelete(
  endpoint: string,
  expectedStatus: number = 200,
): Promise<void> {
  const url = `${BASE_URL}${endpoint}`;
  const response = await fetch(url, { method: 'DELETE' });

  if (response.status !== expectedStatus) {
    throw new Error(
      `DELETE ${endpoint} - expected ${expectedStatus}, got ${response.status}`,
    );
  }
}

/**
 * Assert JSON field exists and matches expected value
 */
export function assertJsonField(
  json: string,
  field: string,
  expectedValue?: string,
): void {
  let obj: unknown;
  try {
    obj = JSON.parse(json);
  } catch {
    throw new Error(`Invalid JSON: ${json}`);
  }

  // Simple JSON path resolution (supports dot notation)
  const parts = field.split('.');
  let current: unknown = obj;

  for (const part of parts) {
    if (typeof current === 'object' && current !== null && part.startsWith('.')) {
      const key = part.substring(1);
      current = (current as Record<string, unknown>)[key];
    } else if (typeof current === 'object' && current !== null) {
      current = (current as Record<string, unknown>)[part];
    } else {
      throw new Error(`Field ${field} is missing or invalid`);
    }
  }

  if (expectedValue !== undefined && String(current) !== expectedValue) {
    throw new Error(`Expected ${field}=${expectedValue}, got ${current}`);
  }

  if (current === null || current === undefined) {
    throw new Error(`Field ${field} is missing or null`);
  }
}

/**
 * Wait for server to be ready
 */
export async function waitForServer(maxAttempts: number = MAX_RETRY_ATTEMPTS): Promise<void> {
  for (let attempt = 0; attempt < maxAttempts; attempt++) {
    try {
      const response = await fetch(`${BASE_URL}/healthz`);
      if (response.ok) {
        console.log('Server is ready');
        return;
      }
    } catch {
      // Server not ready, retry
    }
    await new Promise(resolve => setTimeout(resolve, RETRY_DELAY_MS));
  }

  throw new Error(`Server failed to become ready after ${maxAttempts} seconds`);
}

/**
 * Sleep for specified milliseconds
 */
export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}
