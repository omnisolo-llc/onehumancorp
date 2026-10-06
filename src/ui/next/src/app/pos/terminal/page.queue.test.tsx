import {act,fireEvent,render,screen,waitFor} from '@testing-library/react';
import {beforeEach,afterEach,expect,it,vi} from 'vitest';
import POSTerminal from './page';
import POSLayout from '../layout';
const queue=vi.hoisted(()=>vi.fn<()=>Promise<number>>());
vi.mock('./StripeTerminalClient',()=>({default:()=>null}));
vi.mock('../../../components/LocalizationToggle',()=>({LocalizationToggle:()=>null}));
vi.mock('../../../lib/sync/SyncManager',()=>({SyncManager:{getInstance:()=>({getQueueLength:queue,getClockQueueSummary:vi.fn().mockResolvedValue({confirmed:0,unconfirmed:0,legacyHeld:0})})}}));
beforeEach(() => {
 const values = new Map<string, string>();
 vi.stubGlobal('localStorage', {
   get length() { return values.size; },
   key: (index: number) => [...values.keys()][index] ?? null,
   getItem: (key: string) => values.get(key) ?? null,
   setItem: (key: string, value: string) => { values.set(key, String(value)); },
   removeItem: (key: string) => { values.delete(key); },
   clear: () => values.clear(),
 });
 queue.mockReset();
 vi.stubGlobal('fetch',vi.fn(async()=>Response.json([])));
});
afterEach(()=>{vi.restoreAllMocks();vi.unstubAllGlobals();});
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

it.each(['getItem','setItem'] as const)('keeps the real terminal locked and its recovery notice visible when device storage %s fails',async method=>{
 queue.mockRejectedValue(new DOMException('Storage blocked','SecurityError'));
 localStorage.setItem('pos_offline_queue','[{"offline_id":"held-record"}]');
 vi.spyOn(localStorage,method).mockImplementation(()=>{throw new DOMException('Storage blocked','SecurityError');});
 expect(()=>render(<POSLayout><POSTerminal/></POSLayout>)).not.toThrow();
 expect(await screen.findByRole('heading',{name:'Terminal Locked'})).toBeVisible();
 expect(screen.getByRole('heading',{name:method==='getItem'?'Historical POS data could not be checked':'Historical POS data needs review'})).toBeVisible();
 for(const digit of ['1','2','3','4']) fireEvent.click(screen.getByRole('button',{name:digit}));
 expect(screen.getByText('Device storage is unavailable. Enable browser storage before unlocking the terminal.')).toBeVisible();
 expect(screen.getByRole('heading',{name:'Terminal Locked'})).toBeVisible();
 expect(screen.queryByRole('button',{name:/Charge \$/})).toBeNull();
 expect(fetch).not.toHaveBeenCalled();
});
