<script lang="ts">
  import Icons from './Icons.svelte';

  export let isOpen: boolean = false;
  export let onClose: () => void;

  let activeTab: 'general' | 'connection' | 'saveto' = 'general';
  let defaultConnections = 4;
  let defaultFolder = 'C:\\Downloads';
</script>

{#if isOpen}
  <div class="fixed inset-0 bg-slate-900/50 z-50 flex items-center justify-center p-4 select-none">
    <div class="bg-white rounded border border-slate-400 shadow-2xl max-w-md w-full overflow-hidden text-xs">
      <!-- Title Bar -->
      <div class="h-8 bg-slate-200 border-b border-slate-300 px-3 flex items-center justify-between">
        <span class="font-bold text-slate-800 text-[11px]">Dexo Configuration</span>
        <button type="button" on:click={onClose} class="text-slate-500 hover:text-slate-800">
          <Icons name="x" size={14} />
        </button>
      </div>

      <!-- Tab Buttons -->
      <div class="flex border-b border-slate-300 bg-slate-100 px-2 pt-1 gap-1 text-[11px]">
        <button
          type="button"
          on:click={() => (activeTab = 'general')}
          class="px-3 py-1 rounded-t border-t border-x border-slate-300 transition-colors {activeTab === 'general' ? 'bg-white font-bold border-b-white -mb-px' : 'bg-slate-200 text-slate-600'}"
        >
          General
        </button>
        <button
          type="button"
          on:click={() => (activeTab = 'connection')}
          class="px-3 py-1 rounded-t border-t border-x border-slate-300 transition-colors {activeTab === 'connection' ? 'bg-white font-bold border-b-white -mb-px' : 'bg-slate-200 text-slate-600'}"
        >
          Connection
        </button>
        <button
          type="button"
          on:click={() => (activeTab = 'saveto')}
          class="px-3 py-1 rounded-t border-t border-x border-slate-300 transition-colors {activeTab === 'saveto' ? 'bg-white font-bold border-b-white -mb-px' : 'bg-slate-200 text-slate-600'}"
        >
          Save to
        </button>
      </div>

      <!-- Tab Content -->
      <div class="p-4 space-y-3 min-h-[180px]">
        {#if activeTab === 'general'}
          <div class="space-y-2">
            <label class="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" checked class="rounded border-slate-300 text-blue-600" />
              <span>Launch Dexo on Windows startup</span>
            </label>
            <label class="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" checked class="rounded border-slate-300 text-blue-600" />
              <span>Show download completion dialog</span>
            </label>
            <label class="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" checked class="rounded border-slate-300 text-blue-600" />
              <span>Minimize to system tray on close</span>
            </label>
          </div>
        {:else if activeTab === 'connection'}
          <div class="space-y-3">
            <div>
              <label class="block font-semibold mb-1" for="opt-conn">Default max. connection number:</label>
              <select id="opt-conn" bind:value={defaultConnections} class="w-full px-2 py-1 bg-white border border-slate-300 rounded">
                <option value={1}>1 connection</option>
                <option value={2}>2 connections</option>
                <option value={4}>4 connections (Standard)</option>
                <option value={8}>8 connections (High speed)</option>
                <option value={16}>16 connections (Turbo)</option>
              </select>
            </div>
            <p class="text-[10px] text-slate-500">
              Dexo download engine dynamically balances segments across parallel Tokio workers for maximum speed.
            </p>
          </div>
        {:else if activeTab === 'saveto'}
          <div class="space-y-2">
            <label class="block font-semibold" for="opt-folder">Default download directory:</label>
            <input
              id="opt-folder"
              type="text"
              bind:value={defaultFolder}
              class="w-full px-2 py-1 bg-white border border-slate-300 rounded font-mono text-[11px]"
            />
          </div>
        {/if}
      </div>

      <!-- Dialog Footer -->
      <div class="h-10 bg-slate-100 border-t border-slate-300 px-3 flex items-center justify-end gap-2">
        <button
          type="button"
          on:click={onClose}
          class="px-4 py-1 bg-white border border-slate-300 hover:bg-slate-200 rounded text-slate-800"
        >
          OK
        </button>
        <button
          type="button"
          on:click={onClose}
          class="px-4 py-1 bg-white border border-slate-300 hover:bg-slate-200 rounded text-slate-800"
        >
          Cancel
        </button>
      </div>
    </div>
  </div>
{/if}

