import { createElement } from 'react';
import { renderToPipeableStream } from 'react-dom/server';
import { PassThrough } from 'node:stream';
import { JSDOM } from 'jsdom';
import { expect,it,vi } from 'vitest';
const ready=vi.hoisted(()=>({value:false,promise:Promise.resolve(),release:()=>{}}));
vi.mock('next/navigation',()=>({usePathname:()=>'/inbox',useRouter:()=>({push:vi.fn()}),useSearchParams:()=>{if(!ready.value)throw ready.promise;return new URLSearchParams();}}));
vi.mock('@powersync/react',()=>({useQuery:()=>({data:[]})}));
vi.mock('../../lib/powersync/PowerSyncProvider',()=>({PowerSyncProvider:({children}:{children:unknown})=>children}));
import InboxPage from './page';
import { TooltipProvider } from '@/components/TooltipRegistry';
it('settles the real Inbox shell to one title after its streamed Suspense fallback is replaced',async()=>{
 ready.value=false;ready.promise=new Promise(resolve=>{ready.release=()=>{ready.value=true;resolve();};});
 const chunks:Buffer[]=[];const output=new PassThrough();output.on('data',chunk=>chunks.push(Buffer.from(chunk)));
 const html=await new Promise<string>((resolve,reject)=>{
  output.on('end',()=>resolve(Buffer.concat(chunks).toString('utf8')));
  const stream=renderToPipeableStream(createElement(TooltipProvider,{children:createElement(InboxPage)}),{onShellReady(){stream.pipe(output);setImmediate(ready.release);},onError:reject});
 });
 const document='<!doctype html><html><body>'+html+'</body></html>';
 const before=new JSDOM(document);expect(before.window.document.querySelectorAll('h1.app-title')).toHaveLength(2);expect(before.window.document.querySelectorAll('.app-main')).toHaveLength(2);expect(before.window.document.querySelector('[hidden] h1.app-title')).not.toBeNull();before.window.close();
 const settled=new JSDOM(document,{runScripts:'dangerously',url:'https://workspace.test/inbox'});
 try {expect(settled.window.document.querySelectorAll('h1.app-title')).toHaveLength(1);expect(settled.window.document.querySelectorAll('.app-main')).toHaveLength(1);expect(settled.window.document.querySelector('h1.app-title')?.textContent).toBe('Unified Inbox');expect(settled.window.document.querySelector('[data-testid="inbox-settled"]')).not.toBeNull();}
 finally{settled.window.close();}
});
