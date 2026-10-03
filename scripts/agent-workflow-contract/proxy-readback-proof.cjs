// Real HTTP bytes from receipt tests pass through the production session/proxy limit.
const fs = require('node:fs');
const path = require('node:path');
const http = require('node:http');
const assert = require('node:assert/strict');
const ts = require('typescript');
const lock = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../../src/ui/next/package-lock.json'),'utf8'));
assert.equal(ts.version,lock.packages['node_modules/typescript'].version,'use the locked TypeScript transpiler');
if (Number(process.versions.node.split('.')[0]) !== 22) throw new Error('Node 22 is required');
require.extensions['.ts'] = (module, file) => module._compile(ts.transpileModule(fs.readFileSync(file, 'utf8'), { compilerOptions: { module: ts.ModuleKind.CommonJS, esModuleInterop: true, resolveJsonModule: true, target: ts.ScriptTarget.ES2022 } }).outputText, file);
const auth = path.resolve(__dirname, '../../src/ui/next/src/lib/auth');
const { proxyAuthenticatedRequest } = require(path.join(auth, 'backendTransport.ts'));
const { parseAuthRuntimeConfig } = require(path.join(auth, 'runtimeConfig.ts'));
const { parseSessionKeyRing } = require(path.join(auth, 'sessionKeys.ts'));
const { sealSession } = require(path.join(auth, 'sessionCodec.ts'));
const { cookieForSession, serializeSessionCookie, sessionCodecContext } = require(path.join(auth, 'sessionCookie.ts'));
(async () => {
  const snapshot = JSON.parse(fs.readFileSync(0, 'utf8'));
  const body = JSON.stringify(snapshot);
  let calls = 0;
  const server = http.createServer((req, res) => {
    assert.equal(req.headers.authorization, 'Bearer public-owned-readback-fixture');
    calls++; res.writeHead(200, { 'content-type': 'application/json' });
    // Deliberately chunked, so the real cumulative response cap is exercised.
    for (let i = 0; i < body.length; i += 8192) res.write(body.slice(i, i + 8192));
    res.end();
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  try {
    const now = Math.floor(Date.now() / 1000);
    const config = parseAuthRuntimeConfig({ OMNISOLO_WEB_LOCAL_DEV: 'true', OMNISOLO_WEB_CANONICAL_ORIGIN: 'http://127.0.0.1:3000', BACKEND_URL: `http://127.0.0.1:${server.address().port}` });
    const ring = await parseSessionKeyRing({ OMNISOLO_WEB_SESSION_KEY_ID: 'owned-readback-test', OMNISOLO_WEB_SESSION_SECRET: 'Ww7LSLEn9AaAN6IT5kwJ0yGqVO11CMI9nOEqi7wF10I' });
    const session = { version:1, iat:now, exp:now+300, accessToken:'public-owned-readback-fixture', user:{id:'owned-reader',username:'Owned fixture',roles:['ADMIN'],organizationId:'workflow-tenant-a'} };
    const compact = await sealSession(session, ring, sessionCodecContext(config), {now,backendExpiresAt:session.exp});
    const cookie = serializeSessionCookie(cookieForSession(config,compact,now,session.exp)).split(';',1)[0];
    const response = await proxyAuthenticatedRequest(new Request('http://127.0.0.1:3000/api/v1/agents/workflows', {headers:{cookie}}), '/api/v1/agents/workflows', {config,ring,now:()=>now,fetchImpl:fetch,timeoutMs:5000,requestLimitBytes:1048576,responseLimitBytes:2097152});
    assert.equal(response.status, 200, 'actual proxy must accept this complete receipt page');
    assert.deepEqual(await response.json(), snapshot, 'proxy must preserve every task and output byte');
    assert.equal(calls,1);
    process.stdout.write(JSON.stringify({status:200,bytes:Buffer.byteLength(body),calls}));
  } finally { await new Promise(resolve => server.close(resolve)); }
})().catch(error=>{console.error(error);process.exitCode=1;});
