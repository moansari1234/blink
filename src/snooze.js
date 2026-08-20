// Blink — Snooze Overlay Logic

const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke || (async () => {});
const getCurrentWindow = () => window.__TAURI__?.window?.getCurrentWindow?.() || {
  hide: async () => console.log('Mock hide overlay'),
  close: async () => console.log('Mock close overlay')
};

let autoDismissTimeout = 30; // 30 seconds auto-dismiss
let remaining = autoDismissTimeout;
const bar = document.getElementById('countdownBar');

const interval = setInterval(async () => {
  remaining -= 0.1;
  const percentage = Math.max(0, (remaining / autoDismissTimeout) * 100);
  if (bar) bar.style.width = `${percentage}%`;

  if (remaining <= 0) {
    clearInterval(interval);
    dismissOverlay();
  }
}, 100);

async function dismissOverlay() {
  try {
    const win = getCurrentWindow();
    await win.hide();
  } catch (e) {
    console.error('Failed to hide snooze window', e);
  }
}

document.getElementById('btnDismiss')?.addEventListener('click', async () => {
  clearInterval(interval);
  try {
    await invoke('reset_timer');
  } catch (e) {
    console.error('Reset timer failed', e);
  }
  dismissOverlay();
});

document.getElementById('btnSnooze')?.addEventListener('click', async () => {
  clearInterval(interval);
  try {
    await invoke('snooze_timer');
  } catch (e) {
    console.error('Snooze timer failed', e);
  }
  dismissOverlay();
});
