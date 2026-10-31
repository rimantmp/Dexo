// Dexo Media Sniffer & Cloud Storage Handler Content Script

(function () {
  const processedVideos = new WeakSet();

  // 1. Scan HTML5 Video & Audio Elements
  function scanMediaElements() {
    const videos = document.querySelectorAll('video');
    videos.forEach((video) => {
      if (processedVideos.has(video)) return;
      processedVideos.add(video);

      attachDownloadButton(video);
    });
  }

  function attachDownloadButton(video) {
    const videoSrc = video.currentSrc || video.src || (video.querySelector('source') ? video.querySelector('source').src : null);
    if (!videoSrc || videoSrc.startsWith('blob:') || videoSrc.startsWith('data:')) return;

    const btn = document.createElement('div');
    btn.className = 'dexo-video-panel';
    btn.innerHTML = `
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
        <polyline points="7 10 12 15 17 10"></polyline>
        <line x1="12" y1="15" x2="12" y2="3"></line>
      </svg>
      <span>Download with Dexo</span>
    `;

    btn.addEventListener('click', (e) => {
      e.preventDefault();
      e.stopPropagation();

      const activeSrc = video.currentSrc || video.src;
      if (activeSrc) {
        chrome.runtime.sendMessage({
          action: 'send_url',
          payload: {
            url: activeSrc,
            title: document.title,
            referrer: window.location.href
          }
        });
      }
    });

    const parent = video.parentElement;
    if (parent) {
      const position = window.getComputedStyle(parent).position;
      if (position === 'static') {
        parent.style.position = 'relative';
      }
      parent.appendChild(btn);
    }
  }

  // 2. Google Drive "Download anyway" helper
  function checkGoogleDriveDownloadPage() {
    if (!window.location.hostname.includes('drive.google.com') && !window.location.hostname.includes('googleusercontent.com')) {
      return;
    }

    // Google Drive large file confirmation link or form
    const dlLink = document.getElementById('uc-download-link');
    if (dlLink && !dlLink.dataset.downbyteHooked) {
      dlLink.dataset.downbyteHooked = 'true';
      dlLink.addEventListener('click', (e) => {
        const form = dlLink.closest('form');
        let targetUrl = dlLink.href;
        if (!targetUrl && form) {
          targetUrl = form.action;
        }
        if (targetUrl) {
          chrome.runtime.sendMessage({
            action: 'send_url',
            payload: {
              url: targetUrl,
              title: document.title,
              referrer: window.location.href
            }
          });
        }
      });
    }
  }

  // Run scans periodically for dynamic single page applications
  setInterval(() => {
    scanMediaElements();
    checkGoogleDriveDownloadPage();
  }, 1500);
})();
