<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Icons from './Icons.svelte';
  import type { DownloadItem, DownloadProgress } from '../types';

  export let downloads: DownloadItem[] = [];
  export let progressMap: Record<string, DownloadProgress> = {};
  export let onRefresh: () => void;
  export let onOpenAddModal: () => void;

  function formatBytes(bytes: number | null | undefined): string {
    if (!bytes || bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  function formatSpeed(bytesPerSec: number | undefined): string {
    if (!bytesPerSec || bytesPerSec === 0) return '0 KB/s';
    return `${formatBytes(bytesPerSec)}/s`;
  }

  function formatEta(seconds: number | null | undefined): string {
    if (!seconds) return '--';
    if (seconds < 60) return `${seconds}s`;
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}m ${secs}s`;
  }

  async function handlePause(id: string) {
    try {
      await invoke('pause_download', { id });
      onRefresh();
    } catch (e) {
      console.error(e);
    }
  }

  async function handleDelete(id: string) {
    try {
      await invoke('delete_download', { id });
      onRefresh();
    } catch (e) {
      console.error(e);
    }
  }
</script>

<div class="flex-1 overflow-y-auto p-5 space-y-2.5">
  {#if downloads.length === 0}
    <!-- Meaningful Empty State -->
    <div class="h-80 flex flex-col items-center justify-center text-center p-6 border border-dashed border-slate-300 rounded-lg bg-white">
      <div class="w-10 h-10 rounded-full bg-slate-100 flex items-center justify-center text-slate-400 mb-2.5">
        <Icons name="download" size={20} />
      </div>
      <h3 class="text-sm font-semibold text-slate-800">No active or queued transfers</h3>
      <p class="text-xs text-slate-500 max-w-sm mt-1 mb-4">
        Add a URL to start downloading with parallel multi-chunk acceleration.
      </p>
      <button
        type="button"
        on:click={onOpenAddModal}
        class="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-semibold bg-blue-600 hover:bg-blue-700 text-white rounded transition-colors"
      >
        <Icons name="plus" size={13} strokeWidth={2.5} />
        <span>Add Download</span>
      </button>
    </div>
  {:else}
    <!-- Active & Completed Item Rows -->
    {#each downloads as item (item.id)}
      {@const progress = progressMap[item.id]}
      {@const downloaded = progress ? progress.downloaded_bytes : item.downloaded_size}
      {@const total = progress?.total_bytes ?? item.total_size}
      {@const pct = progress ? progress.progress_percentage : (total ? (downloaded / total) * 100 : 0)}
      {@const isDownloading = item.status === 'Downloading'}

      <div class="bg-white rounded-lg border border-slate-200 p-3.5 shadow-xs hover:border-slate-300 transition-colors space-y-2.5">
        <!-- Item Header -->
        <div class="flex items-start justify-between gap-3">
          <div class="flex items-center gap-2.5 min-w-0">
            <div class="w-8 h-8 rounded bg-slate-100 border border-slate-200 flex items-center justify-center text-slate-600 shrink-0">
              <Icons name="file" size={16} />
            </div>
            <div class="min-w-0">
              <h4 class="text-xs font-bold text-slate-900 truncate leading-snug" title={item.filename}>
                {item.filename}
              </h4>
              <p class="text-[11px] text-slate-400 font-mono truncate max-w-lg mt-0.5 leading-none" title={item.url}>
                {item.url}
              </p>
            </div>
          </div>

          <!-- Controls -->
          <div class="flex items-center gap-1.5 shrink-0">
            {#if isDownloading}
              <button
                type="button"
                on:click={() => handlePause(item.id)}
                class="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium text-amber-800 bg-amber-50 hover:bg-amber-100 border border-amber-200 rounded transition-colors"
                title="Pause transfer"
              >
                <Icons name="pause" size={12} strokeWidth={2.5} />
                <span>Pause</span>
              </button>
            {/if}
            <button
              type="button"
              on:click={() => handleDelete(item.id)}
              class="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium text-slate-600 hover:text-red-700 bg-slate-50 hover:bg-red-50 border border-slate-200 hover:border-red-200 rounded transition-colors"
              title="Delete record"
            >
              <Icons name="trash" size={12} />
              <span>Delete</span>
            </button>
          </div>
        </div>

        <!-- Progress Track -->
        <div class="w-full bg-slate-100 h-1.5 rounded-full overflow-hidden">
          <div
            class="h-full transition-all duration-200 {item.status === 'Completed' ? 'bg-emerald-600' : 'bg-blue-600'}"
            style="width: {pct}%"
          ></div>
        </div>

        <!-- Metric Details Row -->
        <div class="flex items-center justify-between text-[11px] text-slate-600 font-mono">
          <div class="flex items-center gap-3">
            <span class="tabular-nums font-medium text-slate-800">
              {formatBytes(downloaded)} / {formatBytes(total)} ({pct.toFixed(1)}%)
            </span>
            {#if isDownloading && progress}
              <span class="text-blue-700 font-bold tabular-nums">
                {formatSpeed(progress.speed_bytes_per_sec)}
              </span>
              <span class="text-slate-500 tabular-nums">
                ETA: {formatEta(progress.eta_seconds)}
              </span>
              <span class="text-slate-400">
                ({item.connection_count} streams)
              </span>
            {/if}
          </div>

          <div>
            <span class="px-2 py-0.5 rounded text-[11px] font-sans font-medium {
              item.status === 'Completed' ? 'bg-emerald-50 text-emerald-800 border border-emerald-200' :
              item.status === 'Downloading' ? 'bg-blue-50 text-blue-800 border border-blue-200' :
              item.status === 'Paused' ? 'bg-amber-50 text-amber-800 border border-amber-200' : 'bg-slate-100 text-slate-700 border border-slate-200'
            }">
              {item.status}
            </span>
          </div>
        </div>
      </div>
    {/each}
  {/if}
</div>
