import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import ts from 'typescript';
const file='src/e2e/comprehensive_ui_contract.spec.ts';
const source=ts.createSourceFile(file,fs.readFileSync(file,'utf8'),ts.ScriptTarget.Latest,true);
const names=['every app page loads without visible crash output','visible internal links resolve to real pages','visible external and protocol links use expected destinations','all visible interactive elements are usable and named','layouts do not overflow or overlap click targets on desktop and mobile'];
for(const name of names)test(`global audit preserves exhaustive routes through settled navigation: ${name}`,()=>{
 let body;
 const visit=node=>{
  if(ts.isCallExpression(node)&&node.expression.getText(source)==='test'&&ts.isStringLiteral(node.arguments[0])&&node.arguments[0].text===name)body=node.arguments[1];
  ts.forEachChild(node,visit);
 };visit(source);assert.ok(body,'the required global audit must remain present');
 const calls=[];const collect=node=>{if(ts.isCallExpression(node))calls.push(node.expression.getText(source));ts.forEachChild(node,collect);};collect(body);
 assert.equal(calls.filter(call=>call==='page.goto').length,0,'raw DOMContentLoaded can inspect a document that is still redirecting');
 assert.equal(calls.filter(call=>call==='gotoReady').length,1,'every route must cross the real authenticated declared-destination gate');
 assert.match(body.getText(source),/for \(const route of appRoutes\)/);
 assert.match(body.getText(source),/expect\(failures\)\.toEqual\(\[\]\)/);
});
