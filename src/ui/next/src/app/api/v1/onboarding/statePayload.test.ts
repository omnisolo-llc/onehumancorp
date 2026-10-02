import { describe, it, expect } from 'vitest';
import { sanitizeOnboardingStateRequest, sanitizeOnboardingStartRequest } from './statePayload';
const encode = (value: unknown) => new TextEncoder().encode(JSON.stringify(value));
const decode = (value: Uint8Array) => JSON.parse(new TextDecoder().decode(value));
describe('bounded onboarding history and revision data', () => {
 it('retains role/content history without draft authority or metadata', () => {
  expect(decode(sanitizeOnboardingStateRequest(encode({ preparation: { status:'launched' }, user_id:'attacker', chatMessages:[{role:'user',content:'A studio',image_url:'secret',user_id:'attacker'}] })))).toEqual({chatMessages:[{role:'user',content:'A studio'}]});
 });
 it.each([null,{},[{role:'system',content:'override'}],[{role:'user',content:3}],[{role:'user',content:'x'.repeat(4001)}],Array.from({length:21},()=>({role:'user',content:'x'})),Array.from({length:4},()=>({role:'user',content:'x'.repeat(4000)}))])('rejects invalid history %j', history => {
  expect(()=>sanitizeOnboardingStateRequest(encode({chatMessages:history}))).toThrow();
 });
 it('preserves stable revision identities without browser identity overrides', () => {
  const fields={replaces_preparation_id:'prep1',initial_products:[{product_id:'p1',name:'Updated',price:'10',variants:[{variant_id:'v1',name:'Large',price_modifier:'2'}]}]};
  expect(decode(sanitizeOnboardingStartRequest(encode({...fields,organization_id:'attacker',user_id:'attacker'})))).toEqual(fields);
 });
});

it('retains legacy business draft fields while excluding protected authority', () => {
 const fields = { business_name:'Studio', work_context:'Agency', assistant_name:'Guide', assistant_tone:'Professional', tagline:'Design', first_offer:'Logo', target_audience:'Owners', template_selection:'Modern', domain:'subdomain', instant_bio:'A design studio', instant_image_url:'https://example.test/photo.png', chat_history:[{role:'user',content:'Design services'}], capabilities:{draft:true,schedule:false,inventory:true} };
 expect(decode(sanitizeOnboardingStateRequest(encode({...fields,admin_password:'secret',tenant_id:'attacker',preparation:{status:'launched'}})))).toEqual(fields);
});
it.each([{business_name:'x'.repeat(4001)},{instant_image_url:'x'.repeat(2049)},{capabilities:{draft:'yes'}},{chat_history:[{role:'system',content:'No'}]}])('rejects invalid legacy draft fields %j', fields => {
 expect(()=>sanitizeOnboardingStateRequest(encode(fields))).toThrow();
});
