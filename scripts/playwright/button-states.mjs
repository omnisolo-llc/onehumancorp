// Serialized into the browser by locator.evaluateHandle.
export function observeButtonStates(button) {
  const states = [];
  const observer = new button.ownerDocument.defaultView.MutationObserver(() => {
    states.push({ disabled: button.disabled, text: button.textContent });
  });
  observer.observe(button, {
    attributes: true,
    attributeFilter: ['disabled'],
    childList: true,
    characterData: true,
    subtree: true,
  });
  return { states, disconnect() { observer.disconnect(); } };
}
