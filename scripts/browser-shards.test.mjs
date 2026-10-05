import test from 'node:test';
import assert from 'node:assert/strict';
import protocol from './ui-click-audit.cjs';

test('complete CI accepts only owned three-way grouping, never a caller test list', () => {
  assert.equal(protocol.completeSelection(['--grouped-shard=1/3', '--workers=2', '--retries=0']), true);
  for (const args of [['--grouped-shard=0/3'], ['--grouped-shard=1/4'], ['--grouped-shard=1/3','--shard=1/12'], ['--grouped-shard=1/3','--test-list=arbitrary.txt']]) {
    assert.equal(protocol.completeSelection(args), false, args.join(' '));
  }
});

import shards from './browser-shards.cjs';
const source = { commit: 'a'.repeat(40), sourceDigest: 'b'.repeat(64) };
test('groups balance observed variability within the 15–20 minute testing target', () => {
  // Original twelve-runner baseline 37209169837 and isolated three-runner
  // run 37218212302, plus failed run 37220871509: actual logical wall time,
  // not summed test durations. Failed-run timings are not correctness evidence.
  const observations = [
    [515.711,583.814,592.806,636.707,169.255,176.713,177.645,151.762,131.271,97.056,84.588,168.227],
    [520.936734,475.641143,574.614912,710.200950,198.374961,128.114481,130.298400,150.215440,133.160062,55.572656,104.618273,204.805025],
    [470.170703,713.415726,455.530801,599.729003,163.682787,137.504137,142.918925,125.912642,144.385638,70.415350,103.044192,162.060979],
  ];
  for (const seconds of observations) {
    const totals=shards.GROUPS.map(group=>group.reduce((sum,index)=>sum+seconds[index-1],0));
    for(const estimate of totals) assert.ok(estimate>=15*60&&estimate<=20*60, `${estimate/60} modeled minutes`);
    // Exhaustive partitioning puts the best possible worst spread at 121.78s
    // for these observations. Allow 130s while preserving the 15–20m bounds.
    assert.ok(Math.max(...totals)-Math.min(...totals)<=130,'modeled active testing spread must stay within 130 seconds');
  }
});

test('slower complete observation stays evenly distributed without relaxing historical targets', () => {
  // Run 37225051173 passed all browser tests. Its 63.18 total active minutes
  // cannot fit within three 20-minute bins; retain the historical checks above.
  const seconds = [559.978027,825.794172,573.258538,615.061265,169.078037,141.788080,145.410142,157.279647,186.467892,98.301978,106.100368,212.047404];
  const totals = shards.GROUPS.map(group => group.reduce((sum,index) => sum+seconds[index-1],0));
  assert.ok(seconds.reduce((sum,value) => sum+value,0) > 3*20*60);
  // Best spread among all partitions satisfying both historical guards: 206.44s.
  assert.ok(Math.max(...totals)-Math.min(...totals)<=210, 'slower-run spread must stay within 210 seconds');
});

function fixture() {
  const slices = Array.from({length:12}, (_,i) => [{id:`id-${i}`,title:`test ${i}`,file:`src/${i}.spec.ts`}]);
  const full = slices.flat();
  const selections = shards.groupInventory(full,slices);
  const receipts = shards.GROUPS.flatMap((group,i) => group.map(logical => ({shard:{index:logical,total:12},selection:slices[logical-1],tests:structuredClone(slices[logical-1]),
    groupedInventory:{schemaVersion:1,source,shard:{current:i+1,total:3},logicalShard:logical,groups:shards.GROUPS,inventory:full,slices}})));
  return {slices,full,selections,receipts};
}
test('grouping retains each complete logical unit and the exact full inventory', () => {
  const {full,slices,selections,receipts}=fixture();
  assert.deepEqual(selections.map(group=>group.map(t=>t.id)), shards.GROUPS.map(group=>group.map(n=>`id-${n-1}`)));
  shards.sameInventory(selections.flat(),full);
  shards.validateGroupedReceipts(receipts,source,true);
  const added={id:'newly-discovered',title:'new test',file:'src/new.spec.ts'};
  full.push(added);slices[0].push(added);
  assert.ok(shards.groupInventory(full,slices)[0].some(test=>test.id===added.id));
});
for (const [name,mutate] of [
  ['omitted ordinary test',f=>{f.receipts[0].selection=f.receipts[0].selection.slice(1);f.receipts[0].tests=f.receipts[0].tests.slice(1);} ],
  ['missing inventory proof',f=>f.receipts.forEach(r=>delete r.groupedInventory)],
  ['stale source',f=>{f.receipts[0].groupedInventory.source={...source,commit:'c'.repeat(40)};}],
  ['duplicate execution',f=>f.receipts[0].tests.push(f.receipts[0].tests[0])],
  ['changed group assignment',f=>{[f.receipts[0].selection,f.receipts[1].selection]=[f.receipts[1].selection,f.receipts[0].selection];}],
  ['missing grouped runner',f=>f.receipts.pop()],
]) test(`full discovery guard rejects ${name}`,()=>{const f=fixture();mutate(f);assert.throws(()=>shards.validateGroupedReceipts(f.receipts,source,true));});
test('logical shard inventory refuses missing, duplicate and substituted identities even at the same count',()=>{
  for(const mutate of [f=>f.slices.pop(),f=>f.slices[0].pop(),f=>f.slices[1].splice(0,1,f.slices[0][0]),f=>{f.slices[1][0]={...f.slices[1][0],title:'changed'};}]) {
    const f=fixture();mutate(f);assert.throws(()=>shards.groupInventory(f.full,f.slices));
  }
  const f=fixture();assert.throws(()=>shards.groupInventory(f.full,f.slices,[[1,2,10],[3,5,7,8,11],[4,6,9,11]]));
});

test('complete inventory transfer survives bounded native command output', async () => {
  const { runNativeCommand } = await import('./native-process.mjs');
  const { createRequire } = await import('node:module');
  const require = createRequire(import.meta.url);
  const reporter = require.resolve('./browser-inventory-reporter.cjs');
  const expected = Array.from({length:2000}, (_,i)=>({id:`test-${i}`,title:'large discovery title '.repeat(10),file:'src/example.spec.ts',selector:`[chromium] › example.spec.ts › test ${i}`}));
  assert.ok(JSON.stringify(expected).length > 128 * 1024);
  const actual = await shards.readDiscovery(async filename => {
    await runNativeCommand(process.execPath,['-e', `const Reporter=require(process.argv[1]);const r=new Reporter();r.tests=JSON.parse(require('node:fs').readFileSync(0,'utf8'));r.onEnd({status:'passed'});`,reporter],
      {env:{...process.env,OHC_BROWSER_INVENTORY_OUTPUT:filename},input:JSON.stringify(expected),quiet:true});
  });
  assert.deepEqual(actual,expected);
});

import { readFile, access } from 'node:fs/promises';
function discoveryFixture() {
  const {full,slices}=fixture();
  return async selection => {
    if (!selection.length) return full.map(t=>({...t,selector:t.id}));
    const logical=Number(selection[0].match(/^--shard=(\d+)\/12$/)?.[1]);
    if(logical) return slices[logical-1].map(t=>({...t,selector:t.id}));
    const selected=(await readFile(selection[1],'utf8')).trim().split('\n');
    return full.filter(t=>selected.includes(t.id));
  };
}
test('grouped units retain separate proof, artifacts and lifecycle; later success cannot erase failure',async()=>{
  const visited=[],proofs=[],directories=[];let active=0;
  await assert.rejects(shards.runGroupedUnits({index:2,source,list:discoveryFixture(),
    runUnit:async(logical,context)=>{
      assert.equal(active++,0);visited.push(logical);directories.push(context.artifactSuffix);
      const proof=JSON.parse(await readFile(context.proofFile,'utf8'));proofs.push(context.proofFile);
      assert.deepEqual(shards.validateProof(proof,source,proof.slices[logical-1]),{current:logical,total:12});
      await context.runBrowser(async({timeoutMs,signal})=>{assert.equal(signal.aborted,false);assert.ok(timeoutMs>0&&timeoutMs<=60*60*1000);if(visited.length===1)assert.equal(timeoutMs,60*60*1000);});
      active--;if(logical===shards.GROUPS[1][0])throw new Error('original assertion failure');
    }}),/original assertion failure/);
  assert.deepEqual(visited,shards.GROUPS[1]);assert.equal(new Set(directories).size,visited.length);
  for(const file of proofs)await assert.rejects(access(file));
});
test('grouped units share one active browser budget and stop launching on exhaustion',async()=>{
  let clock=0;const visited=[],budgets=[];
  await assert.rejects(shards.runGroupedUnits({index:2,source,list:discoveryFixture(),now:()=>clock,budgetMs:100,
    runUnit:async(logical,context)=>{visited.push(logical);clock+=1000; // service initialization is separate
      await context.runBrowser(async({timeoutMs})=>{budgets.push(timeoutMs);clock+=60;});
    }}),/browser budget exhausted/);
  assert.deepEqual(visited,shards.GROUPS[1].slice(0,2));assert.deepEqual(budgets,[100,40]);
});
test('group cancellation aborts the active lifecycle and prevents remaining units',async()=>{
  const controller=new AbortController(),visited=[];
  await assert.rejects(shards.runGroupedUnits({index:2,source,list:discoveryFixture(),signal:controller.signal,
    runUnit:async(logical,context)=>{visited.push(logical);controller.abort(new Error('cancelled by owner'));context.signal.throwIfAborted();}
  }),/cancelled by owner/);
  assert.deepEqual(visited,shards.GROUPS[1].slice(0,1));
});
test('required grouped evidence refuses the old three shared lifecycles and wrong logical assignment',()=>{
  const f=fixture();
  const old=f.selections.map((selection,i)=>({shard:{index:i+1,total:3},selection,tests:selection,
    groupedInventory:{...f.receipts[0].groupedInventory,logicalShard:undefined,shard:{current:i+1,total:3}}}));
  assert.throws(()=>shards.validateGroupedReceipts(old,source,true));
  const changed=fixture();changed.receipts[0].groupedInventory.logicalShard=shards.GROUPS[1][0];
  assert.throws(()=>shards.validateGroupedReceipts(changed.receipts,source,true));
});

test('a real browser child deadline is bounded across units and cleanup retains earlier failure evidence',async()=>{
  const {runNativeCommand}=await import('./native-process.mjs');
  const visited=[];
  await assert.rejects(shards.runGroupedUnits({index:2,source,list:discoveryFixture(),budgetMs:50,
    runUnit:async(logical,context)=>{
      visited.push(logical);
      await context.runBrowser(options=>runNativeCommand(process.execPath,['-e','setInterval(()=>{},1000)'],{...options,quiet:true}));
    }}),/browser budget exhausted/);
  assert.deepEqual(visited,shards.GROUPS[1].slice(0,1));
});
