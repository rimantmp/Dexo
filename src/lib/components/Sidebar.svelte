<script lang="ts">
  import Icons from './Icons.svelte';

  export let currentTab: string = 'dashboard';
  export let counts: { active: number; completed: number; all: number } = { active: 0, completed: 0, all: 0 };

  const menuItems = [
    { id: 'dashboard', label: 'Dashboard', icon: 'layers' },
    { id: 'all', label: 'All Transfers', icon: 'download', countKey: 'all' },
    { id: 'active', label: 'Active', icon: 'speed', countKey: 'active' },
    { id: 'completed', label: 'Completed', icon: 'check', countKey: 'completed' },
    { id: 'categories', label: 'Categories', icon: 'folder' },
    { id: 'scheduler', label: 'Scheduler', icon: 'clock' },
    { id: 'settings', label: 'Settings', icon: 'settings' },
  ];
</script>

<aside class="w-60 bg-slate-900 text-slate-300 flex flex-col border-r border-slate-800 select-none shrink-0">
  <!-- Brand Header -->
  <div class="h-14 flex items-center px-4 gap-2.5 border-b border-slate-800">
    <div class="w-7 h-7 rounded bg-blue-600 flex items-center justify-center text-white shrink-0">
      <Icons name="download" size={16} strokeWidth={2.5} />
    </div>
    <div class="min-w-0">
      <h1 class="font-semibold text-white text-sm tracking-tight leading-none">Dexo</h1>
      <p class="text-[11px] text-slate-500 mt-1 leading-none font-mono">v0.1.0 (64-bit)</p>
    </div>
  </div>

  <!-- Navigation Menu -->
  <nav class="flex-1 px-2.5 py-3 space-y-1">
    <p class="px-2 pb-1 text-[11px] font-semibold tracking-wider text-slate-500 uppercase">Library</p>
    {#each menuItems as item}
      <button
        type="button"
        on:click={() => (currentTab = item.id)}
        class="w-full flex items-center justify-between px-2.5 py-2 rounded text-xs font-medium transition-colors focus:outline-none focus-visible:ring-1 focus-visible:ring-blue-400 {currentTab === item.id
          ? 'bg-blue-600 text-white font-semibold'
          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/80'}"
      >
        <div class="flex items-center gap-2.5 min-w-0">
          <Icons name={item.icon} size={15} className={currentTab === item.id ? 'text-white' : 'text-slate-400'} />
          <span class="truncate">{item.label}</span>
        </div>
        {#if item.countKey && counts[item.countKey] > 0}
          <span class="text-[11px] font-mono px-1.5 py-0.2 rounded {currentTab === item.id ? 'bg-blue-700 text-white' : 'bg-slate-800 text-slate-300'}">
            {counts[item.countKey]}
          </span>
        {/if}
      </button>
    {/each}
  </nav>

  <!-- Technical Status Bar -->
  <div class="p-3 border-t border-slate-800 text-[11px] text-slate-500 font-mono space-y-1">
    <div class="flex items-center justify-between">
      <span>Engine:</span>
      <span class="text-slate-400">Rust (Tokio)</span>
    </div>
    <div class="flex items-center justify-between">
      <span>Storage:</span>
      <span class="text-slate-400">SQLite WAL</span>
    </div>
  </div>
</aside>
