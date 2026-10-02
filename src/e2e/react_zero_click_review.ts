import type { Page } from '@playwright/test';
import { expect } from './fixtures';

type Owner={userId:string;tenantId:string};
type ChatResult={status:number;body:Record<string,unknown>};
const capture=(page:Page,path:string)=>page.waitForResponse(response=>new URL(response.url()).pathname===path&&response.request().method()==='POST').then(async response=>({status:response.status(),body:await response.json()}));
/** Uses real responses and explicit owner input; no provider output is substituted. */
export async function completeReactZeroClickReview(page:Page,owner:Owner,result:ChatResult){
 const origin=new URL(page.url());expect(['localhost','127.0.0.1','[::1]']).toContain(origin.hostname);
 let prepared,launched;let mode:string;
 if(result.status===503){
  expect(result.body).toMatchObject({error:'onboarding_ai_unconfigured'});expect(result.body.success).not.toBe(true);mode='Explicit manual setup after the real unconfigured-provider response';
  await page.getByRole('button',{name:'Review Details Manually',exact:true}).click();await expect(page).toHaveURL(/\/onboarding$/);await expect(page.getByRole('heading',{name:'Review Details',exact:true})).toBeVisible();
  await expect(page.getByRole('textbox',{name:'First Product',exact:true})).toHaveValue('');await expect(page.getByRole('textbox',{name:'Price',exact:true})).toHaveValue('');
  await page.getByRole('textbox',{name:'Business Name',exact:true}).fill('Owner-reviewed test studio');await page.getByRole('textbox',{name:'Business Type',exact:true}).fill('Services');await page.getByRole('textbox',{name:'First Product',exact:true}).fill('Owner-reviewed consultation');await page.getByRole('textbox',{name:'Price',exact:true}).fill('25.00');
  await page.getByRole('button',{name:'Continue',exact:true}).click();await expect(page.getByText('Style & Team',{exact:true})).toBeVisible();
  [prepared,launched]=await Promise.all([capture(page,'/api/v1/onboarding/start'),capture(page,'/api/v1/onboarding/launch'),page.getByRole('button',{name:'Approve & Complete Setup'}).click()]);
  expect(prepared.body.preparation.catalog).toEqual(expect.arrayContaining([expect.objectContaining({name:'Owner-reviewed consultation',price:'25.00'})]));
 }else{
  expect(result.status).toBe(200);expect(result.body.is_complete).toBe(true);mode='Configured provider returned reviewable intake';
  [prepared]=await Promise.all([capture(page,'/api/v1/onboarding/start'),page.getByRole('button',{name:/Approve.*Prepare Workspace/}).click()]);
  await expect(page.getByText('Your workspace is prepared',{exact:true})).toBeVisible();await expect(page.getByText('Setup complete',{exact:true})).toHaveCount(0);
  [launched]=await Promise.all([capture(page,'/api/v1/onboarding/launch'),page.getByRole('button',{name:/Launch My Store/}).click()]);
 }
 expect(prepared.status).toBe(200);expect(prepared.body).toMatchObject({success:true,status:'prepared',organization_id:owner.tenantId,user_id:owner.userId});expect(typeof prepared.body.preparation_id).toBe('string');expect(prepared.body.preparation_id.length).toBeGreaterThan(0);
 expect(launched.status).toBe(200);expect(launched.body).toMatchObject({success:true,status:'launched',preparation_id:prepared.body.preparation_id,organization_id:owner.tenantId,user_id:owner.userId});await expect(page.getByText('Setup complete',{exact:true})).toBeVisible();
 const state=await page.request.get('/api/v1/onboarding/state');expect(state.status()).toBe(200);expect((await state.json()).preparation).toMatchObject({status:'launched',preparation_id:prepared.body.preparation_id,organization_id:owner.tenantId,user_id:owner.userId});
 // Reopen the original page from the protected receipt, retaining its real
 // dashboard/share affordances without treating manual entry as AI generation.
 await page.goto('/onboarding/zero-click');await expect(page.getByText('Setup complete',{exact:true})).toBeVisible();return mode;
}
