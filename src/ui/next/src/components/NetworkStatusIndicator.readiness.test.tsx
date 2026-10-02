import {act,render,screen,waitFor} from '@testing-library/react';
import {afterEach,beforeEach,expect,it,vi} from 'vitest';
import {NetworkStatusIndicator} from './NetworkStatusIndicator';
import {readQueueOwner,notifyQueueIdentityChange} from '../lib/sync/queueIdentity';
const summary=vi.hoisted(()=>vi.fn());
vi.mock('../lib/sync/SyncManager',()=>({SyncManager:{getInstance:()=>({getQueueSummary:summary})}}));
vi.mock('./TooltipRegistry',()=>({WithTooltip:({children}:React.PropsWithChildren)=><>{children}</>}));
const empty={pending:0,needsAttention:0,reconciliation:0,legacyHeld:0,storageUnavailable:false};
beforeEach(()=>{localStorage.clear();notifyQueueIdentityChange();vi.spyOn(navigator,'onLine','get').mockReturnValue(true);vi.stubGlobal('fetch',vi.fn(async()=>Response.json({userId:'a',tenantId:'t',expiresAt:Date.now()+60_000})));});
afterEach(()=>{vi.restoreAllMocks();vi.unstubAllGlobals();});
it('reports readiness only after verified identity and usable owner-scoped storage',async()=>{
 let storageReady!:()=>void;summary.mockImplementation(async()=>{await readQueueOwner();await new Promise<void>(resolve=>{storageReady=resolve;});return empty;});
 render(<NetworkStatusIndicator/>);await waitFor(()=>expect(storageReady).toBeDefined());expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','held');
 await act(async()=>storageReady());expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','ready');
});
it('withdraws readiness during revalidation and never supplies an uncertain offline owner',async()=>{
 summary.mockImplementation(async()=>{await readQueueOwner();return empty;});render(<NetworkStatusIndicator/>);
 await waitFor(()=>expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','ready'));
 let reply!:(r:Response)=>void;vi.mocked(fetch).mockImplementation(()=>new Promise(resolve=>{reply=resolve;}));
 let verification!:Promise<unknown>;act(()=>{verification=readQueueOwner().catch(error=>error);});
 expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','held');vi.spyOn(navigator,'onLine','get').mockReturnValue(false);
 await expect(readQueueOwner()).rejects.toThrow('identity');await act(async()=>reply(new Response('{}',{status:401})));await verification;
 expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','held');
});
it('withdraws readiness on canonical logout even before another queue read finishes',async()=>{
 summary.mockImplementation(async()=>{await readQueueOwner();return empty;});render(<NetworkStatusIndicator/>);
 await waitFor(()=>expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','ready'));
 summary.mockImplementation(()=>new Promise(()=>{}));act(()=>notifyQueueIdentityChange());expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','held');
});
it('keeps an unavailable secondary adapter visible while exposing readiness of the pinned active queue',async()=>{
 summary.mockImplementation(async()=>{await readQueueOwner();return {...empty,storageUnavailable:true};});render(<NetworkStatusIndicator/>);
 await screen.findByText(/Queue status unavailable/);expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','ready');
});
it('withdraws readiness when the verified session expires without a network event',async()=>{
 vi.useFakeTimers();
 try {
  vi.mocked(fetch).mockImplementation(async()=>Response.json({userId:'a',tenantId:'t',expiresAt:Date.now()+1000}));summary.mockImplementation(async()=>{await readQueueOwner();return empty;});
  await act(async()=>{render(<NetworkStatusIndicator/>);});expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','ready');
  await act(async()=>{vi.advanceTimersByTime(1001);});expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','held');
 }finally{vi.useRealTimers();}
});
it('waits for all concurrent identity checks before announcing offline readiness',async()=>{
 summary.mockImplementation(async()=>{await readQueueOwner();return empty;});render(<NetworkStatusIndicator/>);
 await waitFor(()=>expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','ready'));
 const replies:Array<(r:Response)=>void>=[];vi.mocked(fetch).mockImplementation(()=>new Promise(resolve=>{replies.push(resolve);}));
 let first!:Promise<unknown>;let second!:Promise<unknown>;act(()=>{first=readQueueOwner();second=readQueueOwner();});
 await act(async()=>{replies[0](Response.json({userId:'a',tenantId:'t',expiresAt:Date.now()+60_000}));await first;});
 expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','held');
 await act(async()=>{replies[1](Response.json({userId:'a',tenantId:'t',expiresAt:Date.now()+60_000}));await second;});
 expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','ready');
});
it.each(['auth','epoch','clear'])('clears prior-owner queue counts while replacement verification stalls after %s invalidation',async mode=>{
 summary.mockImplementation(async()=>{await readQueueOwner();return {...empty,pending:3,needsAttention:2,reconciliation:1};});render(<NetworkStatusIndicator/>);await screen.findByText(/Pending: 3/);
 let finishOld!:(value:typeof empty)=>void;summary.mockImplementationOnce(()=>new Promise(resolve=>{finishOld=resolve;})).mockImplementation(()=>new Promise(()=>{}));
 act(()=>window.dispatchEvent(new Event('omnisolo_queue_updated')));await waitFor(()=>expect(finishOld).toBeDefined());
 act(()=>window.dispatchEvent(mode==='auth'?new Event('omnisolo_auth_changed'):new StorageEvent('storage',{key:mode==='epoch'?'omnisolo_queue_identity_epoch_v2':null})));
 expect(screen.queryByText(/Pending: 3/)).toBeNull();expect(screen.queryByText(/Needs attention: 2/)).toBeNull();expect(screen.getByText(/Queue status unavailable/)).toBeVisible();
 await act(async()=>finishOld({...empty,pending:7}));expect(screen.queryByText(/Pending: 7/)).toBeNull();expect(screen.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state','held');
});
