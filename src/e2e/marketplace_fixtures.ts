import { test as base, expect } from './fixtures';
import { createGrowthOwner } from './growth_owner';
import type { Page } from '@playwright/test';

/** Synthetic catalogue publication is restricted to the isolated native app. */
export async function createMarketplaceOwner(page:Page,baseURL:string|undefined){
 if(!baseURL)throw new Error('An explicit isolated app origin is required.');
 const origin=new URL(baseURL);
 if(!['http:','https:'].includes(origin.protocol)||!['localhost','127.0.0.1','[::1]'].includes(origin.hostname))throw new Error('Marketplace publication tests must stay on the isolated local app.');
 const owner=await createGrowthOwner(page,baseURL);
 const response=await page.request.get(new URL('/api/v1/auth/session-identity',origin).href);
 expect(response.status()).toBe(200);expect(await response.json()).toMatchObject({userId:owner.userId,tenantId:owner.tenantId});return owner;
}
export const test=base.extend<{marketplaceOwner:Awaited<ReturnType<typeof createMarketplaceOwner>>}>({marketplaceOwner:[async({page,baseURL},use)=>{await use(await createMarketplaceOwner(page,baseURL));},{auto:true}]});
export {expect};
