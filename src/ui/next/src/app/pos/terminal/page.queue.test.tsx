import {act,render,screen,waitFor} from '@testing-library/react';
import {beforeEach,afterEach,expect,it,vi} from 'vitest';
import POSTerminal from './page';
const queue=vi.hoisted(()=>vi.fn<()=>Promise<number>>());
vi.mock('./StripeTerminalClient',()=>({default:()=>null}));
vi.mock('../../../components/LocalizationToggle',()=>({LocalizationToggle:()=>null}));
vi.mock('../../../lib/sync/SyncManager',()=>({SyncManager:{getInstance:()=>({getQueueLength:queue})}}));
beforeEach(()=>{localStorage.clear();queue.mockReset();vi.stubGlobal('fetch',vi.fn(async()=>Response.json([])));});
afterEach(()=>vi.unstubAllGlobals());
it('shows an unavailable queue without leaking an unhandled rejection or claiming sync success',async()=>{
 queue.mockRejectedValue(new Error('Verified queue identity unavailable'));
 render(<POSTerminal/>);
 expect(await screen.findByText(/queue.*unavailable|queue.*could not be verified/i)).toBeVisible();
 expect(screen.queryByText(/All changes synced/i)).toBeNull();
});
it('does not let a stale successful queue read erase a later failure',async()=>{
 let release!:(count:number)=>void;queue.mockImplementationOnce(()=>new Promise(resolve=>{release=resolve;})).mockRejectedValue(new Error('Verified queue identity unavailable'));
 render(<POSTerminal/>);await waitFor(()=>expect(release).toBeDefined());act(()=>window.dispatchEvent(new Event('omnisolo_queue_updated')));
 expect(await screen.findByText(/queue.*unavailable|queue.*could not be verified/i)).toBeVisible();
 await act(async()=>release(0));expect(screen.getByText(/queue.*unavailable|queue.*could not be verified/i)).toBeVisible();
});
it('retires an older queue read when another tab changes the canonical auth epoch',async()=>{
 let release!:(count:number)=>void;queue.mockImplementationOnce(()=>new Promise(resolve=>{release=resolve;})).mockRejectedValue(new Error('Verified queue identity unavailable'));
 render(<POSTerminal/>);await waitFor(()=>expect(release).toBeDefined());
 act(()=>window.dispatchEvent(new StorageEvent('storage',{key:'omnisolo_queue_identity_epoch_v2',newValue:'new-account'})));
 expect(await screen.findByText(/queue.*unavailable|queue.*could not be verified/i)).toBeVisible();
 await act(async()=>release(0));expect(screen.getByText(/queue.*unavailable|queue.*could not be verified/i)).toBeVisible();
});
