import os
with open("src/ui/next/src/app/triage/page.tsx", "r") as f:
    c = f.read()
c = c.replace("          : data.items", "          : data.items.map((i: any) => ({ ...i, priority: 'High', source: 'Instagram DM', context: i.intent, action_type: Array.isArray(i.suggested_actions) && i.suggested_actions.length > 0 ? i.suggested_actions[0].action_type : 'No action', action_payload: Array.isArray(i.suggested_actions) && i.suggested_actions.length > 0 ? i.suggested_actions[0].message : JSON.stringify(i.suggested_actions) }))")
c = c.replace('data-testid="approve-btn"', 'data-testid={`triage-approve-${item.id}`}')
c = c.replace('data-testid="dismiss-btn"', 'data-testid={`triage-dismiss-${item.id}`}')
with open("src/ui/next/src/app/triage/page.tsx", "w") as f:
    f.write(c)
