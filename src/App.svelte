<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import MenuBar from './lib/components/MenuBar.svelte';
  import Toolbar from './lib/components/Toolbar.svelte';
  import CategoryTree from './lib/components/CategoryTree.svelte';
  import TransferTable from './lib/components/TransferTable.svelte';
  import AddDownloadModal from './lib/components/AddDownloadModal.svelte';
  import OptionsModal from './lib/components/OptionsModal.svelte';
  import type { DownloadItem, DownloadProgress } from './lib/types';

  let isAddModalOpen = false;
  let isOptionsModalOpen = false;
  let searchQuery = '';
  let selectedCategory = 'all';
  let selectedId: string | null = null;
  let initialUrl = '';
  let initialFilename = '';

  let downloads: DownloadItem[] = [];
  let progressMap: Record<string, DownloadProgress> = {};

  let unlistenProgress: UnlistenFn | null = null;
  let unlistenCompleted: UnlistenFn | null = null;
  let unlistenFailed: UnlistenFn | null = null;
  let unlistenExternal: UnlistenFn | null = null;

  async function loadDownloads() {
    try {
      downloads = await invoke<DownloadItem[]>('get_downloads');
      // If selected item no longer exists, deselect
      if (selectedId && !downloads.some((d) => d.id === selectedId)) {
        selectedId = null;
      }
    } catch (e) {
      console.error('Failed to load downloads:', e);
    }
  }

  onMount(async () => {
    await loadDownloads();

    try {
      unlistenProgress = await listen<DownloadProgress>('download-progress', (event) => {
        progressMap[event.payload.download_id] = event.payload;
        progressMap = { ...progressMap };
      });

      unlistenCompleted = await listen<string>('download-completed', async () => {
        await loadDownloads();
      });

      unlistenFailed = await listen<[string, string]>('download-failed', async () => {
        await loadDownloads();
      });

      unlistenExternal = await listen<{ url: string; filename?: string }>('external-download-received', (event) => {
        initialUrl = event.payload.url;
        initialFilename = event.payload.filename || '';
        isAddModalOpen = true;
      });
    } catch (e) {
      console.warn('Tauri event listeners could not be attached outside desktop window:', e);
    }
  });

  onDestroy(() => {
    if (unlistenProgress) unlistenProgress();
    if (unlistenCompleted) unlistenCompleted();
    if (unlistenFailed) unlistenFailed();
    if (unlistenExternal) unlistenExternal();
  });

  // Action Handlers
  async function handleStopSelected() {
    if (!selectedId) return;
    try {
      await invoke('pause_download', { id: selectedId });
      await loadDownloads();
    } catch (e) {
      console.error(e);
    }
  }

  async function handleResumeSelected() {
    if (!selectedId) return;
    const item = downloads.find((d) => d.id === selectedId);
    if (!item) return;
    try {
      await invoke('start_download', {
        url: item.url,
        filename: item.filename,
        destination: item.destination.substring(0, item.destination.lastIndexOf('\\')),
        connectionCount: item.connection_count || 4,
      });
      await loadDownloads();
    } catch (e) {
      console.error(e);
    }
  }

  async function handleStopAll() {
    const activeItems = downloads.filter((d) => d.status === 'Downloading');
    for (const item of activeItems) {
      try {
        await invoke('pause_download', { id: item.id });
      } catch (e) {
        console.error(e);
      }
    }
    await loadDownloads();
  }

  async function handleDeleteSelected() {
    if (!selectedId) return;
    try {
      await invoke('delete_download', { id: selectedId });
      selectedId = null;
      await loadDownloads();
    } catch (e) {
      console.error(e);
    }
  }

  function getExtension(filename: string): string {
    const idx = filename.lastIndexOf('.');
    return idx !== -1 ? filename.slice(idx).toLowerCase() : '';
  }

  function matchesCategory(filename: string, catId: string): boolean {
    const ext = getExtension(filename);
    if (catId === 'cat-compressed') {
      return ['.zip', '.rar', '.7z', '.tar', '.gz', '.iso', '.bz2'].includes(ext);
    }
    if (catId === 'cat-documents') {
      return ['.pdf', '.doc', '.docx', '.xls', '.xlsx', '.txt', '.ppt', '.pptx', '.odt'].includes(ext);
    }
    if (catId === 'cat-music') {
      return ['.mp3', '.wav', '.flac', '.aac', '.ogg', '.m4a'].includes(ext);
    }
    if (catId === 'cat-programs') {
      return ['.exe', '.msi', '.apk', '.dmg', '.bat', '.cmd', '.ps1'].includes(ext);
    }
    if (catId === 'cat-video') {
      return ['.mp4', '.mkv', '.avi', '.mov', '.wmv', '.webm', '.flv', '.m4v'].includes(ext);
    }
    return true;
  }

  // Filtered downloads for grid
  $: filteredDownloads = downloads.filter((d) => {
    const matchesSearch = d.filename.toLowerCase().includes(searchQuery.toLowerCase()) ||
                          d.url.toLowerCase().includes(searchQuery.toLowerCase());
    if (!matchesSearch) return false;

    if (selectedCategory === 'unfinished') {
      return d.status === 'Downloading' || d.status === 'Paused' || d.status === 'Pending';
    }
    if (selectedCategory === 'finished') {
      return d.status === 'Completed';
    }
    if (selectedCategory.startsWith('cat-')) {
      return matchesCategory(d.filename, selectedCategory);
    }
    return true;
  });

  // Selected item capabilities
  $: selectedItem = downloads.find((d) => d.id === selectedId);
  $: canResume = selectedItem ? selectedItem.status === 'Paused' || selectedItem.status === 'Failed' : false;
  $: canStop = selectedItem ? selectedItem.status === 'Downloading' : false;
  $: canDelete = selectedItem != null;

  // Counters for Category Tree
  $: counts = {
    all: downloads.length,
    unfinished: downloads.filter((d) => d.status === 'Downloading' || d.status === 'Paused').length,
    finished: downloads.filter((d) => d.status === 'Completed').length,
    compressed: downloads.filter((d) => matchesCategory(d.filename, 'cat-compressed')).length,
    documents: downloads.filter((d) => matchesCategory(d.filename, 'cat-documents')).length,
    music: downloads.filter((d) => matchesCategory(d.filename, 'cat-music')).length,
    programs: downloads.filter((d) => matchesCategory(d.filename, 'cat-programs')).length,
    video: downloads.filter((d) => matchesCategory(d.filename, 'cat-video')).length,
  };
</script>

<div class="flex flex-col h-screen w-screen overflow-hidden bg-slate-100 font-sans text-slate-900 select-none">
  <!-- 1. IDM Classic Menu Bar -->
  <MenuBar
    onAddUrl={() => (isAddModalOpen = true)}
    onOptions={() => (isOptionsModalOpen = true)}
  />

  <!-- 2. IDM Classic Action Toolbar -->
  <Toolbar
    bind:searchQuery
    onAddUrl={() => (isAddModalOpen = true)}
    onResume={handleResumeSelected}
    onStop={handleStopSelected}
    onStopAll={handleStopAll}
    onDelete={handleDeleteSelected}
    onOptions={() => (isOptionsModalOpen = true)}
    onScheduler={() => (isOptionsModalOpen = true)}
    {canResume}
    {canStop}
    {canDelete}
  />

  <!-- 3. Main Workspace: Category Tree (Left) + Transfer Table (Right) -->
  <div class="flex-1 flex min-h-0 overflow-hidden">
    <!-- Left Category Tree -->
    <CategoryTree
      bind:selectedCategory
      {counts}
    />

    <!-- Right Transfer Grid Table -->
    <TransferTable
      downloads={filteredDownloads}
      {progressMap}
      bind:selectedId
      onSelect={(id) => (selectedId = id)}
      onDoubleClick={(id) => {
        selectedId = id;
        if (canStop) handleStopSelected();
        else if (canResume) handleResumeSelected();
      }}
    />
  </div>

  <!-- Modals -->
  <AddDownloadModal
    isOpen={isAddModalOpen}
    bind:initialUrl
    bind:initialFilename
    onClose={() => {
      isAddModalOpen = false;
      initialUrl = '';
      initialFilename = '';
    }}
    onDownloadStarted={loadDownloads}
  />

  <OptionsModal
    isOpen={isOptionsModalOpen}
    onClose={() => (isOptionsModalOpen = false)}
  />
</div>
