// Blink — Focus Veil & Ambient Edge Logic (v1.4.0)

const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke || (async () => {});
const listen = window.__TAURI__?.event?.listen || (async () => () => {});
const getCurrentWindow = () => window.__TAURI__?.window?.getCurrentWindow?.() || {
  hide: async () => console.log('Mock hide veil')
};

// DOM Elements
const edgeFrame = document.getElementById('edgeFrame');
const veilBackdrop = document.getElementById('veilBackdrop');
const countdownSeconds = document.getElementById('countdownSeconds');
const veilTitle = document.getElementById('veilTitle');
const veilMessage = document.getElementById('veilMessage');
const exerciseCard = document.getElementById('exerciseCard');
const exerciseIcon = document.getElementById('exerciseIcon');
const exerciseInstruction = document.getElementById('exerciseInstruction');
const btnVeilSnooze = document.getElementById('btnVeilSnooze');
const btnVeilDismiss = document.getElementById('btnVeilDismiss');
const strictLockoutLabel = document.getElementById('strictLockoutLabel');
const lockoutSecondsDisplay = document.getElementById('lockoutSeconds');

let totalDuration = 20;
let remainingSeconds = 20;
let lockoutRemaining = 0;
let timerInterval = null;
let currentExerciseSet = 0;

// Rotating Guided Exercises
const EXERCISE_SETS = [
  [
    { icon: '←', text: 'Look far to the LEFT for 5 seconds' },
    { icon: '→', text: 'Look far to the RIGHT for 5 seconds' },
    { icon: '↑', text: 'Look UP towards the ceiling' },
    { icon: '↓', text: 'Look DOWN towards the floor' }
  ],
  [
    { icon: '🔄', text: 'Slowly roll your eyes in a circle clockwise' },
    { icon: '🔁', text: 'Slowly roll your eyes counter-clockwise' },
    { icon: '👀', text: 'Blink rapidly 5 times to lubricate your eyes' },
    { icon: '🧘', text: 'Close your eyes tightly and take a deep breath' }
  ],
  [
    { icon: '🔍', text: 'Focus on your thumb 10 inches away' },
    { icon: '🌳', text: 'Now look at something 20+ feet in the distance' },
    { icon: '🔍', text: 'Switch back to your thumb' },
    { icon: '✨', text: 'Relax your gaze into the distance' }
  ]
];

// Listen for mode events
listen('veil_mode', (event) => {
  const mode = event?.payload || 'veil';
  initMode(mode);
});

listen('break_message', (event) => {
  if (event?.payload && veilMessage) {
    veilMessage.textContent = event.payload;
  }
});

async function initMode(mode) {
  try {
    const cfg = await invoke('get_config');
    const timerState = await invoke('get_timer_state');

    totalDuration = cfg?.break_duration_seconds || 20;
    if (cfg?.timer_mode === 'pomodoro') {
      totalDuration = timerState?.is_long_break
        ? (cfg.pomodoro_long_break_minutes || 15) * 60
        : (cfg.pomodoro_short_break_minutes || 5) * 60;
    }
    remainingSeconds = totalDuration;

    // Apply Dimmer Opacity
    if (veilBackdrop && cfg?.veil_opacity) {
      veilBackdrop.style.background = `rgba(12, 12, 12, ${cfg.veil_opacity})`;
    }

    if (mode === 'edge') {
      if (edgeFrame) edgeFrame.style.display = 'block';
      if (veilBackdrop) veilBackdrop.style.display = 'none';
    } else {
      if (edgeFrame) edgeFrame.style.display = 'none';
      if (veilBackdrop) veilBackdrop.style.display = 'flex';
    }

    // Title for Pomodoro
    if (cfg?.timer_mode === 'pomodoro') {
      if (timerState?.is_long_break) {
        if (veilTitle) veilTitle.textContent = '🍅 Pomodoro Long Break';
      } else {
        if (veilTitle) veilTitle.textContent = `🍅 Pomodoro Cycle ${timerState?.current_cycle || 1} Break`;
      }
    } else {
      if (veilTitle) veilTitle.textContent = 'Rest Your Eyes (20-20-20)';
    }

    // Eye Exercise toggle
    if (exerciseCard) {
      exerciseCard.style.display = cfg?.eye_exercises_enabled !== false ? 'flex' : 'none';
    }

    // Strict Mode lockout
    if (cfg?.strict_mode_enabled) {
      lockoutRemaining = 10;
      setLockout(true);
    } else {
      setLockout(false);
    }

    startCountdown();
  } catch (err) {
    console.debug('Failed to init veil:', err);
    startCountdown();
  }
}

function setLockout(isLocked) {
  if (btnVeilDismiss) btnVeilDismiss.disabled = isLocked;
  if (btnVeilSnooze) btnVeilSnooze.disabled = isLocked;
  if (strictLockoutLabel) strictLockoutLabel.style.display = isLocked ? 'inline' : 'none';
}

function startCountdown() {
  if (timerInterval) clearInterval(timerInterval);

  currentExerciseSet = Math.floor(Math.random() * EXERCISE_SETS.length);
  updateExercisePrompt(0);

  timerInterval = setInterval(() => {
    remainingSeconds -= 1;
    if (countdownSeconds) countdownSeconds.textContent = Math.max(0, remainingSeconds);

    // Exercise rotation (every 5 seconds)
    const elapsed = totalDuration - remainingSeconds;
    const exerciseIdx = Math.floor(elapsed / 5) % EXERCISE_SETS[currentExerciseSet].length;
    updateExercisePrompt(exerciseIdx);

    // Strict lockout countdown
    if (lockoutRemaining > 0) {
      lockoutRemaining -= 1;
      if (lockoutSecondsDisplay) lockoutSecondsDisplay.textContent = lockoutRemaining;
      if (lockoutRemaining <= 0) {
        setLockout(false);
      }
    }

    if (remainingSeconds <= 0) {
      clearInterval(timerInterval);
      dismissVeil();
    }
  }, 1000);
}

function updateExercisePrompt(idx) {
  const ex = EXERCISE_SETS[currentExerciseSet][idx];
  if (ex && exerciseIcon && exerciseInstruction) {
    exerciseIcon.textContent = ex.icon;
    exerciseInstruction.textContent = ex.text;
  }
}

async function dismissVeil() {
  if (timerInterval) clearInterval(timerInterval);
  try {
    const win = getCurrentWindow();
    await win.hide();
  } catch (err) {
    console.error('Failed to hide veil:', err);
  }
}

// Button actions
btnVeilDismiss?.addEventListener('click', async () => {
  try {
    await invoke('record_break_action', { action: 'dismissed', durationSeconds: totalDuration });
    await invoke('reset_timer');
  } catch (e) {
    console.error(e);
  }
  dismissVeil();
});

btnVeilSnooze?.addEventListener('click', async () => {
  try {
    await invoke('record_break_action', { action: 'snoozed', durationSeconds: 0 });
    await invoke('snooze_timer');
  } catch (e) {
    console.error(e);
  }
  dismissVeil();
});

// Emergency Esc Key bypass
document.addEventListener('keydown', (e) => {
  if (e.key === 'Escape') {
    dismissVeil();
  }
});

document.addEventListener('DOMContentLoaded', () => {
  initMode('veil');
});
