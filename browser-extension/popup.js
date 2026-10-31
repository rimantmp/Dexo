// Downbyte Extension Popup Controller

document.addEventListener('DOMContentLoaded', () => {
  const urlInput = document.getElementById('url-input');
  const btnDownload = document.getElementById('btn-download');
  const toggleCatch = document.getElementById('toggle-catch');
  const extInput = document.getElementById('ext-input');
  const btnSave = document.getElementById('btn-save');
  const statusIndicator = document.getElementById('status-indicator');
  const statusText = document.getElementById('status-text');

  // Load existing configuration
  chrome.storage.local.get(['catchDownloads', 'monitoredExtensions'], (res) => {
    toggleCatch.checked = res.catchDownloads !== false;
    extInput.value = res.monitoredExtensions || 'zip, rar, 7z, iso, exe, msi, mp4, mkv, pdf';
  });

  // Check connectivity to Downbyte
  chrome.runtime.sendMessage({ action: 'check_connection' }, (response) => {
    if (response && response.connected) {
      statusIndicator.className = 'status-pill online';
      statusText.textContent = 'Connected';
    } else {
      statusIndicator.className = 'status-pill offline';
      statusText.textContent = 'Desktop Ready';
    }
  });

  // Handle Quick Download
  btnDownload.addEventListener('click', () => {
    const rawUrl = urlInput.value.trim();
    if (!rawUrl) return;

    chrome.runtime.sendMessage({
      action: 'send_url',
      payload: {
        url: rawUrl,
        referrer: ''
      }
    }, () => {
      urlInput.value = '';
      window.close();
    });
  });

  // Save Settings
  btnSave.addEventListener('click', () => {
    chrome.storage.local.set({
      catchDownloads: toggleCatch.checked,
      monitoredExtensions: extInput.value.trim()
    }, () => {
      btnSave.textContent = 'Saved!';
      setTimeout(() => {
        btnSave.textContent = 'Save Settings';
      }, 1200);
    });
  });
});

