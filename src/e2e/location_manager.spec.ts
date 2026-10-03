import { randomUUID } from 'node:crypto';
import { test, expect } from './fixtures';
import { createLinkBioActor } from './link_bio_owner';
import { e2eDbQuery } from './db_utils';

test.describe('Recorded location data and explicit escalation availability', () => {
  test('a verified empty owner does not receive sample staff, tasks or complaints', async ({page,baseURL,adminUser}) => {
    await createLinkBioActor(page,baseURL,adminUser);
    const pending=page.waitForResponse(response=>new URL(response.url()).pathname==='/api/v1/location/dashboard');
    await page.goto('/location-dashboard');const response=await pending;
    expect(response.status()).toBe(200);expect(await response.json()).toEqual({tasks:[],alerts:[],staff:[]});
    await expect(page.getByText('No recorded staff summaries.')).toBeVisible();
    await expect(page.getByText('Alice',{exact:true})).toHaveCount(0);
    await expect(page.getByText('Restock coffee beans',{exact:true})).toHaveCount(0);
    await expect(page.getByRole('button',{name:'Escalate to Owner'})).toHaveCount(0);
  });

  test('reads actual owned records and retains them when no escalation delivery is implemented', async ({page,baseURL,adminUser}) => {
    const actor=await createLinkBioActor(page,baseURL,adminUser);
    const staffId=randomUUID(),taskId=randomUUID(),summaryId=randomUUID();
    await e2eDbQuery('INSERT INTO ohc_staff_member(id,tenant_id,name,phone_number,role) VALUES($1,$2,$3,$4,$5)',[staffId,actor.tenantId,'Recorded location manager','+15555550123','Manager']);
    await e2eDbQuery('INSERT INTO staff_tasks(id,tenant_id,staff_id,title,description,status,priority) VALUES($1,$2,$3,$4,$5,$6,$7)',[taskId,actor.tenantId,staffId,'Recorded task','Verify the recorded stock count','pending','medium']);
    await e2eDbQuery('INSERT INTO shift_summaries(id,tenant_id,shift_date,summary_text) VALUES($1,$2,CURRENT_DATE,$3)',[summaryId,actor.tenantId,'Manager recorded a supply question for review.']);
    const pending=page.waitForResponse(response=>new URL(response.url()).pathname==='/api/v1/location/dashboard');
    await page.goto('/location-dashboard');const response=await pending;expect(response.status()).toBe(200);
    const data=await response.json();expect(data.tasks.map((row:{id:string})=>row.id)).toEqual([taskId]);expect(data.staff.map((row:{id:string})=>row.id)).toEqual([staffId]);expect(data.alerts.map((row:{id:string})=>row.id)).toEqual([summaryId]);
    await expect(page.locator('#dashboard-title')).toHaveText('Location Dashboard');
    await expect(page.locator('#dashboard-title')).toBeVisible();
    await expect(page.getByText('Recorded task', {exact:true})).toBeVisible();
    await expect(page.getByText('Recorded location manager')).toBeVisible();
    await expect(page.getByText('Shift status unavailable')).toBeVisible();
    const drafting=page.waitForResponse(response=>new URL(response.url()).pathname==='/api/v1/agent/draft-escalation');
    await page.getByRole('button',{name:'Escalate to Owner'}).click();
    expect([404,501,503]).toContain((await drafting).status());
    await expect(page.getByRole('alert').filter({hasText:'No escalation draft was confirmed'})).toBeVisible();
    const draft=page.getByRole('textbox',{name:'Escalation draft'});await expect(draft).toHaveValue('');
    await draft.fill('Please review the recorded supply question.');
    const sending=page.waitForResponse(response=>new URL(response.url()).pathname==='/api/v1/location/escalate');
    await page.getByRole('button',{name:'Send to Owner'}).click();expect((await sending).status()).toBe(501);
    await expect(page.getByRole('alert').filter({hasText:'No escalation was confirmed'})).toBeVisible();
    await expect(draft).toHaveValue('Please review the recorded supply question.');
    await expect(page.getByText('Manager recorded a supply question for review.')).toBeVisible();
    const stored=await e2eDbQuery('SELECT summary_text,escalations FROM shift_summaries WHERE id=$1 AND tenant_id=$2',[summaryId,actor.tenantId]);
    expect(stored).toEqual([{summary_text:'Manager recorded a supply question for review.',escalations:null}]);
  });
});
