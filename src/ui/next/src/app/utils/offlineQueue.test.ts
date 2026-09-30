import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { enqueueAction, enqueueActions, getActions, removeAction } from './offlineQueue';

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

describe('offlineQueue fallback durability', () => {
  beforeEach(() => {
    dbMock.execute.mockRejectedValue(new Error('SQLite unavailable'));
  });

  it('rejects when neither browser storage backend is available', async () => {
    vi.stubGlobal('window', {});
    await expect(enqueueAction({ id: 'absent', type: 'cash_sale', timestamp: 1 })).rejects.toThrow(/storage|IndexedDB/i);
  });

  it('propagates IndexedDB opening errors instead of reporting a queued payment', async () => {
    const failure = new Error('IndexedDB opening denied');
    vi.stubGlobal('window', { indexedDB: { open: () => {
      const request = { error: failure, onerror: null as null | (() => void) };
      queueMicrotask(() => request.onerror?.());
      return request;
    } } });
    await expect(enqueueAction({ id: 'open-failed', type: 'cash_sale', timestamp: 1 })).rejects.toBe(failure);
  });

  it('does not resolve on request success before the IndexedDB transaction commits', async () => {
    const write = { onsuccess: null as null | (() => void), onerror: null };
    const transaction = { oncomplete: null as null | (() => void), onabort: null, onerror: null, objectStore: () => ({ put: vi.fn(() => write) }) };
    vi.stubGlobal('window', { indexedDB: { open: () => {
      const request = { onsuccess: null as null | ((event: unknown) => void) };
      queueMicrotask(() => request.onsuccess?.({ target: { result: { transaction: () => transaction, close: vi.fn() } } }));
      return request;
    } } });
    let settled = false;
    const persisted = enqueueAction({ id: 'commit', type: 'cash_sale', timestamp: 1 }).then(() => { settled = true; });
    await vi.waitFor(() => expect(write.onsuccess || transaction.oncomplete).not.toBeNull());
    write.onsuccess?.();
    await new Promise(resolve => setTimeout(resolve, 0));
    expect(settled).toBe(false);
    transaction.oncomplete?.();
    await persisted;
    expect(settled).toBe(true);
  });

  it('rejects a transaction abort after its write request succeeded', async () => {
    const failure = new Error('Transaction aborted');
    const write = { onsuccess: null as null | (() => void), onerror: null };
    const transaction = { error: failure, oncomplete: null, onabort: null as null | (() => void), onerror: null, objectStore: () => ({ put: () => write }) };
    vi.stubGlobal('window', { indexedDB: { open: () => {
      const request = { onsuccess: null as null | ((event: unknown) => void) };
      queueMicrotask(() => request.onsuccess?.({ target: { result: { transaction: () => transaction, close: vi.fn() } } }));
      return request;
    } } });
    const persisted = enqueueAction({ id: 'aborted', type: 'cash_sale', timestamp: 1 });
    const rejected = expect(persisted).rejects.toBe(failure);
    await vi.waitFor(() => expect(write.onsuccess || transaction.onabort).not.toBeNull());
    write.onsuccess?.();
    transaction.onabort?.();
    await rejected;
  });

  it('rolls back a two-item IndexedDB batch and retries without duplicating its first item', async () => {
    let records = new Map<string, { id: string }>();
    let rejectSecond = true;
    let transactions = 0;
    const database = {
      close: vi.fn(),
      transaction: () => {
        transactions += 1;
        const staged = new Map(records);
        let writes = 0;
        let aborted = false;
        const transaction = {
          error: new Error('Second write failed'),
          oncomplete: null as null | (() => void),
          onabort: null as null | (() => void),
          onerror: null,
          objectStore: () => ({ put: (action: { id: string }) => {
            writes += 1;
            if (rejectSecond && writes === 2) {
              aborted = true;
              queueMicrotask(() => transaction.onabort?.());
            } else {
              staged.set(action.id, action);
              queueMicrotask(() => {
                if (!aborted && writes === 2) {
                  records = staged;
                  transaction.oncomplete?.();
                }
              });
            }
            return {};
          } }),
        };
        return transaction;
      },
    };
    vi.stubGlobal('window', { indexedDB: { open: () => {
      const request = { onsuccess: null as null | ((event: unknown) => void) };
      queueMicrotask(() => request.onsuccess?.({ target: { result: database } }));
      return request;
    } } });
    const actions = [{ id: 'a', type: 'cash_sale', timestamp: 1 }, { id: 'b', type: 'cash_sale', timestamp: 1 }];
    await expect(enqueueActions(actions)).rejects.toThrow('Second write failed');
    expect(records.size).toBe(0);
    expect(transactions).toBe(1);
    rejectSecond = false;
    await enqueueActions(actions);
    expect([...records.keys()]).toEqual(['a', 'b']);
    expect(transactions).toBe(2);
  });
});
