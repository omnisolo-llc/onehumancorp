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
test('groups fit the 15–20 minute testing target using observed logical work and scheduling overhead', () => {
  // Run 37212397293: exact identities mapped back to their logical units.
  const seconds = [823.252,1157.768,1092.200,1451.816,361.203,337.050,321.986,282.902,354.014,145.234,196.248,404.513];
  for (const group of shards.GROUPS) {
    const estimate = group.reduce((sum,index) => sum + seconds[index-1],0) / 2 + 32;
    assert.ok(estimate >= 15*60 && estimate <= 20*60, `group ${group}: ${estimate/60} modeled minutes`);
  }
});
function fixture() {
  const slices = Array.from({length:12}, (_,i) => [{id:`id-${i}`,title:`test ${i}`,file:`src/${i}.spec.ts`}]);
  const full = slices.flat();
  const selections = shards.groupInventory(full,slices);
  const receipts = selections.map((selection,i) => ({shard:{index:i+1,total:3},selection,tests:structuredClone(selection),
    groupedInventory:{schemaVersion:1,source,shard:{current:i+1,total:3},groups:shards.GROUPS,inventory:full,slices}}));
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
