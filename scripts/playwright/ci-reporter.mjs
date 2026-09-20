import { mkdirSync, renameSync, writeFileSync } from 'node:fs';
import path from 'node:path';

/** A bounded, crash-visible record of the suite Playwright actually selected. */
export default class CiReporter {
  constructor(options = {}) {
    this.directory = options.directory ?? path.resolve('test-results/ci-coverage');
    this.identity = options.identity ?? JSON.parse(process.env.OMNISOLO_CI_IDENTITY || '{}');
    if (!/^[a-f0-9]{40}$/.test(this.identity.sha)
        || typeof this.identity.runId !== 'string' || !/^[1-9]\d*$/.test(this.identity.runId)
        || !Number.isSafeInteger(this.identity.attempt) || this.identity.attempt < 1) {
      throw new Error('CI reporter requires a complete hosted source/run/attempt identity');
    }
    this.discovery = options.discovery ?? process.env.OMNISOLO_CI_DISCOVERY === '1';
    this.results = new Map();
    this.globalError = false;
  }

  onBegin(config, suite) {
    const shardIndex = config.shard?.current ?? 1;
    const shardTotal = config.shard?.total ?? 1;
    if (!Number.isSafeInteger(shardIndex) || !Number.isSafeInteger(shardTotal)
        || shardIndex < 1 || shardTotal < shardIndex) throw new Error('Invalid CI shard identity');
    const selectedIds = suite.allTests().map(test => test.id);
    this.selected = new Set(selectedIds);
    if (selectedIds.some(id => typeof id !== 'string' || !id)) throw new Error('Invalid test identity');
    if (this.selected.size !== selectedIds.length) throw new Error('Duplicate test identity');
    this.report = { schemaVersion: 1, ...this.identity, shardIndex, shardTotal, selectedIds };
    if (this.discovery) {
      this.report.mode = 'discovery';
      this.filename = shardTotal === 1 ? 'selection-all.json' : `selection-${shardIndex}-of-${shardTotal}.json`;
    } else {
      this.report.complete = false;
      this.report.finished = [];
      this.filename = `execution-${shardIndex}-of-${shardTotal}.json`;
    }
    mkdirSync(this.directory, { recursive: true });
    this.persist();
  }

  onTestEnd(test, result) {
    if (this.discovery) throw new Error('Discovery must not contain execution results');
    if (!this.selected?.has(test.id)) throw new Error('Unexpected test result');
    const previous = this.results.get(test.id);
    this.results.set(test.id, {
      id: test.id,
      // Preserve the first outcome, including an unexpected failure followed by a pass.
      outcome: previous?.outcome ?? result.status,
      attempts: Math.max((previous?.attempts ?? 0) + 1, result.retry + 1),
    });
    this.report.finished = [...this.results.values()];
    this.persist();
  }

  onError() { this.globalError = true; }

  onEnd(result) {
    if (!this.report || this.discovery) return;
    this.report.complete = !this.globalError
      && !['interrupted', 'timedout'].includes(result.status)
      && this.results.size === this.selected.size;
    this.persist();
  }

  persist() {
    const filename = path.join(this.directory, this.filename);
    const temporary = `${filename}.tmp`;
    writeFileSync(temporary, JSON.stringify(this.report), { mode: 0o600 });
    renameSync(temporary, filename);
  }

  printsToStdio() { return false; }
}
