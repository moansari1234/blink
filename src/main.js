// Blink — Settings UI Logic

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
      respect_focus_assist: true
    };
  }
  if (cmd === 'get_timer_state') {
    return {
      remaining_seconds: 1200,
      formatted_time: '20:00',
      state: 'Running',
      is_paused: false
    };
  }
  return {};
});

// State
let currentConfig = null;
let isTimerPaused = false;

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

  document.getElementById('workDuration').value = cfg.work_duration_minutes;
  document.getElementById('breakDuration').value = cfg.break_duration_seconds;
  document.getElementById('snoozeDuration').value = cfg.snooze_duration_minutes;
  document.getElementById('soundEnabled').checked = cfg.sound_enabled;
  document.getElementById('soundVolume').value = cfg.sound_volume;
  volumeValueDisplay.textContent = `${Math.round(cfg.sound_volume * 100)}%`;

  // Notification style radio
  const styleRadio = document.querySelector(`input[name="notificationStyle"][value="${cfg.notification_style}"]`);
  if (styleRadio) styleRadio.checked = true;

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
    respect_focus_assist: document.getElementById('focusAssistEnabled').checked
  };
}

// Save Config
async function saveConfig() {
  const config = readForm();
  try {
    await invoke('save_config', { config });
    currentConfig = config;
    setStatus('Settings saved & applied immediately', 'success');
  } catch (err) {
    console.error('Failed to save config:', err);
    setStatus(`Error: ${err}`, 'error');
  }
}

// Reset Defaults
function resetDefaults() {
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
    respect_focus_assist: true
  };
  populateForm(defaults);
  setStatus('Reset to default values (click Save to apply)', 'normal');
}

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
      } else if (state.state === 'PausedIdle' || state.state === 'PausedManual') {
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

// Event Listeners
btnSaveConfig.addEventListener('click', saveConfig);
btnResetDefaults.addEventListener('click', resetDefaults);

btnTestSound.addEventListener('click', async () => {
  const volume = parseFloat(document.getElementById('soundVolume').value) || 0.5;
  try {
    await invoke('test_sound', { volume });
    setStatus('Playing test chime...', 'normal');
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
    setStatus('Timer reset to interval start', 'normal');
  } catch (err) {
    console.error('Reset timer failed:', err);
  }
});

// Open URL helper (IPC to Rust or window.open)
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
const CURRENT_VERSION = '1.0.1';

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
      // Found newer version!
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
