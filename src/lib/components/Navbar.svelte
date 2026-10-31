<script lang="ts">
  import Icons from './Icons.svelte';

  export let onOpenAddModal: () => void;
  export let searchQuery: string = '';
</script>

<header class="h-14 bg-white border-b border-slate-200 px-5 flex items-center justify-between shrink-0">
  <!-- Search input -->
  <div class="relative w-72">
    <span class="absolute inset-y-0 left-0 pl-2.5 flex items-center pointer-events-none text-slate-400">
      <Icons name="search" size={14} />
    </span>
    <input
      type="text"
      bind:value={searchQuery}
      placeholder="Filter by filename or URL..."
      class="w-full pl-8 pr-3 py-1.5 text-xs bg-slate-50 border border-slate-200 rounded text-slate-900 placeholder:text-slate-400 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:bg-white focus:border-blue-500 transition-colors"
    />
  </div>

  <!-- Primary Action -->
  <div class="flex items-center gap-2">
    <button
      type="button"
      on:click={onOpenAddModal}
      class="inline-flex items-center gap-1.5 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white text-xs font-semibold px-3 py-1.5 rounded shadow-sm focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 transition-colors"
    >
      <Icons name="plus" size={14} strokeWidth={2.5} />
      <span>New Download</span>
    </button>
  </div>
</header>
