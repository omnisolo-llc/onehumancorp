import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import test from 'node:test';
import ts from 'typescript';

// The root and help layout own different TooltipProviders. A completed outer
// fetch can populate the global while the inner wrapper still has fallback
// text; touchstart captures that text. Execute the maintained spec callback
// against this boundary model, not a replacement browser acceptance test.
test('help tooltip waits for its own committed copy before starting the gesture',async()=>{
  const source=readFileSync(new URL('../src/ui/next/src/e2e/tooltips.spec.ts',import.meta.url),'utf8');
  const expected='Search for help articles and videos...';
  let body,ready=false,displayed='',waited=false;
  const pwTest=(title,callback)=>{if(title==='renders help search tooltip on hover')body=callback;};
  pwTest.describe=(_title,callback)=>callback();
  const context={exports:{},window:{OMNISOLO_TOOLTIPS:{'help-search-tooltip':expected}},
    TouchEvent:class {constructor(type){this.type=type;}},
    document:{getElementById:id=>{assert.equal(id,'help-search-tooltip');return {querySelector:()=>({dispatchEvent:event=>{assert.equal(event.type,'touchstart');displayed=ready?expected:'Search for articles, videos, and guides';}})};}},
    require:name=>{assert.equal(name,'@playwright/test');return {test:pwTest,expect:target=>({toHaveAttribute:async(name,value,options)=>{
      assert.equal(target.kind,'target');assert.equal(name,'data-tooltip');assert.equal(value,expected);
      assert.equal(options.timeout,10000);ready=true;waited=true;
    }})};}};
  vm.runInNewContext(ts.transpileModule(source,{compilerOptions:{module:ts.ModuleKind.CommonJS}}).outputText,context);
  assert.equal(typeof body,'function');
  const page={goto:async()=>{},waitForLoadState:async()=>{},waitForFunction:async predicate=>assert.equal(predicate(),true),
    evaluate:async callback=>callback(),waitForTimeout:async milliseconds=>assert.equal(milliseconds,600),
    locator:(selector,options)=>selector==='#help-search-tooltip'?{kind:'target',waitFor:async()=>{}}:{
      last(){return this;},waitFor:async configuration=>{
        assert.equal(configuration.timeout,5000);assert.equal(options.hasText,expected);
        assert.equal(displayed,expected,'original tooltip assertion must retain the fetched server copy');
      },
    },
  };
  await body({page});assert.equal(waited,true);
});
