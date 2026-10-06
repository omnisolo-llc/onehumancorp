// Keep this callback self-contained: Playwright evaluates it in the page realm.
// Next's accessibility announcer is not an application failure. Identify only
// its exact element inside its exact shadow host; retain all other global,
// portal, shadow-DOM and empty alerts so genuine errors still fail the test.
export function applicationAlertTexts(alerts: Element[]): string[] {
  return alerts.filter(alert => {
    const root = alert.getRootNode();
    return !(root instanceof ShadowRoot
      && root.host.localName === 'next-route-announcer'
      && alert.id === '__next-route-announcer__'
      && (alert.textContent ?? '') === '');
  }).map(alert => alert.textContent ?? '');
}
