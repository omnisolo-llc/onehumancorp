import { describe, it, expect, vi, beforeEach } from 'vitest';
import { enqueueAction, getActions, removeAction } from './offlineQueue';
import { getPowerSyncInstance } from '../../lib/powersync/db';

vi.mock('../../lib/powersync/db', () => {
  const db = {
    execute: vi.fn(),
    getAll: vi.fn()
  };
  return {
    getPowerSyncInstance: vi.fn(() => db)
  };
});

describe('offlineQueue with PowerSync (SQLite)', () => {
  let dbMock: any;

  beforeEach(() => {
    // Reset mocks before each test
    dbMock = getPowerSyncInstance();
    dbMock.execute.mockReset();
    dbMock.getAll.mockReset();

    // mock window object for offlineQueue window checks
    global.window = {} as any;
  });

  it('enqueueAction inserts an action into pending_actions table', async () => {
    const action = { id: 'uuid-123', type: 'test_action', payload: { a: 1 }, timestamp: 12345 };

    await enqueueAction(action);

    expect(dbMock.execute).toHaveBeenCalledWith(
      'INSERT OR REPLACE INTO pending_actions (id, type, payload, timestamp) VALUES (?, ?, ?, ?)',
      ['uuid-123', 'test_action', '{"a":1}', 12345]
    );
  });

  it('getActions retrieves and parses actions from pending_actions table', async () => {
    dbMock.getAll.mockResolvedValue([
      { id: 'uuid-1', type: 'action1', payload: '{"foo":"bar"}', timestamp: 100 },
      { id: 'uuid-2', type: 'action2', payload: '{"baz":"qux"}', timestamp: 200 }
    ]);

    const actions = await getActions();

    expect(dbMock.getAll).toHaveBeenCalledWith('SELECT * FROM pending_actions ORDER BY timestamp ASC');
    expect(actions).toEqual([
      { id: 'uuid-1', type: 'action1', payload: { foo: 'bar' }, timestamp: 100 },
      { id: 'uuid-2', type: 'action2', payload: { baz: 'qux' }, timestamp: 200 }
    ]);
  });

  it('removeAction deletes an action by id', async () => {
    await removeAction('uuid-123');

    expect(dbMock.execute).toHaveBeenCalledWith('DELETE FROM pending_actions WHERE id = ?', ['uuid-123']);
  });
});
