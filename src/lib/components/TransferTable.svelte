<script lang="ts">
  import Icons from './Icons.svelte';
  import type { DownloadItem, DownloadProgress } from '../types';

  export let downloads: DownloadItem[] = [];
  export let progressMap: Record<string, DownloadProgress> = {};
  export let selectedId: string | null = null;
  export let onSelect: (id: string) => void;
  export let onDoubleClick: (id: string) => void;

  function formatBytes(bytes: number | null | undefined): string {
    if (!bytes || bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  function formatSpeed(bytesPerSec: number | undefined): string {
    if (!bytesPerSec || bytesPerSec === 0) return '--';
    return `${formatBytes(bytesPerSec)}/s`;
  }

  function formatEta(seconds: number | null | undefined): string {
    if (!seconds) return '--';
    if (seconds < 60) return `${seconds}s`;
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}m ${secs}s`;
  }

  function formatDate(isoDate: string): string {
    if (!isoDate) return '--';
    try {
      const d = new Date(isoDate);
      return `${d.toLocaleDateString()} ${d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`;
    } catch {
      return isoDate;
    }
  }
</script>

<div class="flex-1 flex flex-col min-w-0 bg-white overflow-hidden select-none">
  <!-- Table Header -->
  <div class="h-7 bg-slate-100 border-b border-slate-300 flex items-center text-[11px] font-bold text-slate-700 shrink-0">
    <div class="w-2/5 px-3 border-r border-slate-300 truncate">File Name</div>
    <div class="w-1/6 px-2 border-r border-slate-300 truncate">Size</div>
    <div class="w-1/4 px-2 border-r border-slate-300 truncate">Status</div>
    <div class="w-1/8 px-2 border-r border-slate-300 truncate text-right">Speed</div>
    <div class="w-1/8 px-2 border-r border-slate-300 truncate text-right">Time Left</div>
    <div class="w-1/6 px-2 truncate">Date Added</div>
  </div>

  <!-- Table Body / Rows -->
  <div class="flex-1 overflow-y-auto">
    {#if downloads.length === 0}
      <div class="h-64 flex flex-col items-center justify-center text-slate-400 text-xs">
        <Icons name="file" size={24} className="mb-2 text-slate-300" />
        <p>No downloads in this view</p>
        <p class="text-[10px] text-slate-400 mt-0.5">Click "Add URL" to start an accelerated download</p>
      </div>
    {:else}
      <table class="w-full text-left text-xs border-collapse">
        <tbody>
          {#each downloads as item (item.id)}
            {@const progress = progressMap[item.id]}
            {@const downloaded = progress ? progress.downloaded_bytes : item.downloaded_size}
            {@const total = progress?.total_bytes ?? item.total_size}
            {@const pct = progress ? progress.progress_percentage : (total ? (downloaded / total) * 100 : 0)}
            {@const isSelected = selectedId === item.id}
            {@const isDownloading = item.status === 'Downloading'}

            <tr
              on:click={() => onSelect(item.id)}
              on:dblclick={() => onDoubleClick(item.id)}
              class="h-7.5 border-b border-slate-200 cursor-pointer font-sans transition-colors {isSelected
                ? 'bg-blue-600 text-white'
                : 'hover:bg-blue-50/60 text-slate-800'}"
            >
              <!-- File Name -->
              <td class="w-2/5 px-3 truncate max-w-xs">
                <div class="flex items-center gap-2 truncate">
                  <Icons
                    name="file"
                    size={14}
                    className={isSelected ? 'text-white' : 'text-slate-500'}
                  />
                  <span class="font-medium truncate" title={item.filename}>
                    {item.filename}
                  </span>
                </div>
              </td>

              <!-- Size -->
              <td class="w-1/6 px-2 font-mono text-[11px] tabular-nums truncate">
                {#if total}
                  <span>{formatBytes(total)}</span>
                {:else}
                  <span class="text-slate-400">Unknown</span>
                {/if}
              </td>

              <!-- Status with progress track -->
              <td class="w-1/4 px-2 truncate">
                <div class="flex items-center gap-2 min-w-0">
                  <!-- Mini Progress Bar -->
                  <div class="w-20 bg-slate-200 h-2.5 rounded-sm overflow-hidden shrink-0 border {isSelected ? 'border-blue-400 bg-blue-700' : 'border-slate-300'}">
                    <div
                      class="h-full {item.status === 'Completed' ? 'bg-emerald-500' : isSelected ? 'bg-white' : 'bg-blue-600'}"
                      style="width: {pct}%"
                    ></div>
                  </div>
                  <!-- Percentage & Status Text -->
                  <span class="font-mono text-[10px] tabular-nums truncate {isSelected ? 'text-blue-100' : 'text-slate-600'}">
                    {#if isDownloading}
                      {pct.toFixed(0)}%
                    {:else}
                      {item.status}
                    {/if}
                  </span>
                </div>
              </td>

              <!-- Transfer Rate (Speed) -->
              <td class="w-1/8 px-2 font-mono text-[11px] tabular-nums text-right truncate">
                {#if isDownloading && progress}
                  <span class={isSelected ? 'text-white font-bold' : 'text-blue-700 font-bold'}>
                    {formatSpeed(progress.speed_bytes_per_sec)}
                  </span>
                {:else}
                  <span class={isSelected ? 'text-blue-200' : 'text-slate-400'}>--</span>
                {/if}
              </td>

              <!-- Time Left (ETA) -->
              <td class="w-1/8 px-2 font-mono text-[11px] tabular-nums text-right truncate">
                {#if isDownloading && progress}
                  <span>{formatEta(progress.eta_seconds)}</span>
                {:else}
                  <span class={isSelected ? 'text-blue-200' : 'text-slate-400'}>--</span>
                {/if}
              </td>

              <!-- Date Added -->
              <td class="w-1/6 px-2 font-mono text-[10px] truncate {isSelected ? 'text-blue-100' : 'text-slate-500'}">
                {formatDate(item.created_at)}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>

  <!-- Bottom Status Bar -->
  <div class="h-6 bg-slate-100 border-t border-slate-300 px-3 flex items-center justify-between text-[11px] text-slate-600 shrink-0 font-mono">
    <div>
      Total: <span class="text-slate-900 font-semibold">{downloads.length}</span> file(s)
    </div>
    {#if selectedId}
      <div class="truncate max-w-sm">
        Selected ID: <span class="text-blue-600">{selectedId.slice(0, 8)}...</span>
      </div>
    {/if}
    <div>
      Dexo Rust Engine: <span class="text-emerald-700 font-semibold">Active</span>
    </div>
  </div>
</div>

