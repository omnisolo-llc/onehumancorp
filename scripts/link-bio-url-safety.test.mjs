import {test} from 'node:test';
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {JSDOM,VirtualConsole} from 'jsdom';
const roots=['src/ui/next/public','src/ui/next/public/ui','src/ui/tauri/src/ui'];
const unsafe=['javascript:alert(1)','JaVaScRiPt:alert(1)','data:text/html,<script>alert(1)</script>','blob:https://example.test/id','mailto:a@example.test','tel:+12025550123','/relative','//example.test','https:example.test','https://',' https://example.test','https://exa\nmple.test','https://example.test/white space','https://example.test/\u0000x','https:\\example.test'];
const turn=()=>new Promise(r=>setImmediate(r));
async function setup(root,file,url,title='Stored destination'){
 const dom=new JSDOM(await readFile(new URL(`../${root}/${file}`,import.meta.url),'utf8'),{url:`http://127.0.0.1:39151/ui/${file}?tenant=tenant-a`,runScripts:'dangerously',virtualConsole:new VirtualConsole(),beforeParse(w){w.Headers=Headers;w.fetch=async path=>Response.json(path==='/api/v1/auth/session-identity'?{userId:'member-a',tenantId:'tenant-a',expiresAt:Date.now()+60000}:{store_name:'Actual owned business',bio:'Owned bio',theme:'light',links:[{title,url}],remove_branding:false});}});
 await turn();await turn();return dom;
}
for(const root of roots)for(const file of ['link-in-bio-generator.html','bio.html']){
 test(`${root}/${file}: every stored unsafe URL is unavailable without becoming an anchor`,async()=>{
  const observed=[];for(const url of unsafe){const dom=await setup(root,file,url);try{const target=dom.window.document.getElementById(file==='bio.html'?'links':'preview-links');observed.push({url,anchors:target.querySelectorAll('a[href]').length,text:target.textContent});}finally{dom.window.close();}}
  for(const row of observed){assert.equal(row.anchors,0,`unsafe link ${JSON.stringify(row.url)}`);assert.match(row.text,/Stored destination \(unavailable\)/);}
 });
 test(`${root}/${file}: quoted Unicode web URLs stay exact and titles remain text`,async()=>{
  const title='<img src=x onerror="throw 1">';for(const url of ['http://example.test/','https://example.test/路径?q=\'"&x=%26#✓','HTTPS://example.test/Case']){const dom=await setup(root,file,url,title);try{const target=dom.window.document.getElementById(file==='bio.html'?'links':'preview-links');const anchor=target.querySelector('a');assert.equal(anchor.getAttribute('href'),url);assert.equal(anchor.textContent,title);assert.equal(target.querySelector('img'),null);}finally{dom.window.close();}}
 });
}
