import {act,render,screen,waitFor} from '@testing-library/react';
import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import Dashboard from './page';
import {TooltipProvider} from '../../components/TooltipRegistry';
const state=vi.hoisted(()=>({sync:vi.fn(),summary:vi.fn(),remove:vi.fn(),actions:vi.fn()}));
vi.mock('../../lib/sync/SyncManager',()=>({SyncManager:{getInstance:()=>({sync:state.sync,getQueueSummary:state.summary})}}));
vi.mock('../utils/offlineQueue',()=>({getActions:state.actions,removeAction:state.remove}));
vi.mock('./UnifiedAgentFeed',()=>({UnifiedAgentFeed:()=>null}));
vi.mock('next/navigation',()=>({useRouter:()=>({push:vi.fn(),replace:vi.fn()}),usePathname:()=>'/dashboard',useSearchParams:()=>new URLSearchParams()}));
beforeEach(()=>{localStorage.clear();vi.clearAllMocks();state.actions.mockResolvedValue([{id:'queued-order',type:'UPDATE_ORDER_STATUS',payload:{order_id:'order-a'}}]);state.summary.mockResolvedValue({pending:1,needsAttention:0,reconciliation:0,legacyHeld:0,storageUnavailable:false});state.sync.mockResolvedValue(undefined);vi.stubGlobal('fetch',vi.fn(async()=>Response.json({})));vi.spyOn(navigator,'onLine','get').mockReturnValue(true);});
afterEach(()=>{vi.unstubAllGlobals();vi.restoreAllMocks();});
it('uses the durable route dispatcher on reconnect instead of sending every action to the offline payment adapter',async()=>{
 render(<TooltipProvider><Dashboard/></TooltipProvider>);
 await waitFor(()=>expect(state.sync).toHaveBeenCalled());
 act(()=>window.dispatchEvent(new Event('online')));await waitFor(()=>expect(state.sync.mock.calls.length).toBeGreaterThan(1));
 expect(vi.mocked(fetch).mock.calls.some(([url])=>String(url)==='/api/v1/sync/offline')).toBe(false);expect(state.remove).not.toHaveBeenCalled();
});
it('reports retained reconciliation and unavailable identity instead of claiming an empty queue',async()=>{
 state.summary.mockResolvedValue({pending:0,needsAttention:0,reconciliation:1,legacyHeld:0,storageUnavailable:false});render(<TooltipProvider><Dashboard/></TooltipProvider>);
 expect(await screen.findByText(/1 action.*attention|1 action.*reconciliation/i)).toBeVisible();
 state.summary.mockRejectedValue(new Error('Verified queue identity unavailable'));act(()=>window.dispatchEvent(new Event('online')));
 expect(await screen.findByText(/queue.*unavailable|queue.*could not be verified/i)).toBeVisible();
});
it.each(['auth','epoch','clear'])('clears prior-owner counts and retires an old dispatcher on %s invalidation',async mode=>{
 state.summary.mockResolvedValue({pending:3,needsAttention:2,reconciliation:1,legacyHeld:0,storageUnavailable:false});let complete!:()=>void;state.sync.mockImplementation(()=>new Promise<void>(resolve=>{complete=resolve;}));
 render(<TooltipProvider><Dashboard/></TooltipProvider>);await screen.findByText('3 Actions Pending Sync');await waitFor(()=>expect(complete).toBeDefined());
 state.summary.mockImplementation(()=>new Promise(()=>{}));act(()=>window.dispatchEvent(mode==='auth'?new Event('omnisolo_auth_changed'):new StorageEvent('storage',{key:mode==='epoch'?'omnisolo_queue_identity_epoch_v2':null})));
 expect(screen.queryByText('3 Actions Pending Sync')).toBeNull();expect(screen.queryByText(/3 actions need attention/)).toBeNull();
 const reads=state.summary.mock.calls.length;await act(async()=>complete());expect(state.summary.mock.calls).toHaveLength(reads);
});
