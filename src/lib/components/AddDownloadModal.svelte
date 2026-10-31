<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Icons from './Icons.svelte';
  import type { UrlMetadata } from '../types';

  export let isOpen: boolean = false;
  export let onClose: () => void;
  export let onDownloadStarted: () => void;
  export let initialUrl: string = '';
  export let initialFilename: string = '';

  let url = '';
  let filename = '';
  let destination = 'C:\\Downloads';
  let connectionCount = 4;
  let isProbing = false;
  let metadata: UrlMetadata | null = null;
  let errorMessage = '';

  $: if (initialUrl && initialUrl !== url) {
    url = initialUrl;
    filename = initialFilename || '';
    handleProbe();
  }

  async function handleProbe() {
    if (!url.trim()) return;
    isProbing = true;
    errorMessage = '';
    try {
      metadata = await invoke<UrlMetadata>('probe_url', { url: url.trim() });
      filename = metadata.filename;
    } catch (e: any) {
      errorMessage = typeof e === 'string' ? e : 'Failed to probe target URL';
    } finally {
      isProbing = false;
    }
  }

  async function handleStartDownload() {
    if (!url.trim() || !filename.trim()) return;
    try {
      await invoke('start_download', {
        url: url.trim(),
        filename: filename.trim(),
        destination,
        connectionCount: Number(connectionCount),
      });
      onClose();
      onDownloadStarted();
      url = '';
      filename = '';
      metadata = null;
    } catch (e: any) {
      errorMessage = typeof e === 'string' ? e : 'Failed to start download process';
    }
  }

  function formatBytes(bytes: number | null): string {
    if (!bytes || bytes === 0) return 'Unknown size';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 bg-slate-900/60 z-50 flex items-center justify-center p-4">
    <div class="bg-white rounded-lg shadow-xl max-w-lg w-full overflow-hidden border border-slate-200">
      <!-- Modal Header -->
      <div class="px-5 py-3.5 border-b border-slate-200 flex items-center justify-between bg-slate-50">
        <h2 class="text-xs font-bold text-slate-800 uppercase tracking-wider">New Transfer</h2>
        <button
          type="button"
          on:click={onClose}
          class="text-slate-400 hover:text-slate-600 focus:outline-none p-1 rounded"
          aria-label="Close dialog"
        >
          <Icons name="x" size={16} />
        </button>
      </div>

      <!-- Modal Body -->
      <div class="p-5 space-y-4">
        {#if errorMessage}
          <div class="p-2.5 text-xs bg-red-50 text-red-700 rounded border border-red-200 font-mono">
            {errorMessage}
          </div>
        {/if}

        <!-- URL Input -->
        <div>
          <label class="block text-xs font-semibold text-slate-700 mb-1" for="url-input">
            Source URL
          </label>
          <div class="flex gap-2">
            <input
              id="url-input"
              type="text"
              bind:value={url}
              on:blur={handleProbe}
              placeholder="https://example.com/distribution/file.zip"
              class="flex-1 px-3 py-1.5 text-xs bg-slate-50 border border-slate-200 rounded text-slate-900 placeholder:text-slate-400 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:bg-white font-mono"
            />
            <button
              type="button"
              on:click={handleProbe}
              disabled={isProbing}
              class="px-3 py-1.5 bg-slate-100 hover:bg-slate-200 text-slate-700 rounded text-xs font-medium border border-slate-300 disabled:opacity-50 transition-colors shrink-0"
            >
              {isProbing ? 'Probing...' : 'Inspect'}
            </button>
          </div>
        </div>

        {#if metadata}
          <div class="p-3 bg-slate-50 rounded border border-slate-200 text-xs text-slate-700 flex justify-between items-center font-mono">
            <div>
              <p>Size: <span class="font-semibold text-slate-900">{formatBytes(metadata.total_size)}</span></p>
              <p class="text-[11px] text-slate-500 mt-0.5">
                Resume: {metadata.supports_range ? 'Supported (HTTP 206)' : 'Single stream only'}
              </p>
            </div>
            <span class="px-2 py-0.5 rounded text-[11px] font-sans font-medium {metadata.supports_range ? 'bg-emerald-100 text-emerald-800 border border-emerald-200' : 'bg-amber-100 text-amber-800 border border-amber-200'}">
              {metadata.supports_range ? 'Multi-chunk ready' : 'Single connection'}
            </span>
          </div>
        {/if}

        <!-- Filename -->
        <div>
          <label class="block text-xs font-semibold text-slate-700 mb-1" for="filename-input">
            Target Filename
          </label>
          <input
            id="filename-input"
            type="text"
            bind:value={filename}
            placeholder="package.zip"
            class="w-full px-3 py-1.5 text-xs bg-slate-50 border border-slate-200 rounded text-slate-900 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:bg-white font-mono"
          />
        </div>

        <!-- Destination Folder & Connections -->
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-xs font-semibold text-slate-700 mb-1" for="dest-input">
              Destination Directory
            </label>
            <input
              id="dest-input"
              type="text"
              bind:value={destination}
              class="w-full px-3 py-1.5 text-xs bg-slate-50 border border-slate-200 rounded text-slate-900 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:bg-white font-mono"
            />
          </div>

          <div>
            <label class="block text-xs font-semibold text-slate-700 mb-1" for="conn-select">
              Concurrent Streams
            </label>
            <select
              id="conn-select"
              bind:value={connectionCount}
              class="w-full px-3 py-1.5 text-xs bg-slate-50 border border-slate-200 rounded text-slate-900 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:bg-white font-sans"
            >
              <option value={1}>1 stream (Sequential)</option>
              <option value={2}>2 streams</option>
              <option value={4}>4 streams (Default)</option>
              <option value={8}>8 streams (High throughput)</option>
              <option value={16}>16 streams (Maximum)</option>
            </select>
          </div>
        </div>
      </div>

      <!-- Modal Footer -->
      <div class="px-5 py-3 bg-slate-50 border-t border-slate-200 flex justify-end gap-2">
        <button
          type="button"
          on:click={onClose}
          class="px-3 py-1.5 text-xs text-slate-600 hover:bg-slate-200 rounded font-medium transition-colors"
        >
          Cancel
        </button>
        <button
          type="button"
          on:click={handleStartDownload}
          disabled={!url || !filename}
          class="px-4 py-1.5 text-xs bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white rounded font-semibold transition-colors"
        >
          Start Transfer
        </button>
      </div>
    </div>
  </div>
{/if}
