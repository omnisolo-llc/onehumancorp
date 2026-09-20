import type { FullConfig, FullResult, Reporter, Suite, TestCase, TestResult } from '@playwright/test/reporter';
import { mkdir, rename, writeFile } from 'node:fs/promises';
import path from 'node:path';

function testIdentity(test: TestCase): string {
  const titles = [test.title];
  for (let parent: Suite | undefined = test.parent; parent; parent = parent.parent) {
    if (parent.type === 'describe') titles.unshift(parent.title);
  }
  const file = path.relative(process.cwd(), test.location.file).split(path.sep).join('/');
  return JSON.stringify([test.parent.project()?.name, file, ...titles, test.repeatEachIndex]);
}

export default class CoverageReporter implements Reporter {
  private selected: string[] = [];
  private results: { id: string; status: string; retry: number; duration: number }[] = [];
  private index = 0;
  private total = 0;

  onBegin(config: FullConfig, suite: Suite) {
    this.selected = suite.allTests().map(testIdentity);
    this.index = config.shard?.current ?? 0;
    this.total = config.shard?.total ?? 0;
  }

  onTestEnd(test: TestCase, result: TestResult) {
    this.results.push({ id: testIdentity(test), status: result.status, retry: result.retry, duration: result.duration });
  }

  async onEnd(result: FullResult) {
    const phase = process.env.OMNISOLO_CI_REPORT_PHASE;
    if (phase !== 'inventory' && phase !== 'results') throw new Error('Missing CI evidence phase');
    const identity = { sha: process.env.OMNISOLO_CI_SOURCE_SHA, run: process.env.GITHUB_RUN_ID,
      attempt: process.env.GITHUB_RUN_ATTEMPT };
    if (!/^[a-f0-9]{40}$/.test(identity.sha ?? '') || !/^\d+$/.test(identity.run ?? '') ||
      !/^[1-9]\d*$/.test(identity.attempt ?? '')) throw new Error('Invalid CI evidence identity');
    const directory = path.resolve('target/ci-browser');
    await mkdir(directory, { recursive: true });
    const output = path.join(directory, `${phase}-${this.index}.json`);
    await writeFile(`${output}.tmp`, JSON.stringify({ schema: 1, kind: phase, identity,
      complete: result.status === 'passed', index: this.index, total: this.total,
      selected: this.selected, results: this.results }, null, 2) + '\n');
    await rename(`${output}.tmp`, output);
  }
}
