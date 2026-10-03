import {test} from 'node:test';
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {JSDOM,VirtualConsole} from 'jsdom';
const roots=['src/ui/next/public','src/ui/next/public/ui','src/ui/tauri/src/ui'];
const owner={userId:'member-a',tenantId:'tenant-a / &',expiresAt:Date.now()+600000};
const profile={store_name:'Private member business',bio:'Private saved description',theme:'dark',links:[{id:'1',title:'Owned link',url:'https://example.test/owned'}],remove_branding:false};
const deferred=()=>{let resolve;const promise=new Promise(r=>{resolve=r;});return{promise,resolve};};
const turn=()=>new Promise(r=>setImmediate(r));
async function waitFor(test){const deadline=Date.now()+2500;while(!test()){if(Date.now()>deadline)throw new Error('Expected actual editor condition did not occur');await new Promise(r=>setTimeout(r,15));}}
async function setup(root,options={}){
 const requests=[];const copied=[];const errors=[];const vc=new VirtualConsole();vc.on('jsdomError',e=>errors.push(e.message));
 const dom=new JSDOM(await readFile(new URL(`../${root}/link-in-bio-generator.html`,import.meta.url),'utf8'),{url:'http://127.0.0.1:39151/ui/link-in-bio-generator.html?tenant=forged-query',runScripts:'dangerously',virtualConsole:vc,beforeParse(w){
  w.Headers=Headers;w.localStorage.setItem('tenant','forged-cache');w.localStorage.setItem('token','untrusted-local-token');
  w.fetch=async(url,init)=>{requests.push({url,init});if(url==='/api/v1/auth/session-identity')return options.identity?.()??Response.json(owner);if(init?.method==='POST')return options.save?.()??new Response('',{status:200});return options.read?.()??Response.json(profile);};
  Object.defineProperty(w.navigator,'clipboard',{value:{writeText:async text=>{copied.push(text);if(options.copyDenied)throw new Error('clipboard denied');}}});
 }});
 await turn();return{dom,doc:dom.window.document,requests,copied,errors,close:()=>dom.window.close()};
}
for(const root of roots){
 test(`${root}: waits for verified private identity before enabling editor`,async()=>{
  const pending=deferred();const f=await setup(root,{identity:()=>pending.promise});try{
   assert.equal(f.doc.getElementById('store-name').disabled||f.doc.getElementById('store-name').closest('fieldset')?.disabled,true);
   assert.equal(f.requests.filter(r=>r.url.includes('/growth/')).length,0);
  }finally{f.close();pending.resolve(Response.json(owner));}
 });
 test(`${root}: saves only the verified tenant and reports private persistence`,async()=>{
  const f=await setup(root);try{
   await waitFor(()=>f.doc.getElementById('store-name').value===profile.store_name);
   const input=f.doc.getElementById('store-name');input.value='Owner-entered change';input.dispatchEvent(new f.dom.window.Event('input',{bubbles:true}));
   await waitFor(()=>f.requests.some(r=>r.init?.method==='POST'));await turn();
   const write=f.requests.find(r=>r.init?.method==='POST');assert.equal(JSON.parse(write.init.body).tenant_id,owner.tenantId);assert.equal(JSON.parse(write.init.body).store_name,'Owner-entered change');
   const headers=new Headers(write.init.headers);assert.equal(headers.get('x-ohc-expected-user'),owner.userId);assert.equal(headers.get('x-ohc-expected-tenant'),owner.tenantId);assert.equal(headers.get('authorization'),null);
   assert.match(f.doc.getElementById('private-profile-status').textContent,/Saved private configuration/);assert.equal(f.errors.length,0);
  }finally{f.close();}
 });
 test(`${root}: retired account cannot display a late private read`,async()=>{
  const pending=deferred();const f=await setup(root,{read:()=>pending.promise});try{
   await waitFor(()=>f.requests.some(r=>r.url.includes('/growth/link-in-bio/')));f.dom.window.dispatchEvent(new f.dom.window.Event('omnisolo_auth_changed'));
   pending.resolve(Response.json(profile));await turn();await turn();
   assert.notEqual(f.doc.getElementById('store-name').value,profile.store_name);assert.equal(f.doc.getElementById('preview-title').textContent,'');
   assert.equal(f.doc.getElementById('store-name').disabled||f.doc.getElementById('store-name').closest('fieldset')?.disabled,true);
  }finally{f.close();}
 });
 test(`${root}: clipboard denial never says copied or claims a public link`,async()=>{
  const f=await setup(root,{copyDenied:true});try{
   await waitFor(()=>f.doc.getElementById('store-name').value===profile.store_name);f.doc.getElementById('copy-btn').click();await turn();await turn();
   assert.equal(f.copied[0],`${f.dom.window.location.origin}/bio/${encodeURIComponent(owner.tenantId)}`);assert.notEqual(f.doc.getElementById('copy-btn').textContent,'Copied Link!');
   assert.match(f.doc.getElementById('private-profile-status').textContent,/Copy failed/);assert.match(f.doc.body.textContent,/Public publication is not available/);
  }finally{f.close();}
 });
 test(`${root}: unconfirmed save holds further automatic writes`,async()=>{
  const f=await setup(root,{save:()=>new Response('',{status:503})});try{
   await waitFor(()=>f.doc.getElementById('store-name').value===profile.store_name);const input=f.doc.getElementById('store-name');input.value='Unsaved change';input.dispatchEvent(new f.dom.window.Event('input',{bubbles:true}));
   await waitFor(()=>f.requests.some(r=>r.init?.method==='POST'));await turn();await turn();
   assert.match(f.doc.getElementById('private-profile-status').textContent,/could not be confirmed/);assert.equal(input.disabled||input.closest('fieldset')?.disabled,true);
  }finally{f.close();}
 });
 for(const status of [401,403])test(`${root}: post-preflight ${status} clears private state without waiting for body`,async()=>{
  let bodyReads=0;const f=await setup(root,{save:()=>({status,text:()=>{bodyReads++;return new Promise(()=>{});}})});try{
   await waitFor(()=>f.doc.getElementById('store-name').value===profile.store_name);const input=f.doc.getElementById('store-name');input.value='Private edited name';input.dispatchEvent(new f.dom.window.Event('input',{bubbles:true}));
   await waitFor(()=>f.requests.some(r=>r.init?.method==='POST'));await turn();await turn();
   assert.equal(input.value,'');assert.equal(f.doc.getElementById('bio-text').value,'');assert.equal(f.doc.getElementById('preview-title').textContent,'');assert.equal(f.doc.getElementById('preview-links').children.length,0);assert.equal(f.doc.getElementById('copy-btn').disabled,true);assert.equal(bodyReads,0);
  }finally{f.close();}
 });
 for(const error of ['queued owner does not match the current session','session_identity_changed'])test(`${root}: post-preflight owner denial ${error} retires private data`,async()=>{
  const f=await setup(root,{save:()=>Response.json({error},{status:409})});try{
   await waitFor(()=>f.doc.getElementById('store-name').value===profile.store_name);const input=f.doc.getElementById('store-name');input.value='Private edited name';input.dispatchEvent(new f.dom.window.Event('input',{bubbles:true}));
   await waitFor(()=>f.requests.some(r=>r.init?.method==='POST'));await turn();await turn();
   assert.equal(input.value,'');assert.equal(f.doc.getElementById('preview-title').textContent,'');assert.match(f.doc.getElementById('private-profile-status').textContent,/Your session changed/);
  }finally{f.close();}
 });
 for(const status of [500,409])test(`${root}: ordinary ${status} keeps draft held without owner invalidation`,async()=>{
  const f=await setup(root,{save:()=>Response.json({error:'revision_conflict'},{status})});try{
   await waitFor(()=>f.doc.getElementById('store-name').value===profile.store_name);const input=f.doc.getElementById('store-name');input.value='Private edited name';input.dispatchEvent(new f.dom.window.Event('input',{bubbles:true}));
   await waitFor(()=>f.requests.some(r=>r.init?.method==='POST'));await turn();await turn();
   assert.equal(input.value,'Private edited name');assert.match(f.doc.getElementById('private-profile-status').textContent,/could not be confirmed/);assert.equal(input.disabled||input.closest('fieldset')?.disabled,true);assert.equal(f.requests.filter(r=>r.init?.method==='POST').length,1);
  }finally{f.close();}
 });

}

for(const root of roots){
 test(`${root}: private GET omits JSON body metadata while POST retains it`,async()=>{
  const f=await setup(root);try{
   await waitFor(()=>f.requests.some(r=>r.url.includes('/growth/link-in-bio/')));
   const read=f.requests.find(r=>r.url.includes('/growth/link-in-bio/'));
   const readHeaders=new Headers(read.init.headers);
   assert.equal(readHeaders.get('content-type'),null,'The catch-all proxy must not try to parse an empty GET as JSON');
   assert.equal(readHeaders.get('x-ohc-expected-user'),owner.userId);assert.equal(readHeaders.get('x-ohc-expected-tenant'),owner.tenantId);
   await waitFor(()=>f.doc.getElementById('store-name').value===profile.store_name);
   const input=f.doc.getElementById('store-name');input.value='Explicit private change';input.dispatchEvent(new f.dom.window.Event('input',{bubbles:true}));
   await waitFor(()=>f.requests.some(r=>r.init?.method==='POST'));
   assert.equal(new Headers(f.requests.find(r=>r.init?.method==='POST').init.headers).get('content-type'),'application/json');
  }finally{f.close();}
 });
}
