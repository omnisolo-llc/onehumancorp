// Owned-runtime budget API through the actual sealed-session web proxy.
const fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict'),ts=require('typescript');
assert.equal(Number(process.versions.node.split('.')[0]),22);
assert.equal(ts.version,JSON.parse(fs.readFileSync(path.resolve(__dirname,'../../src/ui/next/package-lock.json'),'utf8')).packages['node_modules/typescript'].version);
require.extensions['.ts']=(module,file)=>module._compile(ts.transpileModule(fs.readFileSync(file,'utf8'),{compilerOptions:{module:ts.ModuleKind.CommonJS,target:ts.ScriptTarget.ES2022,esModuleInterop:true,resolveJsonModule:true}}).outputText,file);
const auth=path.resolve(__dirname,'../../src/ui/next/src/lib/auth');
const {proxyAuthenticatedRequest}=require(path.join(auth,'backendTransport.ts'));
const {parseAuthRuntimeConfig}=require(path.join(auth,'runtimeConfig.ts'));
const {parseSessionKeyRing}=require(path.join(auth,'sessionKeys.ts'));
const {sealSession}=require(path.join(auth,'sessionCodec.ts'));
const {cookieForSession,serializeSessionCookie,sessionCodecContext}=require(path.join(auth,'sessionCookie.ts'));
(async()=>{
 const input=JSON.parse(fs.readFileSync(0,'utf8'));const target=new URL(input.base_url);assert.equal(target.hostname,'127.0.0.1');assert.equal(target.protocol,'http:');
 const now=Math.floor(Date.now()/1000),canonical='http://127.0.0.1:3000';
 const config=parseAuthRuntimeConfig({OMNISOLO_WEB_LOCAL_DEV:'true',OMNISOLO_WEB_CANONICAL_ORIGIN:canonical,BACKEND_URL:target.origin});
 const ring=await parseSessionKeyRing({OMNISOLO_WEB_SESSION_KEY_ID:'owned-usage-test',OMNISOLO_WEB_SESSION_SECRET:'Ww7LSLEn9AaAN6IT5kwJ0yGqVO11CMI9nOEqi7wF10I'});
 const session={version:1,iat:now,exp:now+300,accessToken:input.token,user:{id:input.actor,username:'Owned budget fixture',roles:['OWNER'],organizationId:input.tenant}};
 const compact=await sealSession(session,ring,sessionCodecContext(config),{now,backendExpiresAt:session.exp});
 const cookie=serializeSessionCookie(cookieForSession(config,compact,now,session.exp)).split(';',1)[0];
 const write=input.operation==='set_limit';
 assert.ok(write||input.operation==='records');
 const route=write?'/api/v1/billing/usage/spending-limit':'/api/v1/billing/usage/records';
 const request=new Request(canonical+route+(write?'':'?after=&tenant_id=untrusted-input&user_id=untrusted-input'),{method:write?'PUT':'GET',headers:{cookie,origin:canonical,'content-type':'application/json'},...(write?{body:JSON.stringify({limit_micros:3000})}:{})});
 const response=await proxyAuthenticatedRequest(request,route,{config,ring,now:()=>now,fetchImpl:fetch,timeoutMs:5000,requestLimitBytes:1048576,responseLimitBytes:2097152});
 const body=await response.text();
 assert.equal(response.status,200,`${input.operation} through actual proxy failed: ${response.status}`);
 const value=JSON.parse(body);
 if(write){assert.equal(value.spending_authorization.limit_micros,3000);assert.equal(value.is_payment_credit,false);}else{assert.ok(Array.isArray(value.records));assert.equal(value.records.length,1);assert.equal(value.records[0].event_id,'owned-record-a');assert.ok(value.records.every(record=>record.scope.tenant_id===input.tenant));}
 process.stdout.write(JSON.stringify({status:200,operation:input.operation,body:value}));
})().catch(error=>{console.error(error);process.exitCode=1;});
