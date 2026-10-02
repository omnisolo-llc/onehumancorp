import { beforeEach, expect, it, vi } from 'vitest';
const fixture = vi.hoisted(() => {
  const query = vi.fn();
  const release = vi.fn();
  const connect = vi.fn(async () => ({ query, release }));
  const verify = vi.fn(() => 'postgres://owned-fixture');
  return { query, release, connect, verify };
});
vi.mock('../../../../../node_modules/pg/esm/index.mjs', () => ({ Pool: class { connect = fixture.connect; } }));
vi.mock('../../../../../scripts/e2e-fixture-database.mjs', () => ({ verifiedFixtureDatabaseUrl: fixture.verify }));
beforeEach(() => {
  vi.resetModules(); vi.clearAllMocks();
  fixture.verify.mockReturnValue('postgres://owned-fixture');
  fixture.query.mockResolvedValue({ rows: [] });
});

it('commits a verified fixture batch on one connection and returns its observed rows', async () => {
  const { e2eDbTransaction } = await import('../../../../e2e/db_utils');
  fixture.query.mockImplementation(async query => ({ rows: query === 'SELECT fixture' ? [{ id: 'owned' }] : [] }));
  expect(await e2eDbTransaction(query => query('SELECT fixture'))).toEqual([{ id: 'owned' }]);
  expect(fixture.verify).toHaveBeenCalledTimes(1);
  expect(fixture.connect).toHaveBeenCalledTimes(1);
  expect(fixture.query.mock.calls.map(([query]) => query)).toEqual(['BEGIN', 'SELECT fixture', 'COMMIT']);
  expect(fixture.release).toHaveBeenCalledTimes(1);
});

it('rolls back all case data on a rejected statement before releasing the connection', async () => {
  const { e2eDbTransaction } = await import('../../../../e2e/db_utils');
  fixture.query.mockImplementation(async query => {
    if (query === 'INSERT collision') throw new Error('duplicate owned key');
    return { rows: [] };
  });
  await expect(e2eDbTransaction(query => query('INSERT collision'))).rejects.toThrow('duplicate owned key');
  expect(fixture.query.mock.calls.map(([query]) => query)).toEqual(['BEGIN', 'INSERT collision', 'ROLLBACK']);
  expect(fixture.release).toHaveBeenCalledTimes(1);
});

it('never opens a database connection without the disposable runner proof', async () => {
  fixture.verify.mockImplementation(() => { throw new Error('fixture proof rejected'); });
  const { e2eDbTransaction } = await import('../../../../e2e/db_utils');
  await expect(e2eDbTransaction(query => query('INSERT forbidden'))).rejects.toThrow('fixture proof rejected');
  expect(fixture.connect).not.toHaveBeenCalled();
  expect(fixture.query).not.toHaveBeenCalled();
});
