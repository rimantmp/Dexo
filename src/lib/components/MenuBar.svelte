<script lang="ts">
  export let onAddUrl: () => void;
  export let onOptions: () => void;

  let activeMenu: string | null = null;

  function toggleMenu(menu: string) {
    activeMenu = activeMenu === menu ? null : menu;
  }

  function closeMenu() {
    activeMenu = null;
  }
</script>

<svelte:window on:click={closeMenu} />

<div class="h-6 bg-slate-100 border-b border-slate-300 px-2 flex items-center gap-1 text-[11px] text-slate-800 select-none relative z-30">
  <!-- Tasks -->
  <div class="relative">
    <button
      type="button"
      on:click|stopPropagation={() => toggleMenu('tasks')}
      class="px-2 py-0.5 rounded hover:bg-slate-200 {activeMenu === 'tasks' ? 'bg-slate-200' : ''}"
    >
      Tasks
    </button>
    {#if activeMenu === 'tasks'}
      <div class="absolute left-0 top-full mt-0.5 w-44 bg-white border border-slate-300 shadow-md rounded py-1 z-50 text-xs">
        <button
          type="button"
          on:click={() => { onAddUrl(); closeMenu(); }}
          class="w-full text-left px-3 py-1 hover:bg-blue-600 hover:text-white flex justify-between"
        >
          <span>Add URL</span>
          <span class="text-slate-400 font-mono text-[10px]">Ctrl+N</span>
        </button>
        <div class="border-t border-slate-200 my-1"></div>
        <button
          type="button"
          on:click={closeMenu}
          class="w-full text-left px-3 py-1 hover:bg-blue-600 hover:text-white"
        >
          Exit
        </button>
      </div>
    {/if}
  </div>

  <!-- Downloads -->
  <div class="relative">
    <button
      type="button"
      on:click|stopPropagation={() => toggleMenu('downloads')}
      class="px-2 py-0.5 rounded hover:bg-slate-200 {activeMenu === 'downloads' ? 'bg-slate-200' : ''}"
    >
      Downloads
    </button>
    {#if activeMenu === 'downloads'}
      <div class="absolute left-0 top-full mt-0.5 w-44 bg-white border border-slate-300 shadow-md rounded py-1 z-50 text-xs">
        <button
          type="button"
          on:click={() => { onAddUrl(); closeMenu(); }}
          class="w-full text-left px-3 py-1 hover:bg-blue-600 hover:text-white"
        >
          Start New Download
        </button>
        <button
          type="button"
          on:click={() => { onOptions(); closeMenu(); }}
          class="w-full text-left px-3 py-1 hover:bg-blue-600 hover:text-white"
        >
          Options...
        </button>
      </div>
    {/if}
  </div>

  <!-- View -->
  <div class="relative">
    <button
      type="button"
      on:click|stopPropagation={() => toggleMenu('view')}
      class="px-2 py-0.5 rounded hover:bg-slate-200 {activeMenu === 'view' ? 'bg-slate-200' : ''}"
    >
      View
    </button>
    {#if activeMenu === 'view'}
      <div class="absolute left-0 top-full mt-0.5 w-44 bg-white border border-slate-300 shadow-md rounded py-1 z-50 text-xs">
        <div class="px-3 py-1 text-slate-500 font-semibold text-[10px] uppercase">Toolbar Style</div>
        <div class="px-3 py-1 text-slate-700">✓ Modern Classic IDM</div>
      </div>
    {/if}
  </div>

  <!-- Help -->
  <div class="relative">
    <button
      type="button"
      on:click|stopPropagation={() => toggleMenu('help')}
      class="px-2 py-0.5 rounded hover:bg-slate-200 {activeMenu === 'help' ? 'bg-slate-200' : ''}"
    >
      Help
    </button>
    {#if activeMenu === 'help'}
      <div class="absolute left-0 top-full mt-0.5 w-44 bg-white border border-slate-300 shadow-md rounded py-1 z-50 text-xs">
        <div class="px-3 py-1.5 font-medium text-slate-800">
          Dexo v0.1.0
          <p class="text-[10px] text-slate-500 font-mono mt-0.5">Rust Engine & Tauri 2</p>
        </div>
      </div>
    {/if}
  </div>
</div>

