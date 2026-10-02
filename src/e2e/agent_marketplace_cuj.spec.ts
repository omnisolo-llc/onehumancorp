import { test, expect } from './marketplace_fixtures';

test.describe('Agent Marketplace E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/agent-marketplace');
    await expect(page.getByRole('main').getByRole('heading', { name: 'Agent Marketplace', level: 1 })).toBeVisible();
  });
  test('Page load and initial agents visible', async ({ page }) => {
    await expect(page.getByPlaceholder('Search for agents...')).toBeVisible();
    await expect(page.getByRole('article',{name:'Senior Rust Developer'}).getByRole('heading',{name:'Senior Rust Developer'})).toBeVisible();
    await expect(page.getByRole('article',{name:'Technical Writer'})).toBeVisible();
  });
  test('Search filters agents correctly', async ({ page }) => {
    await page.getByPlaceholder('Search for agents...').fill('Senior');
    await expect(page.getByRole('article',{name:'Senior Rust Developer'})).toBeVisible();
    await expect(page.getByRole('article',{name:'Technical Writer'})).toHaveCount(0);
  });
  test('Search with no results shows empty state', async ({ page }) => {
    await page.getByPlaceholder('Search for agents...').fill('NonexistentAgent123');
    await expect(page.getByText('No agents found.',{exact:true})).toBeVisible();
    await expect(page.getByRole('article')).toHaveCount(0);
  });
  test('Clear search restores original list', async ({ page }) => {
    const search=page.getByPlaceholder('Search for agents...');await search.fill('Senior');
    await expect(page.getByRole('article',{name:'Senior Rust Developer'})).toBeVisible();await expect(page.getByRole('article',{name:'Technical Writer'})).toHaveCount(0);
    await search.fill('');await expect(page.getByRole('article',{name:'Senior Rust Developer'})).toBeVisible();await expect(page.getByRole('article',{name:'Technical Writer'})).toBeVisible();
  });
  test('Install an agent shows toast notification and updates button', async ({ page,marketplaceOwner }) => {
    await page.getByPlaceholder('Search for agents...').fill('Senior Rust Developer');
    const card=page.getByRole('article',{name:'Senior Rust Developer'});await card.getByRole('button',{name:'Install Agent',exact:true}).click();
    const review=page.getByRole('region',{name:'Review inactive installation'});await expect(review).toContainText('No work starts');await expect(review.locator('pre')).not.toBeEmpty();
    const pending=page.waitForResponse(response=>/\/api\/v1\/agents\/definitions\/[^/]+\/install$/.test(new URL(response.url()).pathname)&&response.request().method()==='POST').then(async response=>({status:response.status(),body:await response.json()}));
    await review.getByRole('button',{name:'Confirm Inactive Installation'}).click();const receipt=await pending;
    expect(receipt.status).toBe(200);expect(receipt.body).toMatchObject({success:true,status:'installed_inactive',user_id:marketplaceOwner.userId,organization_id:marketplaceOwner.tenantId,installation:{name:'Senior Rust Developer',status:'installed_inactive'}});
    await expect(page.getByText(/Agent installed successfully as an inactive definition/)).toBeVisible();await expect(card.getByRole('button',{name:'Installed',exact:true})).toHaveAttribute('aria-pressed','true');
    const saved=await page.request.get('/api/v1/agents/definitions?q=Senior');expect(saved.status()).toBe(200);const catalogue=await saved.json();expect(catalogue.installations.filter((item:{id:string})=>item.id===receipt.body.installation.id)).toEqual([receipt.body.installation]);
    await page.reload();await page.getByPlaceholder('Search for agents...').fill('Senior Rust Developer');await expect(page.getByRole('article',{name:'Senior Rust Developer'}).getByRole('button',{name:'Installed',exact:true})).toHaveAttribute('aria-pressed','true');
  });
});
