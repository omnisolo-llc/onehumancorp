import { test, expect } from './fixtures';
import { createLinkBioActor } from './link_bio_owner';

test.describe('Private Link in Bio Generator', () => {
  test('saves and reloads the exact owner configuration without a publication claim',async({page,anonymousPage,baseURL,adminUser})=>{
    const actor=await createLinkBioActor(page,baseURL,adminUser);
    await page.goto('/dashboard');await page.getByRole('link',{name:/Link in Bio Generator/i}).click();await expect(page).toHaveURL(/\/link-in-bio-generator/);
    await page.getByRole('textbox',{name:'Store / Creator Name'}).fill('My Awesome Creator Store');
    await page.getByRole('textbox',{name:'Bio / Description'}).fill('This is a test bio description.');
    await page.getByRole('button',{name:'Dark',exact:true}).click();
    await page.getByRole('button',{name:'+ Add Link',exact:true}).click();
    await page.getByPlaceholder('Link Title (e.g. Shop My Collection)').fill('My Custom Link');await page.getByPlaceholder('URL (e.g. https://...)').fill('https://example.com/shop');
    const pending=page.waitForResponse(response=>new URL(response.url()).pathname==='/api/v1/growth/link-in-bio' && response.request().method()==='POST');
    await page.getByRole('button',{name:'Save private configuration',exact:true}).click();
    const response=await pending;expect(response.status()).toBe(200);expect(await response.text()).toBe('');
    expect(response.request().postDataJSON().tenant_id).toBe(actor.tenantId);expect(response.request().headers()['x-ohc-expected-user']).toBe(actor.userId);
    await expect(page.getByRole('status',{name:'Private profile status'})).toContainText('Saved private configuration');
    await expect(page.getByRole('status',{name:'Private profile status'})).toContainText('Public publication is not available');
    await page.goto(`/bio/${encodeURIComponent(actor.tenantId)}`);
    await expect(page.getByRole('heading',{name:'My Awesome Creator Store'})).toBeVisible();await expect(page.getByText('This is a test bio description.')).toBeVisible();
    const link=page.getByRole('link',{name:'My Custom Link'});await expect(link).toBeVisible();await expect(link).toHaveAttribute('href','https://example.com/shop');
    const poweredBy=page.getByRole('link',{name:'⚡ OmniSolo'});await expect(poweredBy).toBeVisible();
    const href=await poweredBy.getAttribute('href');expect(new URL(href!,baseURL).searchParams.get('ref')).toBe(`linkinbio_${actor.tenantId}`);
    const denied=await anonymousPage.request.get(new URL(`/api/v1/growth/link-in-bio/${encodeURIComponent(actor.tenantId)}`,baseURL).href);
    expect(denied.status()).toBe(401);
    expect(await denied.text()).not.toContain('My Awesome Creator Store');
    await anonymousPage.goto(`/bio/${encodeURIComponent(actor.tenantId)}`);
    await expect(anonymousPage).toHaveURL(/\/login(?:\?|$)/);
    await expect(anonymousPage.getByRole('heading',{name:'My Awesome Creator Store'})).toHaveCount(0);
    await expect(anonymousPage.getByText('This is a test bio description.',{exact:true})).toHaveCount(0);
  });
});
