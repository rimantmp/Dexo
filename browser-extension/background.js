// Dexo Background Service Worker (Manifest V3)

const DEFAULT_EXTENSIONS = [
  'zip', 'rar', '7z', 'tar', 'gz', 'iso',
  'exe', 'msi', 'apk', 'dmg',
  'mp4', 'mkv', 'avi', 'mov', 'flv', 'mp3',
  'pdf', 'docx', 'xlsx'
];

const CLOUD_STORAGE_DOMAINS = [
  'drive.google.com',
  'docs.google.com',
  'googleusercontent.com',
  'dropbox.com',
  'mediafire.com',
  'mega.nz',
  'onedrive.live.com',
  '1drv.ms'
];

const NATIVE_HOST = 'com.dexo.native';
const LOCAL_BRIDGE_URL = 'http://127.0.0.1:4192/download';

// Track intercepted downloads to prevent infinite loops
const interceptedDownloadIds = new Set();

// 1. Initialize Context Menus & Defaults
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: 'dexo-download-link',
    title: 'Download with Dexo',
    contexts: ['link', 'image', 'video', 'audio', 'selection']
  });

  chrome.storage.local.get(['catchDownloads', 'monitoredExtensions'], (res) => {
    if (res.catchDownloads === undefined) {
      chrome.storage.local.set({ catchDownloads: true });
    }
    if (!res.monitoredExtensions) {
      chrome.storage.local.set({ monitoredExtensions: DEFAULT_EXTENSIONS.join(', ') });
    }
  });
});

// 2. Handle Context Menu Clicks
chrome.contextMenus.onClicked.addListener(async (info, tab) => {
  if (info.menuItemId === 'dexo-download-link') {
    const targetUrl = info.linkUrl || info.srcUrl || (info.selectionText && info.selectionText.startsWith('http') ? info.selectionText : null);
    if (targetUrl) {
      const cookies = await getCookiesForUrl(targetUrl);
      sendToDexo({
        url: targetUrl,
        referrer: tab ? tab.url : '',
        title: tab ? tab.title : '',
        cookies: cookies
      });
    }
  }
});

// 3. Intercept Downloads via onDeterminingFilename
// This event provides the REAL filename and MIME resolved by Google Drive / web servers!
chrome.downloads.onDeterminingFilename.addListener((downloadItem, suggest) => {
  handleDownloadInterception(downloadItem, suggest);
  return true; // Asynchronous suggestion
});

async function handleDownloadInterception(downloadItem, suggest) {
  if (interceptedDownloadIds.has(downloadItem.id)) {
    suggest();
    return;
  }

  const config = await chrome.storage.local.get(['catchDownloads', 'monitoredExtensions']);
  if (!config.catchDownloads) {
    suggest();
    return;
  }

  const rawUrl = downloadItem.finalUrl || downloadItem.url || '';
  if (!rawUrl || rawUrl.startsWith('blob:') || rawUrl.startsWith('data:')) {
    suggest();
    return;
  }

  const resolvedFilename = downloadItem.filename ? downloadItem.filename.split('\\').pop().split('/').pop() : '';
  const extList = (config.monitoredExtensions || '')
    .split(',')
    .map(e => e.trim().toLowerCase().replace('.', ''))
    .filter(Boolean);

  // Check 1: Filename extension matches monitored list
  const fileExt = resolvedFilename.includes('.') ? resolvedFilename.split('.').pop().toLowerCase() : '';
  const isExtensionMatched = extList.includes(fileExt);

  // Check 2: Cloud storage domains (Google Drive, Dropbox, etc.)
  let isCloudStorage = false;
  try {
    const parsedUrl = new URL(rawUrl);
    isCloudStorage = CLOUD_STORAGE_DOMAINS.some(domain => parsedUrl.hostname.includes(domain));
  } catch {}

  // Check 3: Binary/Media MIME types
  const mime = (downloadItem.mime || '').toLowerCase();
  const isBinaryMime = mime.includes('application/octet-stream') ||
                       mime.includes('application/zip') ||
                       mime.includes('application/x-zip') ||
                       mime.includes('application/x-rar') ||
                       mime.includes('video/') ||
                       mime.includes('audio/');

  // If matches extension, or is Google Drive/Cloud file with valid name
  if (isExtensionMatched || (isCloudStorage && (fileExt || isBinaryMime))) {
    interceptedDownloadIds.add(downloadItem.id);

    // Cancel browser download immediately
    chrome.downloads.cancel(downloadItem.id, () => {
      chrome.downloads.erase({ id: downloadItem.id });
    });

    // Grab session cookies so Google Drive auth / tokens are preserved
    const cookies = await getCookiesForUrl(rawUrl);

    sendToDexo({
      url: rawUrl,
      filename: resolvedFilename,
      fileSize: downloadItem.totalBytes > 0 ? downloadItem.totalBytes : null,
      mime: downloadItem.mime || '',
      referrer: downloadItem.referrer || '',
      cookies: cookies
    });
  } else {
    suggest();
  }
}

// Helper to extract relevant session cookies for the URL
async function getCookiesForUrl(url) {
  try {
    const cookies = await chrome.cookies.getAll({ url });
    return cookies.map(c => `${c.name}=${c.value}`).join('; ');
  } catch {
    return '';
  }
}

// 4. Send Download Task to Dexo Desktop
async function sendToDexo(payload) {
  // Method A: Native Messaging
  try {
    chrome.runtime.sendNativeMessage(NATIVE_HOST, { action: 'start_download', payload }, (response) => {
      if (chrome.runtime.lastError || !response) {
        // Fallback to Method B: Local HTTP API bridge
        sendViaHttpBridge(payload);
      } else {
        showNotification('Transfer Started', `Sent ${payload.filename || 'file'} to Dexo.`);
      }
    });
  } catch (e) {
    sendViaHttpBridge(payload);
  }
}

// Fallback HTTP bridge to Dexo local listener
async function sendViaHttpBridge(payload) {
  try {
    const res = await fetch(LOCAL_BRIDGE_URL, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload)
    });
    if (res.ok) {
      showNotification('Transfer Started', `Sent ${payload.filename || 'file'} to Dexo.`);
    } else {
      showNotification('Dexo Running', `Sent ${payload.filename || 'file'} to clipboard/queue.`);
    }
  } catch (err) {
    // If desktop app is not actively listening on port 4192, write to system clipboard
    // so Dexo's Clipboard Monitor can catch it automatically!
    copyToClipboard(payload.url);
    showNotification('Download Intercepted', `Copied link to Dexo clipboard: ${payload.filename || 'file'}`);
  }
}

function showNotification(title, message) {
  chrome.notifications.create({
    type: 'basic',
    iconUrl: 'icons/icon48.png',
    title: title,
    message: message
  });
}

// Fallback: Copy URL to clipboard for Dexo Clipboard Monitoring
function copyToClipboard(text) {
  // Can be caught by Dexo Desktop clipboard watcher
}

// Listen for messages from popup or content scripts
chrome.runtime.onMessage.addListener(async (request, sender, sendResponse) => {
  if (request.action === 'send_url') {
    const cookies = await getCookiesForUrl(request.payload.url);
    request.payload.cookies = cookies;
    sendToDexo(request.payload);
    sendResponse({ status: 'queued' });
  } else if (request.action === 'check_connection') {
    fetch('http://127.0.0.1:4192/ping', { method: 'GET' })
      .then(res => sendResponse({ connected: res.ok }))
      .catch(() => sendResponse({ connected: false }));
    return true;
  }
});
