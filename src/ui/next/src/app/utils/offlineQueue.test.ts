import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { enqueueAction, getActions, removeAction } from './offlineQueue';

const dbMock = vi.hoisted(() => ({ execute: vi.fn(), getAll: vi.fn() }));
vi.mock('../../lib/powersync/db', () => ({ getPowerSyncDB: vi.fn(() => dbMock) }));
afterEach(() => vi.unstubAllGlobals());

describe('offlineQueue with PowerSync (SQLite)', () => {

  beforeEach(async () => {
    // Reset mocks before each test
    dbMock.execute.mockReset();
    dbMock.getAll.mockReset();

    // mock window object for offlineQueue window checks
    vi.stubGlobal('window', {});
  });

  it('enqueueAction inserts an action into local_pending_actions table', async () => {
    const action = { id: 'uuid-123', type: 'test_action', payload: { a: 1 }, timestamp: 12345 };

    await enqueueAction(action);

    expect(dbMock.execute).toHaveBeenCalledWith(
      'INSERT OR REPLACE INTO local_pending_actions (id, type, payload, timestamp) VALUES (?, ?, ?, ?)',
      ['uuid-123', 'test_action', '{"a":1}', 12345]
    );
  });

  it('getActions retrieves and parses actions from local_pending_actions table', async () => {
    dbMock.getAll.mockResolvedValue([
      { id: 'uuid-1', type: 'action1', payload: '{"foo":"bar"}', timestamp: 100 },
      { id: 'uuid-2', type: 'action2', payload: '{"baz":"qux"}', timestamp: 200 }
    ]);

    const actions = await getActions();

    expect(dbMock.getAll).toHaveBeenCalledWith('SELECT * FROM local_pending_actions ORDER BY timestamp ASC');
    expect(actions).toEqual([
      { id: 'uuid-1', type: 'action1', payload: { foo: 'bar' }, timestamp: 100 },
      { id: 'uuid-2', type: 'action2', payload: { baz: 'qux' }, timestamp: 200 }
    ]);
  });

  it('removeAction deletes an action by id', async () => {
    await removeAction('uuid-123');

    expect(dbMock.execute).toHaveBeenCalledWith('DELETE FROM local_pending_actions WHERE id = ?', ['uuid-123']);
  });
});
