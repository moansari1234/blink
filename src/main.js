// Blink — Settings UI Logic (v1.3.0)

// Tauri invoke helper with fallback for web browser testing
const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke || (async (cmd, args) => {
  console.log(`[IPC Mock] ${cmd}`, args);
  if (cmd === 'get_config') {
    return {
      work_duration_minutes: 20,
      break_duration_seconds: 20,
      notification_style: 'toast',
      idle_detection_enabled: true,
      idle_threshold_seconds: 120,
      auto_start: true,
      sound_enabled: true,
      sound_volume: 0.5,
      snooze_duration_minutes: 5,
      respect_focus_assist: true,
      theme: 'system',
      break_message: 'Time for a 20-second break! Look at something 20 feet away.',
      hotkeys_enabled: true,
      overlay_monitor: 'primary',
      custom_sound_path: null,
      timer_mode: 'twentytwentytwenty',
      pomodoro_work_minutes: 25,
      pomodoro_short_break_minutes: 5,
      pomodoro_long_break_minutes: 15,
      pomodoro_cycles_before_long_break: 4,
      quiet_hours_enabled: false,
      quiet_hours_start: '12:00',
      quiet_hours_end: '13:00',
      quiet_hours_days: [1, 2, 3, 4, 5]
    };
  }
  if (cmd === 'get_timer_state') {
    return {
      remaining_seconds: 1200,
      formatted_time: '20:00',
      state: 'Running',
      is_paused: false,
      timer_mode: 'Standard',
      current_cycle: 1,
      is_long_break: false
    };
  }
  if (cmd === 'get_break_stats') {
    return {
      breaks_today: 6,
      breaks_this_week: 24,
      daily_average: 5.2,
      current_streak: 8,
      best_streak: 14,
      last_7_days: [
        { day: 'Fri', date: '2026-08-14', count: 4 },
        { day: 'Sat', date: '2026-08-15', count: 2 },
        { day: 'Sun', date: '2026-08-16', count: 1 },
        { day: 'Mon', date: '2026-08-17', count: 7 },
        { day: 'Tue', date: '2026-08-18', count: 6 },
        { day: 'Wed', date: '2026-08-19', count: 5 },
        { day: 'Thu', date: '2026-08-20', count: 6 }
      ]
    };
  }
  return {};
});

// State
let currentConfig = null;
let isTimerPaused = false;
let selectedCustomSound = null;
const CURRENT_VERSION = '1.3.0';

// DOM Elements
const tabButtons = document.querySelectorAll('.tab-btn');
const tabPanes = document.querySelectorAll('.tab-pane');
const headerCountdown = document.getElementById('headerCountdown');
const statusDot = document.getElementById('statusDot');
const pauseResumeBtn = document.getElementById('btnPauseResume');
const pauseResumeText = document.getElementById('pauseResumeText');
const pauseResumeIcon = document.getElementById('pauseResumeIcon');
const resetTimerBtn = document.getElementById('btnResetTimer');
const btnSaveConfig = document.getElementById('btnSaveConfig');
const btnResetDefaults = document.getElementById('btnResetDefaults');
const btnTestSound = document.getElementById('btnTestSound');
const btnTestNotification = document.getElementById('btnTestNotification');
const soundVolumeSlider = document.getElementById('soundVolume');
const volumeValueDisplay = document.getElementById('volumeValue');
const footerStatus = document.getElementById('footerStatus');
const idleEnabledSwitch = document.getElementById('idleEnabled');
const idleThresholdRow = document.getElementById('idleThresholdRow');
const themeSelect = document.getElementById('themeSelect');
const btnRefreshStats = document.getElementById('btnRefreshStats');

// v1.3.0 Elements
const mode202020 = document.getElementById('mode202020');
const modePomodoro = document.getElementById('modePomodoro');
const section202020 = document.getElementById('section202020');
const sectionPomodoro = document.getElementById('sectionPomodoro');
const customSoundDisplay = document.getElementById('customSoundDisplay');
const btnBrowseSound = document.getElementById('btnBrowseSound');
const btnClearSound = document.getElementById('btnClearSound');
const quietHoursEnabled = document.getElementById('quietHoursEnabled');
const quietHoursControls = document.getElementById('quietHoursControls');
const daySelector = document.getElementById('daySelector');
const btnExportConfig = document.getElementById('btnExportConfig');
const btnImportConfig = document.getElementById('btnImportConfig');
const confirmModal = document.getElementById('confirmModal');
const btnCancelReset = document.getElementById('btnCancelReset');
const btnConfirmReset = document.getElementById('btnConfirmReset');

// Theme Management
function applyTheme(theme) {
  if (theme === 'dark' || theme === 'light') {
    document.documentElement.setAttribute('data-theme', theme);
  } else {
    document.documentElement.removeAttribute('data-theme');
  }
}

themeSelect?.addEventListener('change', (e) => {
  applyTheme(e.target.value);
});

// Mode Toggle Handler (20-20-20 vs Pomodoro)
function updateModeSections(mode) {
  if (mode === 'pomodoro') {
    section202020.style.display = 'none';
    sectionPomodoro.style.display = 'block';
  } else {
    section202020.style.display = 'block';
    sectionPomodoro.style.display = 'none';
  }
}

mode202020?.addEventListener('change', () => updateModeSections('twentytwentytwenty'));
modePomodoro?.addEventListener('change', () => updateModeSections('pomodoro'));

// Custom Sound Picker
btnBrowseSound?.addEventListener('click', async () => {
  try {
    const pickedPath = await invoke('select_custom_sound');
    if (pickedPath) {
      selectedCustomSound = pickedPath;
      updateSoundUI(pickedPath);
      setStatus('Custom sound selected (click Save to apply)', 'normal');
    }
  } catch (err) {
    console.error('Failed to pick sound file:', err);
  }
});

btnClearSound?.addEventListener('click', () => {
  selectedCustomSound = null;
  updateSoundUI(null);
  setStatus('Reset to default bell chime (click Save to apply)', 'normal');
});

function updateSoundUI(path) {
  if (path) {
    const filename = path.split(/[\\/]/).pop();
    customSoundDisplay.textContent = `🎵 ${filename}`;
    customSoundDisplay.title = path;
    btnClearSound.style.display = 'inline-flex';
  } else {
    customSoundDisplay.textContent = 'Default: Built-in Bell Chime';
    customSoundDisplay.title = '';
    btnClearSound.style.display = 'none';
  }
}

// Quiet Hours Toggle
quietHoursEnabled?.addEventListener('change', (e) => {
  quietHoursControls.style.display = e.target.checked ? 'block' : 'none';
});

// Day Selector (M, T, W, T, F, S, S)
daySelector?.querySelectorAll('.day-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    btn.classList.toggle('active');
  });
});

function getSelectedDays() {
  const days = [];
  daySelector?.querySelectorAll('.day-btn.active').forEach(btn => {
    days.push(parseInt(btn.getAttribute('data-day'), 10));
  });
  return days.length > 0 ? days : [1, 2, 3, 4, 5];
}

function setSelectedDays(days) {
  const daySet = new Set(days || [1, 2, 3, 4, 5]);
  daySelector?.querySelectorAll('.day-btn').forEach(btn => {
    const dayNum = parseInt(btn.getAttribute('data-day'), 10);
    btn.classList.toggle('active', daySet.has(dayNum));
  });
}

// Config Export / Import
btnExportConfig?.addEventListener('click', async () => {
  try {
    const savedPath = await invoke('export_config');
    if (savedPath) {
      setStatus(`Exported settings to ${savedPath}`, 'success');
    }
  } catch (err) {
    console.error('Export config failed:', err);
    setStatus(`Export failed: ${err}`, 'error');
  }
});

btnImportConfig?.addEventListener('click', async () => {
  try {
    const importedCfg = await invoke('import_config');
    if (importedCfg) {
      currentConfig = importedCfg;
      populateForm(importedCfg);
      setStatus('Settings imported and applied successfully!', 'success');
    }
  } catch (err) {
    console.error('Import config failed:', err);
    setStatus(`Import failed: ${err}`, 'error');
  }
});

// Tab Navigation
tabButtons.forEach(btn => {
  btn.addEventListener('click', () => {
    tabButtons.forEach(b => {
      b.classList.remove('active');
      b.setAttribute('aria-selected', 'false');
    });
    tabPanes.forEach(p => p.classList.remove('active'));

    btn.classList.add('active');
    btn.setAttribute('aria-selected', 'true');
    const targetTab = btn.getAttribute('data-tab');
    document.getElementById(`tab-${targetTab}`)?.classList.add('active');

    if (targetTab === 'stats') {
      loadBreakStats();
    }
  });
});

// Sound Volume Slider
soundVolumeSlider.addEventListener('input', (e) => {
  const percent = Math.round(e.target.value * 100);
  volumeValueDisplay.textContent = `${percent}%`;
});

// Idle Toggle Nested visibility
idleEnabledSwitch.addEventListener('change', (e) => {
  idleThresholdRow.style.opacity = e.target.checked ? '1' : '0.4';
  idleThresholdRow.style.pointerEvents = e.target.checked ? 'auto' : 'none';
});

// Load Config from Backend
async function loadConfig() {
  try {
    currentConfig = await invoke('get_config');
    populateForm(currentConfig);
    setStatus('Settings loaded', 'normal');
  } catch (err) {
    console.error('Failed to load config:', err);
    setStatus('Failed to load config from backend', 'error');
  }
}

// Populate form controls from config object
function populateForm(cfg) {
  if (!cfg) return;

  // Mode
  const mode = cfg.timer_mode || 'twentytwentytwenty';
  if (mode === 'pomodoro') {
    modePomodoro.checked = true;
  } else {
    mode202020.checked = true;
  }
  updateModeSections(mode);

  // 20-20-20
  document.getElementById('workDuration').value = cfg.work_duration_minutes;
  document.getElementById('breakDuration').value = cfg.break_duration_seconds;

  // Pomodoro
  document.getElementById('pomodoroWork').value = cfg.pomodoro_work_minutes || 25;
  document.getElementById('pomodoroShortBreak').value = cfg.pomodoro_short_break_minutes || 5;
  document.getElementById('pomodoroLongBreak').value = cfg.pomodoro_long_break_minutes || 15;
  document.getElementById('pomodoroCycles').value = cfg.pomodoro_cycles_before_long_break || 4;

  // Custom Sound
  selectedCustomSound = cfg.custom_sound_path || null;
  updateSoundUI(selectedCustomSound);

  // Quiet Hours
  quietHoursEnabled.checked = !!cfg.quiet_hours_enabled;
  quietHoursControls.style.display = cfg.quiet_hours_enabled ? 'block' : 'none';
  document.getElementById('quietStart').value = cfg.quiet_hours_start || '12:00';
  document.getElementById('quietEnd').value = cfg.quiet_hours_end || '13:00';
  setSelectedDays(cfg.quiet_hours_days);

  // General & Notifications
  document.getElementById('snoozeDuration').value = cfg.snooze_duration_minutes;
  document.getElementById('soundEnabled').checked = cfg.sound_enabled;
  document.getElementById('soundVolume').value = cfg.sound_volume;
  volumeValueDisplay.textContent = `${Math.round(cfg.sound_volume * 100)}%`;

  const styleRadio = document.querySelector(`input[name="notificationStyle"][value="${cfg.notification_style}"]`);
  if (styleRadio) styleRadio.checked = true;

  document.getElementById('breakMessage').value = cfg.break_message || '';
  document.getElementById('overlayMonitor').value = cfg.overlay_monitor || 'primary';
  document.getElementById('themeSelect').value = cfg.theme || 'system';
  document.getElementById('hotkeysEnabled').checked = cfg.hotkeys_enabled !== false;
  applyTheme(cfg.theme || 'system');

  // Behavior
  document.getElementById('idleEnabled').checked = cfg.idle_detection_enabled;
  document.getElementById('idleThreshold').value = cfg.idle_threshold_seconds;
  document.getElementById('focusAssistEnabled').checked = cfg.respect_focus_assist;
  document.getElementById('autoStartEnabled').checked = cfg.auto_start;

  idleThresholdRow.style.opacity = cfg.idle_detection_enabled ? '1' : '0.4';
  idleThresholdRow.style.pointerEvents = cfg.idle_detection_enabled ? 'auto' : 'none';
}

// Read form controls into config object
function readForm() {
  const selectedStyle = document.querySelector('input[name="notificationStyle"]:checked')?.value || 'toast';
  const selectedMode = document.querySelector('input[name="timerMode"]:checked')?.value || 'twentytwentytwenty';

  return {
    work_duration_minutes: Math.max(1, parseInt(document.getElementById('workDuration').value, 10) || 20),
    break_duration_seconds: Math.max(5, parseInt(document.getElementById('breakDuration').value, 10) || 20),
    notification_style: selectedStyle,
    idle_detection_enabled: document.getElementById('idleEnabled').checked,
    idle_threshold_seconds: Math.max(30, parseInt(document.getElementById('idleThreshold').value, 10) || 120),
    auto_start: document.getElementById('autoStartEnabled').checked,
    sound_enabled: document.getElementById('soundEnabled').checked,
    sound_volume: parseFloat(document.getElementById('soundVolume').value) || 0.5,
    snooze_duration_minutes: Math.max(1, parseInt(document.getElementById('snoozeDuration').value, 10) || 5),
    respect_focus_assist: document.getElementById('focusAssistEnabled').checked,
    theme: document.getElementById('themeSelect').value || 'system',
    break_message: document.getElementById('breakMessage').value.trim() || 'Time for a 20-second break! Look at something 20 feet away.',
    hotkeys_enabled: document.getElementById('hotkeysEnabled').checked,
    overlay_monitor: document.getElementById('overlayMonitor').value || 'primary',
    custom_sound_path: selectedCustomSound,
    timer_mode: selectedMode,
    pomodoro_work_minutes: Math.max(1, parseInt(document.getElementById('pomodoroWork').value, 10) || 25),
    pomodoro_short_break_minutes: Math.max(1, parseInt(document.getElementById('pomodoroShortBreak').value, 10) || 5),
    pomodoro_long_break_minutes: Math.max(1, parseInt(document.getElementById('pomodoroLongBreak').value, 10) || 15),
    pomodoro_cycles_before_long_break: Math.max(1, parseInt(document.getElementById('pomodoroCycles').value, 10) || 4),
    quiet_hours_enabled: quietHoursEnabled.checked,
    quiet_hours_start: document.getElementById('quietStart').value || '12:00',
    quiet_hours_end: document.getElementById('quietEnd').value || '13:00',
    quiet_hours_days: getSelectedDays()
  };
}

// Save Config
async function saveConfig() {
  const config = readForm();
  try {
    await invoke('save_config', { config });
    currentConfig = config;
    applyTheme(config.theme);
    setStatus('Settings saved & applied immediately', 'success');
  } catch (err) {
    console.error('Failed to save config:', err);
    setStatus(`Error: ${err}`, 'error');
  }
}

// Reset Defaults Modal Flow
btnResetDefaults.addEventListener('click', () => {
  confirmModal.style.display = 'flex';
});

btnCancelReset.addEventListener('click', () => {
  confirmModal.style.display = 'none';
});

btnConfirmReset.addEventListener('click', () => {
  confirmModal.style.display = 'none';
  const defaults = {
    work_duration_minutes: 20,
    break_duration_seconds: 20,
    notification_style: 'toast',
    idle_detection_enabled: true,
    idle_threshold_seconds: 120,
    auto_start: true,
    sound_enabled: true,
    sound_volume: 0.5,
    snooze_duration_minutes: 5,
    respect_focus_assist: true,
    theme: 'system',
    break_message: 'Time for a 20-second break! Look at something 20 feet away.',
    hotkeys_enabled: true,
    overlay_monitor: 'primary',
    custom_sound_path: null,
    timer_mode: 'twentytwentytwenty',
    pomodoro_work_minutes: 25,
    pomodoro_short_break_minutes: 5,
    pomodoro_long_break_minutes: 15,
    pomodoro_cycles_before_long_break: 4,
    quiet_hours_enabled: false,
    quiet_hours_start: '12:00',
    quiet_hours_end: '13:00',
    quiet_hours_days: [1, 2, 3, 4, 5]
  };
  populateForm(defaults);
  setStatus('Reset to default values (click Save to apply)', 'normal');
});

// Helper: Status message in footer
function setStatus(text, type = 'normal') {
  footerStatus.innerHTML = `<span class="status-text ${type}">${text}</span>`;
  if (type === 'success') {
    setTimeout(() => {
      footerStatus.innerHTML = '<span class="status-text">Ready</span>';
    }, 4000);
  }
}

// Timer Polling & Control
async function updateTimerStatus() {
  try {
    const state = await invoke('get_timer_state');
    if (state) {
      headerCountdown.textContent = state.formatted_time || '--:--';
      isTimerPaused = state.is_paused;

      // Update badge dot
      statusDot.className = 'status-dot';
      if (state.state === 'Running') {
        statusDot.classList.add('green');
      } else if (state.state === 'PausedIdle' || state.state === 'PausedManual' || state.state === 'PausedQuietHours') {
        statusDot.classList.add('yellow');
      } else if (state.state === 'OnBreak') {
        statusDot.classList.add('red');
      }

      // Update Pause button
      if (state.is_paused) {
        pauseResumeIcon.textContent = '▶';
        pauseResumeText.textContent = 'Resume Timer';
      } else {
        pauseResumeIcon.textContent = '⏸';
        pauseResumeText.textContent = 'Pause Timer';
      }
    }
  } catch (err) {
    console.debug('Timer status poll failed:', err);
  }
}

// Stats & Chart Rendering
async function loadBreakStats() {
  try {
    const stats = await invoke('get_break_stats');
    if (!stats) return;

    document.getElementById('statToday').textContent = stats.breaks_today ?? 0;
    document.getElementById('statWeek').textContent = stats.breaks_this_week ?? 0;
    document.getElementById('statAvg').textContent = (stats.daily_average ?? 0).toFixed(1);
    document.getElementById('statStreak').textContent = `${stats.current_streak ?? 0}`;

    drawStatsChart(stats.last_7_days || []);
  } catch (err) {
    console.error('Failed to load break stats:', err);
  }
}

btnRefreshStats?.addEventListener('click', loadBreakStats);

function drawStatsChart(days) {
  const canvas = document.getElementById('statsChart');
  if (!canvas) return;

  const ctx = canvas.getContext('2d');
  const dpr = window.devicePixelRatio || 1;
  const rect = canvas.getBoundingClientRect();

  canvas.width = (rect.width || 380) * dpr;
  canvas.height = (rect.height || 130) * dpr;
  ctx.scale(dpr, dpr);

  const width = rect.width || 380;
  const height = rect.height || 130;

  ctx.clearRect(0, 0, width, height);

  if (!days || days.length === 0) {
    ctx.fillStyle = '#888888';
    ctx.font = '12px Segoe UI, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText('No history data available yet.', width / 2, height / 2);
    return;
  }

  const paddingBottom = 22;
  const paddingTop = 20;
  const chartHeight = height - paddingBottom - paddingTop;
  const maxCount = Math.max(5, ...days.map(d => d.count || 0));

  const numBars = days.length;
  const barWidth = Math.min(32, (width - (numBars + 1) * 12) / numBars);
  const totalSpacing = width - (numBars * barWidth);
  const gap = totalSpacing / (numBars + 1);

  const computed = getComputedStyle(document.documentElement);
  const accentColor = computed.getPropertyValue('--accent').trim() || '#60cdff';
  const textSecColor = computed.getPropertyValue('--text-secondary').trim() || '#888888';

  // Grid line
  ctx.strokeStyle = 'rgba(128, 128, 128, 0.15)';
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(gap / 2, height - paddingBottom);
  ctx.lineTo(width - gap / 2, height - paddingBottom);
  ctx.stroke();

  // Bars
  days.forEach((dayData, index) => {
    const x = gap + index * (barWidth + gap);
    const count = dayData.count || 0;
    const barHeight = Math.max(4, (count / maxCount) * chartHeight);
    const y = height - paddingBottom - barHeight;

    const gradient = ctx.createLinearGradient(0, y, 0, height - paddingBottom);
    gradient.addColorStop(0, accentColor);
    gradient.addColorStop(1, accentColor + '99');

    ctx.fillStyle = gradient;
    roundRect(ctx, x, y, barWidth, barHeight, 4);
    ctx.fill();

    ctx.fillStyle = accentColor;
    ctx.font = 'bold 10px Segoe UI, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(count > 0 ? `${count}` : '0', x + barWidth / 2, y - 4);

    ctx.fillStyle = textSecColor;
    ctx.font = '10px Segoe UI, sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText(dayData.day || '', x + barWidth / 2, height - 6);
  });
}

function roundRect(ctx, x, y, width, height, radius) {
  ctx.beginPath();
  ctx.moveTo(x + radius, y);
  ctx.lineTo(x + width - radius, y);
  ctx.quadraticCurveTo(x + width, y, x + width, y + radius);
  ctx.lineTo(x + width, y + height);
  ctx.lineTo(x, y + height);
  ctx.lineTo(x, y + radius);
  ctx.quadraticCurveTo(x, y, x + radius, y);
  ctx.closePath();
}

// Event Listeners
btnSaveConfig.addEventListener('click', saveConfig);

btnTestSound.addEventListener('click', async () => {
  const volume = parseFloat(document.getElementById('soundVolume').value) || 0.5;
  try {
    await invoke('test_sound', { volume, customPath: selectedCustomSound });
    setStatus('Playing chime preview...', 'normal');
  } catch (err) {
    console.error('Test sound error:', err);
  }
});

btnTestNotification.addEventListener('click', async () => {
  try {
    await invoke('test_notification');
    setStatus('Dispatched test reminder', 'normal');
  } catch (err) {
    console.error('Test notification error:', err);
  }
});

pauseResumeBtn.addEventListener('click', async () => {
  try {
    if (isTimerPaused) {
      await invoke('resume_timer');
    } else {
      await invoke('pause_timer');
    }
    updateTimerStatus();
  } catch (err) {
    console.error('Toggle timer failed:', err);
  }
});

resetTimerBtn.addEventListener('click', async () => {
  try {
    await invoke('reset_timer');
    updateTimerStatus();
    setStatus('Timer reset', 'normal');
  } catch (err) {
    console.error('Reset timer failed:', err);
  }
});

// Open URL helper
window.openUrl = async function(url) {
  try {
    await invoke('open_url', { url });
  } catch (e) {
    window.open(url, '_blank');
  }
};

// Check for Updates
const btnCheckUpdates = document.getElementById('btnCheckUpdates');
const updateResult = document.getElementById('updateResult');

function compareVersions(v1, v2) {
  const p1 = v1.split('.').map(n => parseInt(n, 10) || 0);
  const p2 = v2.split('.').map(n => parseInt(n, 10) || 0);
  for (let i = 0; i < Math.max(p1.length, p2.length); i++) {
    const num1 = p1[i] || 0;
    const num2 = p2[i] || 0;
    if (num1 > num2) return 1;
    if (num1 < num2) return -1;
  }
  return 0;
}

function escapeHtml(str) {
  return str.replace(/[&<>'"]/g, 
    tag => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', "'": '&#39;', '"': '&quot;' }[tag] || tag)
  );
}

btnCheckUpdates?.addEventListener('click', async () => {
  btnCheckUpdates.disabled = true;
  btnCheckUpdates.textContent = 'Checking GitHub...';
  updateResult.innerHTML = '<span class="update-checking">⏳ Fetching latest release info from GitHub...</span>';

  try {
    const res = await fetch('https://api.github.com/repos/moansari1234/blink/releases/latest', {
      headers: { 'Accept': 'application/vnd.github.v3+json' }
    });

    if (!res.ok) {
      throw new Error(`GitHub API returned HTTP ${res.status}`);
    }

    const data = await res.json();
    const latestTag = data.tag_name || '';
    const latestVersion = latestTag.replace(/^v/, '');

    if (compareVersions(latestVersion, CURRENT_VERSION) > 0) {
      const msiAsset = data.assets?.find(a => a.name.endsWith('.msi')) || data.assets?.[0];
      const downloadUrl = msiAsset?.browser_download_url || data.html_url;

      updateResult.innerHTML = `
        <div class="update-available-banner">
          <div class="update-available-header">
            <span class="update-badge new">New Update: v${escapeHtml(latestVersion)}</span>
          </div>
          <p class="update-notes-body">${escapeHtml(data.name || 'New version available with improvements.')}</p>
          <div class="update-download-actions">
            <button class="fluent-button primary small" onclick="openUrl('${downloadUrl}')">
              ⬇️ Download Update (.msi)
            </button>
            <button class="fluent-button secondary small" onclick="openUrl('${data.html_url}')">
              View on GitHub
            </button>
          </div>
        </div>
      `;
    } else {
      updateResult.innerHTML = `
        <div class="update-uptodate">
          <span>✅ You are using the latest version of Blink (v${CURRENT_VERSION}).</span>
        </div>
      `;
    }
  } catch (err) {
    updateResult.innerHTML = `
      <div class="update-error">
        <span>Failed to check for updates: ${escapeHtml(err.message)}</span>
        <div>
          <button class="fluent-button secondary small" onclick="openUrl('https://github.com/moansari1234/blink/releases')">
            Visit GitHub Releases
          </button>
        </div>
      </div>
    `;
  } finally {
    btnCheckUpdates.disabled = false;
    btnCheckUpdates.textContent = '🔄 Check for Updates';
  }
});

// Initialization
document.addEventListener('DOMContentLoaded', () => {
  loadConfig();
  updateTimerStatus();
  setInterval(updateTimerStatus, 1000);
});
