import { beforeEach, expect, it, vi } from 'vitest';
import { evaluateAuthMiddleware } from '@/lib/auth/middlewareCore';
import { parseAuthRuntimeConfig } from '@/lib/auth/runtimeConfig';
import { parseSessionKeyRing } from '@/lib/auth/sessionKeys';
import { sealSession } from '@/lib/auth/sessionCodec';
import { cookieForSession, serializeSessionCookie, sessionCodecContext } from '@/lib/auth/sessionCookie';
import type { BackendTransportDependencies, BackendRequestOptions } from '@/lib/auth/backendTransport';
let dependencies: BackendTransportDependencies;
vi.mock('@/lib/auth/backendTransport', async original => {
 const actual=await original<typeof import('@/lib/auth/backendTransport')>();
 return {...actual,proxyBackendRequest:(request:Request,path:string,options:BackendRequestOptions)=>actual.proxyAuthenticatedRequest(request,path,dependencies,options)};
});
const NOW=1800000000;const backend=vi.fn<typeof fetch>();
const requestId='10000000-0000-4000-8000-000000000001';const definitionId='20000000-0000-4000-8000-000000000001';
const publication={request_id:requestId,name:'Reviewed Writer',description:'Owner-reviewed purpose',role:'Writer',system_prompt:'Draft only.\nWait for review.',visibility:'public'};
beforeEach(async()=>{
 backend.mockReset().mockResolvedValue(Response.json({success:true,status:'published',request_id:requestId}));
 dependencies={config:parseAuthRuntimeConfig({OMNISOLO_WEB_CANONICAL_ORIGIN:'https://app.example.test',BACKEND_URL:'https://backend.example.test'}),ring:await parseSessionKeyRing({OMNISOLO_WEB_SESSION_KEY_ID:'test',OMNISOLO_WEB_SESSION_SECRET:'Ww7LSLEn9AaAN6IT5kwJ0yGqVO11CMI9nOEqi7wF10I'}),now:()=>NOW,fetchImpl:backend,timeoutMs:1000,requestLimitBytes:2*1024*1024,responseLimitBytes:2*1024*1024};
});
async function request(path:string,body?:unknown,extra:Record<string,string>={}){
 const session={version:1 as const,iat:NOW,exp:NOW+3600,accessToken:'synthetic-test-bearer',user:{id:'owner-a',username:'owner',roles:['ADMIN'],organizationId:'tenant-a'}};
 const sealed=await sealSession(session,dependencies.ring,sessionCodecContext(dependencies.config),{now:NOW,backendExpiresAt:session.exp});
 const cookie=serializeSessionCookie(cookieForSession(dependencies.config,sealed,session.iat,session.exp)).split(';',1)[0];
 return new Request('https://app.example.test/api/v1/agents/definitions'+path,{method:body===undefined?'GET':'POST',headers:{cookie,origin:'https://app.example.test',...(body===undefined?{}:{'content-type':'application/json'}),'x-ohc-expected-user':'owner-a','x-ohc-expected-tenant':'tenant-a',...extra},...(body===undefined?{}:{body:JSON.stringify(body)})});
}
it('carries every reviewed definition field through sealed transport and strips browser identity',async()=>{
 const {POST}=await import('./route');const response=await POST(await request('',{...publication,tenant_id:'forged',user_id:'forged'}),{params:Promise.resolve({path:['agents','definitions']})});
 expect(response.status).toBe(200);expect(response.headers.get('cache-control')).toBe('private, no-store');
 const[target,init]=backend.mock.calls[0];expect(String(target)).toBe('https://backend.example.test/api/v1/agents/definitions');
 expect(JSON.parse(new TextDecoder().decode(init?.body as Uint8Array))).toEqual(publication);const headers=new Headers(init?.headers);
 expect(headers.get('authorization')).toBe('Bearer synthetic-test-bearer');expect(headers.get('x-tenant-id')).toBe('tenant-a');expect(headers.get('x-user-id')).toBe('owner-a');expect(init?.redirect).toBe('manual');
});
it('keeps both opaque pagination cursors and Unicode query on the configured backend origin',async()=>{
 const {GET}=await import('./route');const params=new URLSearchParams({q:'作者 / ?','cursor':'public:next','installation_cursor':'private:next',limit:'50'});
 await GET(await request('?'+params),{params:Promise.resolve({path:['agents','definitions']})});
 const target=new URL(String(backend.mock.calls[0][0]));expect(target.origin).toBe('https://backend.example.test');expect([...target.searchParams]).toEqual([...params]);
});
it.each([`/${definitionId}/install`,`/operations/${requestId}`])('preserves the exact operation route %s',async suffix=>{
 const {GET,POST}=await import('./route');const body=suffix.endsWith('/install')?{request_id:requestId,version:1,digest:'a'.repeat(64)}:undefined;
 const response=await(body?POST:GET)(await request(suffix,body),{params:Promise.resolve({path:['agents','definitions',...suffix.slice(1).split('/')]})});
 expect(response.status).toBe(200);expect(String(backend.mock.calls[0][0])).toBe('https://backend.example.test/api/v1/agents/definitions'+suffix);
 if(body)expect(JSON.parse(new TextDecoder().decode(backend.mock.calls[0][1]?.body as Uint8Array))).toEqual(body);
});
it('rejects a stale owner precondition before backend mutation',async()=>{
 const {POST}=await import('./route');expect((await POST(await request('',publication,{'x-ohc-expected-user':'owner-b'}),{params:Promise.resolve({path:['agents','definitions']})})).status).toBe(409);expect(backend).not.toHaveBeenCalled();
});
it('rejects unsealed and cross-origin writes before backend mutation',async()=>{
 const {POST}=await import('./route');const context={params:Promise.resolve({path:['agents','definitions']})};
 expect((await POST(new Request('https://app.example.test/api/v1/agents/definitions',{method:'POST',body:JSON.stringify(publication)}),context)).status).toBe(401);
 const crossOrigin=await request('',publication,{origin:'https://other.example.test','sec-fetch-site':'cross-site'});
 expect(await evaluateAuthMiddleware(crossOrigin,dependencies)).toMatchObject({kind:'response',status:403});
 expect(await evaluateAuthMiddleware(await request('',publication,{'sec-fetch-site':'same-origin'}),dependencies)).toMatchObject({kind:'next'});expect(backend).not.toHaveBeenCalled();
});
