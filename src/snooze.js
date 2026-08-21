// Blink — Snooze Overlay Logic (v1.3.0)

const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke || (async () => {});
const listen = window.__TAURI__?.event?.listen || (async () => () => {});
const getCurrentWindow = () => window.__TAURI__?.window?.getCurrentWindow?.() || {
  hide: async () => console.log('Mock hide overlay'),
  close: async () => console.log('Mock close overlay')
};

let autoDismissTimeout = 30; // 30 seconds auto-dismiss
let remaining = autoDismissTimeout;
const bar = document.getElementById('countdownBar');
const breakSubtitle = document.getElementById('breakSubtitle');
const snoozeBtnLabel = document.getElementById('snoozeBtnLabel');
const snoozeCard = document.getElementById('snoozeCard');
const snoozeTitle = document.getElementById('snoozeTitle');
const snoozeIcon = document.getElementById('snoozeIcon');

// 1. Listen for custom break messages from backend
listen('break_message', (event) => {
  if (event?.payload && breakSubtitle) {
    breakSubtitle.textContent = event.payload;
  }
});

// 2. Fetch config & timer state to update theme, mode, and snooze duration label
async function loadOverlayConfig() {
  try {
    const cfg = await invoke('get_config');
    const timerState = await invoke('get_timer_state');

    if (cfg) {
      // Theme
      if (cfg.theme === 'dark' || cfg.theme === 'light') {
        document.documentElement.setAttribute('data-theme', cfg.theme);
      } else {
        document.documentElement.removeAttribute('data-theme');
      }

      // Snooze label
      if (snoozeBtnLabel && cfg.snooze_duration_minutes) {
        snoozeBtnLabel.textContent = `💤 Snooze (${cfg.snooze_duration_minutes}m)`;
      }

      // Mode-specific labels
      if (cfg.timer_mode === 'pomodoro') {
        if (timerState?.is_long_break) {
          if (snoozeTitle) snoozeTitle.textContent = '🍅 Pomodoro: Long Break!';
          if (snoozeIcon) snoozeIcon.textContent = '🧘';
          if (snoozeCard) snoozeCard.classList.add('long-break');
        } else {
          if (snoozeTitle) snoozeTitle.textContent = `🍅 Pomodoro: Cycle ${timerState?.current_cycle || 1} Break`;
          if (snoozeIcon) snoozeIcon.textContent = '🍅';
        }
      } else {
        if (snoozeTitle) snoozeTitle.textContent = 'Time for an eye break!';
        if (snoozeIcon) snoozeIcon.textContent = '👁';
      }

      // Message fallback
      if (breakSubtitle && cfg.break_message && breakSubtitle.textContent.includes('20-20-20')) {
        const firstMsg = cfg.break_message.split('|')[0].trim();
        if (firstMsg) breakSubtitle.textContent = firstMsg;
      }
    }
  } catch (e) {
    console.debug('Failed to load overlay config:', e);
  }
}

// 3. Countdown timer
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
  if (snoozeCard) {
    snoozeCard.classList.add('dismissing');
  }
  setTimeout(async () => {
    try {
      const win = getCurrentWindow();
      await win.hide();
      if (snoozeCard) snoozeCard.classList.remove('dismissing');
      remaining = autoDismissTimeout;
    } catch (e) {
      console.error('Failed to hide snooze window', e);
    }
  }, 220);
}

// 4. Button Handlers
document.getElementById('btnDismiss')?.addEventListener('click', async () => {
  clearInterval(interval);
  try {
    await invoke('record_break_action', { action: 'dismissed', durationSeconds: 20 });
    await invoke('reset_timer');
  } catch (e) {
    console.error('Reset timer failed', e);
  }
  dismissOverlay();
});

document.getElementById('btnSnooze')?.addEventListener('click', async () => {
  clearInterval(interval);
  try {
    await invoke('record_break_action', { action: 'snoozed', durationSeconds: 0 });
    await invoke('snooze_timer');
  } catch (e) {
    console.error('Snooze timer failed', e);
  }
  dismissOverlay();
});

// Initialization
document.addEventListener('DOMContentLoaded', () => {
  loadOverlayConfig();
});
