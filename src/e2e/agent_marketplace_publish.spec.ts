import { test, expect, createMarketplaceOwner } from './marketplace_fixtures';

test.describe('Agent Marketplace Publish E2E', () => {
  test('User can publish a new agent and then find it in the marketplace', async ({ page,baseURL,marketplaceOwner }) => {
    await page.goto('/agent-marketplace/publish');await expect(page.getByRole('heading',{name:'Publish New Agent',level:1})).toBeVisible();
    const name=`E2E Custom Agent ${marketplaceOwner.userId}`;
    const description='Synthetic local test definition for owner review.';const role='E2E Tester';const prompt='Draft synthetic test text.\nNo tools, network access or execution is authorized by installation.';
    await page.getByLabel('Agent Name',{exact:true}).fill(name);await page.getByLabel('Description',{exact:true}).fill(description);await page.getByLabel('Role',{exact:true}).fill(role);await page.getByLabel('System Prompt',{exact:true}).fill(prompt);
    await page.getByRole('button',{name:'Review Publication'}).click();const review=page.getByRole('region',{name:'Public agent definition review'});
    await expect(review).toContainText('other authenticated users of this OHC marketplace');await expect(review.getByText(name,{exact:true})).toBeVisible();await expect(review.getByText(description,{exact:true})).toBeVisible();await expect(review.getByText(role,{exact:true})).toBeVisible();await expect(review.locator('dd').last()).toHaveText(prompt);
    const pending=page.waitForResponse(response=>new URL(response.url()).pathname==='/api/v1/agents/definitions'&&response.request().method()==='POST').then(async response=>({status:response.status(),body:await response.json()}));
    await page.getByRole('button',{name:'Publish Publicly'}).click();const receipt=await pending;
    expect(receipt.status).toBe(200);expect(receipt.body).toMatchObject({success:true,status:'published',user_id:marketplaceOwner.userId,organization_id:marketplaceOwner.tenantId,definition:{name,description,role,system_prompt:prompt,visibility:'public',source:'community',version:1}});
    expect(receipt.body.definition.digest).toMatch(/^[0-9a-f]{64}$/);await expect(page).toHaveURL(/\/agent-marketplace$/);await page.getByPlaceholder('Search for agents...').fill(name);
    await expect(page.getByRole('article',{name})).toContainText(description);await page.reload();await page.getByPlaceholder('Search for agents...').fill(name);await expect(page.getByRole('article',{name})).toBeVisible();
    const recovered=await page.request.get(`/api/v1/agents/definitions/operations/${encodeURIComponent(receipt.body.request_id)}`);expect(recovered.status()).toBe(200);expect(await recovered.json()).toMatchObject({...receipt.body,replayed:true});
    const other=await createMarketplaceOwner(page,baseURL);expect(other.tenantId).not.toBe(marketplaceOwner.tenantId);await page.goto('/agent-marketplace');await page.getByPlaceholder('Search for agents...').fill(name);await expect(page.getByRole('article',{name})).toContainText(description);
    const foreign=await page.request.get(`/api/v1/agents/definitions/operations/${encodeURIComponent(receipt.body.request_id)}`);expect(foreign.status()).toBe(404);
    const publicView=await page.request.get('/api/v1/agents/definitions?'+new URLSearchParams({q:name}));expect(publicView.status()).toBe(200);const data=await publicView.json();expect(data.definitions).toContainEqual(receipt.body.definition);expect(data.installations).toEqual([]);
    await expect(page.getByRole('article',{name}).getByRole('button',{name:'Install Agent',exact:true})).toBeEnabled();
  });
});
