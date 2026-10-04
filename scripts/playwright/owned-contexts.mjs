/** Keep action failures and every cleanup failure while disposing all owned contexts. */
export async function withOwnedBrowserContexts(browser, options, use) {
  const contexts = [];
  let result, failed = false, primary;
  try {
    contexts.push(await browser.newContext(options));
    contexts.push(await browser.newContext(options));
    result = await use(contexts);
  } catch (error) {
    failed = true; primary = error;
  }
  const closed = await Promise.allSettled(contexts.map(async context => context.close()));
  const cleanup = closed.filter(outcome => outcome.status === 'rejected').map(outcome => outcome.reason);
  if (cleanup.length) {
    if (failed) throw new AggregateError([primary, ...cleanup],
      `Owned checkout failed: ${primary instanceof Error ? primary.message : String(primary)}; context cleanup also failed`, { cause: primary });
    throw new AggregateError(cleanup, 'Owned checkout context cleanup failed');
  }
  if (failed) throw primary;
  return result;
}
