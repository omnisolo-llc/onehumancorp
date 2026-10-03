import {createOriginLockManager} from './test-support/origin-locks.mjs';
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {JSDOM} from './test-support/offline-dom.mjs';
const paths=['src/ui/tauri/src/ui/setup.html',...['setup.html','ui/setup.html','api/ui/setup.html','api/v1/ui/setup.html'].map(p=>`src/ui/next/public/${p}`)];
const key=(owner,name='legacy-draft')=>'omnisolo_onboarding_owned_v1:'+encodeURIComponent(JSON.stringify([owner.userId,owner.tenantId]))+':'+name;
async function setup(path,options={}){
 const html=await readFile(new URL('../'+path,import.meta.url),'utf8');
 const filePage=options.url?.startsWith('file:');
 const dom=new JSDOM(html,{url:filePage?'https://workspace.example/setup.html':options.url||'https://workspace.example/setup.html',runScripts:'outside-only',pretendToBeVisual:true});
 if(filePage){const storage=dom.window.localStorage;dom.reconfigure({url:options.url});Object.defineProperty(dom.window,'localStorage',{value:storage});}
 Object.defineProperty(dom.window.navigator,'locks',{configurable:true,value:options.locks===false?undefined:options.locks||createOriginLockManager()});
 let owner=options.owner||{userId:'b',tenantId:'tb'};const requests=[];
 dom.window.fetch=async(url,request={})=>{
  requests.push({url,request,owner:{...owner}});
  if(url.endsWith('/session-identity'))return options.identity?options.identity(owner):Response.json({...owner,expiresAt:Date.now()+60_000});
  return options.fetch?options.fetch(url,request,owner):Response.json({});
 };
 dom.window.localStorage.setItem('onboardingState',JSON.stringify({business_name:'Owner A private studio',first_offer:'Owner A private service',step:3}));
 options.prepare?.(dom.window);
 dom.window.HTMLElement.prototype.scrollTo=()=>{};dom.window.HTMLElement.prototype.scrollIntoView=()=>{};
 Object.defineProperty(dom.window.HTMLElement.prototype,'innerText',{get(){return this.textContent;},set(v){this.textContent=v;}});
 dom.window.eval([...dom.window.document.scripts].find(s=>s.textContent.includes('const container =')).textContent);
 await new Promise(r=>setTimeout(r,10));
 return {dom,requests,setOwner:value=>{owner=value;}};
}
for(const path of paths){
 test(`${path} holds unowned drafts without exposing or autosaving their payload`,async()=>{
  const {dom,requests}=await setup(path);try{
   assert.equal(dom.window.document.getElementById('business-name').value,'');
   await new Promise(r=>setTimeout(r,1100));
   assert.equal(requests.some(({request})=>request.method==='POST'&&String(request.body).includes('Owner A')),false);
   assert.match(dom.window.localStorage.getItem('onboardingState'),/Owner A private studio/);
  }finally{dom.window.close();}
 });
 test(`${path} keeps a foreign owned draft held and untouched`,async()=>{
  const a={userId:'a',tenantId:'ta'};const saved=JSON.stringify({business_name:'Foreign owned draft',step:3});
  const {dom}=await setup(path,{prepare(window){window.localStorage.setItem(key(a),saved);}});try{
   assert.equal(dom.window.document.getElementById('business-name').value,'');assert.equal(dom.window.localStorage.getItem(key(a)),saved);
  }finally{dom.window.close();}
 });
 test(`${path} does not invoke network mutations from a file-only unauthenticated transport`,async()=>{
  const {dom,requests}=await setup(path,{url:'file:///setup.html'});try{
   dom.window.document.getElementById('template-selection').value='Modern';dom.window.document.getElementById('finish-btn').click();
   await new Promise(r=>setTimeout(r,20));
   assert.equal(requests.length,0);assert.match(dom.window.document.body.textContent,/authenticated transport.*unavailable/i);
   assert.match(dom.window.localStorage.getItem('onboardingState'),/Owner A private studio/);
  }finally{dom.window.close();}
 });
 test(`${path} clears a visible owned draft on the canonical auth lifecycle`,async()=>{
  const a={userId:'a',tenantId:'ta'};const {dom,setOwner}=await setup(path,{owner:a,prepare(window){window.localStorage.setItem(key(a),JSON.stringify({business_name:'Owned A studio',step:3}));}});try{
   assert.equal(dom.window.document.getElementById('business-name').value,'Owned A studio');
   setOwner({userId:'b',tenantId:'tb'});dom.window.dispatchEvent(new dom.window.Event('omnisolo_auth_changed'));
   await new Promise(r=>setTimeout(r,20));assert.equal(dom.window.document.getElementById('business-name').value,'');
   assert.match(dom.window.localStorage.getItem(key(a)),/Owned A studio/);
  }finally{dom.window.close();}
 });
}

for(const path of paths){
 test(`${path} holds a just-typed local edit across auth change and an older server draft`,async()=>{
  const a={userId:'a',tenantId:'ta'};
  const {dom,setOwner}=await setup(path,{owner:a,fetch:async(_url,_options,owner)=>Response.json({step:3,business_name:owner.userId==='a'?'Older server A':''}),prepare(window){window.localStorage.setItem(key(a),JSON.stringify({business_name:'Older server A',step:3}));}});
  try{
   const input=dom.window.document.getElementById('business-name');input.value='Newest local A edit';input.dispatchEvent(new dom.window.Event('input',{bubbles:true}));
   setOwner({userId:'b',tenantId:'tb'});dom.window.dispatchEvent(new dom.window.Event('omnisolo_auth_changed'));
   await new Promise(r=>setTimeout(r,20));assert.equal(input.value,'');
   assert.match(dom.window.localStorage.getItem(key(a)),/Newest local A edit/);
   setOwner(a);dom.window.dispatchEvent(new dom.window.Event('omnisolo_auth_changed'));await new Promise(r=>setTimeout(r,20));
   assert.equal(input.value,'Newest local A edit');assert.match(dom.window.document.body.textContent,/local edits are pending/i);
  }finally{dom.window.close();}
 });
 test(`${path} never paints delayed success after a failed draft acknowledgement`,async()=>{
  const b={userId:'b',tenantId:'tb'};
  const {dom}=await setup(path,{owner:b,fetch:async(_url,options)=>options.method==='POST'?Response.json({success:false,error:'Disk failure'}):Response.json({}),prepare(window){window.localStorage.setItem(key(b),JSON.stringify({business_name:'B studio',step:3}));}});
  try{
   const button=dom.window.document.querySelector('#step-name .save-draft-btn')||dom.window.document.querySelector('.save-draft-btn');button.click();
   await new Promise(r=>setTimeout(r,180));assert.doesNotMatch(button.textContent,/saved/i);assert.equal(button.textContent,'Error!');
  }finally{dom.window.close();}
 });
}

const waitUntil=async predicate=>{const deadline=Date.now()+1000;while(!predicate()){if(Date.now()>deadline)throw new Error('Expected request not observed');await new Promise(r=>setTimeout(r,0));}};
for(const path of paths){
 test(`${path} ignores a previous owner chat completion without releasing the new owner’s request`,async()=>{
  const a={userId:'a',tenantId:'ta'};let resolveA,resolveB;
  const {dom,setOwner}=await setup(path,{owner:a,fetch:async(url,_options,owner)=>url.endsWith('/chat')?new Promise(done=>{if(owner.userId==='a')resolveA=done;else resolveB=done;}):Response.json({})});
  try{
   const document=dom.window.document;dom.window.goToStep('step-chat');document.getElementById('chat-input').value='A message';document.getElementById('chat-send-btn').click();await waitUntil(()=>resolveA);
   setOwner({userId:'b',tenantId:'tb'});dom.window.dispatchEvent(new dom.window.Event('omnisolo_auth_changed'));await new Promise(r=>setTimeout(r,20));
   dom.window.goToStep('step-chat');document.getElementById('chat-input').value='B message';document.getElementById('chat-send-btn').click();await waitUntil(()=>resolveB);
   resolveA(Response.json({reply:'Private A reply',is_complete:false}));await new Promise(r=>setTimeout(r,20));
   assert.equal(document.getElementById('chat-send-btn').disabled,true);assert.doesNotMatch(document.getElementById('chat-messages').textContent,/Private A reply|Failed to connect/);
   resolveB(Response.json({reply:'B reply',is_complete:false}));await new Promise(r=>setTimeout(r,20));assert.equal(document.getElementById('chat-send-btn').disabled,false);assert.match(document.getElementById('chat-messages').textContent,/B reply/);
  }finally{dom.window.close();}
 });
 test(`${path} keeps a prior owner’s delayed save body from changing the next owner’s save status`,async()=>{
  const a={userId:'a',tenantId:'ta'};const closers=[];
  const {dom,setOwner}=await setup(path,{owner:a,fetch:async(url,options)=>url.endsWith('/draft')&&options.method==='POST'?new Response(new ReadableStream({start(controller){closers.push(()=>controller.close());}})):Response.json({step:3})});
  try{
   const button=dom.window.document.querySelector('.save-draft-btn');button.click();await waitUntil(()=>closers.length===1);
   setOwner({userId:'b',tenantId:'tb'});dom.window.dispatchEvent(new dom.window.Event('omnisolo_auth_changed'));await new Promise(r=>setTimeout(r,20));
   button.click();await waitUntil(()=>closers.length===2);closers[0]();await new Promise(r=>setTimeout(r,20));
   assert.equal(button.disabled,true);assert.equal(button.textContent,'Saving...');
   closers[1]();await new Promise(r=>setTimeout(r,20));assert.equal(button.disabled,false);assert.match(button.textContent,/Saved|pending/);
  }finally{dom.window.close();}
 });
}

for(const path of paths){
 test(`${path} ignores an old identity response after a newer owner has loaded`,async()=>{
  const a={userId:'a',tenantId:'ta'};let release;
  const {dom,setOwner}=await setup(path,{owner:a,identity:async(owner)=>owner.userId==='a'?{ok:true,json:()=>new Promise(done=>{release=done;})}:Response.json({...owner,expiresAt:Date.now()+60_000}),fetch:async()=>Response.json({step:3,business_name:'Current B business'})});
  try{
   await waitUntil(()=>release);setOwner({userId:'b',tenantId:'tb'});dom.window.dispatchEvent(new dom.window.Event('omnisolo_auth_changed'));await new Promise(r=>setTimeout(r,20));
   assert.equal(dom.window.document.getElementById('business-name').value,'Current B business');
   release({...a,expiresAt:Date.now()+60_000});await new Promise(r=>setTimeout(r,20));
   assert.equal(dom.window.document.getElementById('business-name').value,'Current B business');
  }finally{dom.window.close();}
 });
 test(`${path} ignores an old protected-state body failure after an owner switch`,async()=>{
  const a={userId:'a',tenantId:'ta'};let reject;
  const {dom,setOwner}=await setup(path,{owner:a,fetch:async(url,_options,owner)=>url.endsWith('/state')&&owner.userId==='a'?{ok:true,json:()=>new Promise((_done,fail)=>{reject=fail;})}:Response.json({step:3,business_name:owner.userId==='b'?'Current B business':''})});
  try{
   await waitUntil(()=>reject);setOwner({userId:'b',tenantId:'tb'});dom.window.dispatchEvent(new dom.window.Event('omnisolo_auth_changed'));await new Promise(r=>setTimeout(r,20));
   reject(new Error('Private A recovery failed'));await new Promise(r=>setTimeout(r,20));
   assert.doesNotMatch(dom.window.document.getElementById('setup-session-status').textContent,/Private A/);
   assert.equal(dom.window.document.getElementById('business-name').value,'Current B business');
  }finally{dom.window.close();}
 });
 test(`${path} holds malformed owned envelopes without replacing their bytes`,async()=>{
  const b={userId:'b',tenantId:'tb'};const original=JSON.stringify({format:1,state:null,revision:'original',acknowledgedRevision:null});
  const {dom}=await setup(path,{owner:b,prepare(window){window.localStorage.setItem(key(b),original);}});
  try{assert.equal(dom.window.localStorage.getItem(key(b)),original);assert.match(dom.window.document.getElementById('setup-session-status').textContent,/could not be read/i);}finally{dom.window.close();}
 });
}

for(const path of paths){
 test(`${path} serializes manual and debounced saves through delayed acknowledgement bodies`,async()=>{
  const owner={userId:'b',tenantId:'tb'};let release;let remote='';const dispatched=[];
  const {dom}=await setup(path,{owner,fetch:async(url,options)=>{
   if(!options.method)return Response.json({step:3});
   const value=JSON.parse(options.body).business_name;dispatched.push(value);
   if(value==='A')return new Response(new ReadableStream({start(controller){release=()=>{remote='A';controller.close();};}}));
   remote=value;return new Response(null,{status:204});
  }});
  try{
   const input=dom.window.document.getElementById('business-name');input.value='A';dom.window.document.querySelector('.save-draft-btn').click();await waitUntil(()=>release);
   input.value='B';input.dispatchEvent(new dom.window.Event('input',{bubbles:true}));await new Promise(r=>setTimeout(r,1100));
   const beforeRelease=[...dispatched];release();await new Promise(r=>setTimeout(r,30));
   assert.deepEqual(beforeRelease,['A']);assert.equal(remote,'B');
  }finally{dom.window.close();}
 });
}

for(const path of paths){
 test(`${path} coordinates saves from two same-owner documents using one origin lock`,async()=>{
  const owner={userId:'b',tenantId:'tb'};const locks=createOriginLockManager();let release;let remote='';const commits=[];
  const fetch=async(_url,options)=>{
   if(options.method!=='POST')return Response.json({step:3});
   const value=JSON.parse(options.body).business_name;commits.push(value);
   if(value==='A')return new Response(new ReadableStream({start(controller){release=()=>{remote='A';controller.close();};}}));
   remote=value;return new Response(null,{status:204});
  };
  const first=await setup(path,{owner,locks,fetch});let second;
  try{
   first.dom.window.document.getElementById('business-name').value='A';first.dom.window.document.querySelector('.save-draft-btn').click();await waitUntil(()=>release);
   second=await setup(path,{owner,locks,fetch,prepare(window){Object.defineProperty(window,'localStorage',{value:first.dom.window.localStorage});}});
   second.dom.window.document.getElementById('business-name').value='B';second.dom.window.document.querySelector('.save-draft-btn').click();await new Promise(r=>setTimeout(r,20));
   assert.deepEqual(commits,['A']);release();await new Promise(r=>setTimeout(r,30));assert.equal(remote,'B');assert.deepEqual(commits,['A','B']);
  }finally{first.dom.window.close();second?.dom.window.close();}
 });
 test(`${path} holds an unknown save through reload without another mutation`,async()=>{
  const owner={userId:'b',tenantId:'tb'};const first=await setup(path,{owner,fetch:async(_url,options)=>{if(options.method==='POST')throw new Error('Reply lost');return Response.json({step:3});}});let second;
  try{
   first.dom.window.document.getElementById('business-name').value='Held edit';first.dom.window.document.querySelector('.save-draft-btn').click();await new Promise(r=>setTimeout(r,30));
   const fence=first.dom.window.localStorage.getItem(key(owner,'write-fence'));assert.ok(fence);
   second=await setup(path,{owner,prepare(window){Object.defineProperty(window,'localStorage',{value:first.dom.window.localStorage});}});
   assert.equal(second.dom.window.document.getElementById('business-name').value,'Held edit');second.dom.window.document.querySelector('.save-draft-btn').click();await new Promise(r=>setTimeout(r,20));
   assert.equal(second.requests.some(({request})=>request.method==='POST'),false);assert.equal(second.dom.window.localStorage.getItem(key(owner,'write-fence')),fence);assert.match(second.dom.window.document.body.textContent,/reconciliation/);
  }finally{first.dom.window.close();second?.dom.window.close();}
 });
 test(`${path} visibly holds writes when origin locking is unavailable`,async()=>{
  const {dom,requests}=await setup(path,{locks:false});try{
   dom.window.document.getElementById('business-name').value='Local edit';dom.window.document.querySelector('.save-draft-btn').click();await new Promise(r=>setTimeout(r,20));
   assert.equal(requests.some(({request})=>request.method==='POST'),false);assert.match(dom.window.document.body.textContent,/cannot coordinate safe saves/);assert.match(dom.window.localStorage.getItem(key({userId:'b',tenantId:'tb'})),/Local edit/);
  }finally{dom.window.close();}
 });
}

for(const path of paths){
 test(`${path} keeps a newer local row pending after an old tab’s queued no-stamp save`,async()=>{
  const owner={userId:'b',tenantId:'tb'};const locks=createOriginLockManager();let release;let remote='';
  const fetch=async(_url,options)=>{
   if(options.method!=='POST')return Response.json({step:3});
   const value=JSON.parse(options.body).business_name;
   if(value==='B')return new Response(new ReadableStream({start(controller){release=()=>{remote='B';controller.close();};}}));
   remote=value;return new Response(null,{status:204});
  };
  const first=await setup(path,{owner,locks,fetch});let second;
  try{
   const a=first.dom.window.document.getElementById('business-name');a.value='A';a.dispatchEvent(new first.dom.window.Event('input',{bubbles:true}));
   second=await setup(path,{owner,locks,fetch,prepare(window){Object.defineProperty(window,'localStorage',{value:first.dom.window.localStorage});}});
   second.dom.window.document.getElementById('business-name').value='B';second.dom.window.document.querySelector('.save-draft-btn').click();await waitUntil(()=>release);
   await new Promise(r=>setTimeout(r,1100));release();await new Promise(r=>setTimeout(r,30));
   const row=JSON.parse(first.dom.window.localStorage.getItem(key(owner)));assert.equal(remote,'A');assert.equal(row.state.business_name,'B');assert.ok(row.acknowledgedRevision!==row.revision || (row.acknowledgedWriteVersion??null)!==first.dom.window.localStorage.getItem(key(owner,'write-version')));assert.match(first.dom.window.document.getElementById('setup-session-status').textContent,/Local edits are pending/);
   second.dom.window.dispatchEvent(new second.dom.window.StorageEvent('storage',{key:key(owner,'write-version')}));assert.match(second.dom.window.document.getElementById('setup-session-status').textContent,/Local edits are pending/);assert.doesNotMatch(second.dom.window.document.querySelector('.save-draft-btn').textContent,/^Saved!$/);
  }finally{first.dom.window.close();second?.dom.window.close();}
 });
}

for(const path of paths)for(const pending of [false,true]){
 test(`${path} holds a local revision changed during remote hydration (pending=${pending})`,async()=>{
  const owner={userId:'b',tenantId:'tb'};let resolve;
  const {dom}=await setup(path,{owner,prepare(window){window.localStorage.setItem(key(owner),JSON.stringify({format:1,state:{business_name:'Cached A',step:3},revision:'before-read',acknowledgedRevision:pending?null:'before-read'}));},fetch:async(url)=>url.endsWith('/draft')?new Promise(done=>{resolve=done;}):Response.json({})});
  try{
   await waitUntil(()=>resolve);const before=JSON.parse(dom.window.localStorage.getItem(key(owner)));const newer=JSON.stringify({...before,state:{...before.state,business_name:'Newer unsent B'},revision:'after-read',acknowledgedRevision:null});dom.window.localStorage.setItem(key(owner),newer);
   resolve(Response.json({step:3,business_name:'Older remote A'}));await new Promise(r=>setTimeout(r,20));
   assert.equal(dom.window.localStorage.getItem(key(owner)),newer);assert.match(dom.window.document.getElementById('setup-session-status').textContent,/draft changed in another view/i);assert.notEqual(dom.window.document.getElementById('business-name').value,'Cached A');
  }finally{dom.window.close();}
 });
}

for(const path of paths){
 test(`${path} consumes a successful OIDC handoff and invalidates other views once`,async()=>{
  let signals=0;
  const {dom}=await setup(path,{url:'https://workspace.example/setup.html?ohc_auth_complete=1&tab=review#summary',prepare(window){window.localStorage.setItem('omnisolo_queue_identity_epoch_v2','prior-login');window.addEventListener('omnisolo_auth_changed',()=>signals++);}});
  try{
   assert.notEqual(dom.window.localStorage.getItem('omnisolo_queue_identity_epoch_v2'),'prior-login');
   assert.equal(signals,1);
   assert.equal(dom.window.location.pathname+dom.window.location.search+dom.window.location.hash,'/setup.html?tab=review#summary');
  }finally{dom.window.close();}
 });
 test(`${path} ordinary setup hydration does not broadcast an account change`,async()=>{
  let signals=0;
  const {dom}=await setup(path,{prepare(window){window.localStorage.setItem('omnisolo_queue_identity_epoch_v2','same-login');window.addEventListener('omnisolo_auth_changed',()=>signals++);}});
  try{assert.equal(signals,0);assert.equal(dom.window.localStorage.getItem('omnisolo_queue_identity_epoch_v2'),'same-login');}finally{dom.window.close();}
 });
}
