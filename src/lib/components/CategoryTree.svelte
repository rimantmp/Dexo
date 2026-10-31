<script lang="ts">
  import Icons from './Icons.svelte';

  export let selectedCategory: string = 'all';
  export let counts: {
    all: number;
    unfinished: number;
    finished: number;
    compressed: number;
    documents: number;
    music: number;
    programs: number;
    video: number;
  } = {
    all: 0,
    unfinished: 0,
    finished: 0,
    compressed: 0,
    documents: 0,
    music: 0,
    programs: 0,
    video: 0,
  };

  const categories = [
    { id: 'cat-compressed', label: 'Compressed', key: 'compressed' },
    { id: 'cat-documents', label: 'Documents', key: 'documents' },
    { id: 'cat-music', label: 'Music', key: 'music' },
    { id: 'cat-programs', label: 'Programs', key: 'programs' },
    { id: 'cat-video', label: 'Video', key: 'video' },
  ];
</script>

<aside class="w-56 bg-slate-50 border-r border-slate-300 flex flex-col text-xs text-slate-800 select-none shrink-0 overflow-y-auto">
  <!-- Tree Container -->
  <div class="p-2 space-y-1">
    <!-- Header -->
    <div class="px-2 py-1 text-[11px] font-bold text-slate-600 uppercase tracking-wider flex items-center gap-1.5">
      <Icons name="folder-open" size={14} className="text-amber-500" />
      <span>Categories</span>
    </div>

    <!-- All Downloads -->
    <button
      type="button"
      on:click={() => (selectedCategory = 'all')}
      class="w-full flex items-center justify-between px-2.5 py-1 rounded text-left transition-colors {selectedCategory === 'all'
        ? 'bg-blue-600 text-white font-semibold'
        : 'hover:bg-slate-200 text-slate-800'}"
    >
      <div class="flex items-center gap-2">
        <Icons name="folder" size={14} className={selectedCategory === 'all' ? 'text-white' : 'text-amber-500'} />
        <span>All Downloads</span>
      </div>
      <span class="font-mono text-[10px] {selectedCategory === 'all' ? 'text-blue-100' : 'text-slate-500'}">
        {counts.all}
      </span>
    </button>

    <!-- Subcategories (Indented) -->
    <div class="pl-4 space-y-0.5 border-l border-slate-200 ml-3">
      {#each categories as cat}
        <button
          type="button"
          on:click={() => (selectedCategory = cat.id)}
          class="w-full flex items-center justify-between px-2 py-1 rounded text-left transition-colors {selectedCategory === cat.id
            ? 'bg-blue-600 text-white font-semibold'
            : 'hover:bg-slate-200 text-slate-700'}"
        >
          <div class="flex items-center gap-1.5">
            <Icons name="folder" size={13} className={selectedCategory === cat.id ? 'text-white' : 'text-slate-400'} />
            <span class="truncate">{cat.label}</span>
          </div>
          {#if counts[cat.key] > 0}
            <span class="font-mono text-[10px] {selectedCategory === cat.id ? 'text-blue-100' : 'text-slate-400'}">
              {counts[cat.key]}
            </span>
          {/if}
        </button>
      {/each}
    </div>

    <div class="border-t border-slate-200 my-2"></div>

    <!-- Status Groups -->
    <div class="space-y-0.5">
      <!-- Unfinished -->
      <button
        type="button"
        on:click={() => (selectedCategory = 'unfinished')}
        class="w-full flex items-center justify-between px-2.5 py-1 rounded text-left transition-colors {selectedCategory === 'unfinished'
          ? 'bg-blue-600 text-white font-semibold'
          : 'hover:bg-slate-200 text-slate-800'}"
      >
        <div class="flex items-center gap-2">
          <Icons name="clock" size={14} className={selectedCategory === 'unfinished' ? 'text-white' : 'text-amber-600'} />
          <span>Unfinished</span>
        </div>
        <span class="font-mono text-[10px] {selectedCategory === 'unfinished' ? 'text-blue-100' : 'text-slate-500'}">
          {counts.unfinished}
        </span>
      </button>

      <!-- Finished -->
      <button
        type="button"
        on:click={() => (selectedCategory = 'finished')}
        class="w-full flex items-center justify-between px-2.5 py-1 rounded text-left transition-colors {selectedCategory === 'finished'
          ? 'bg-blue-600 text-white font-semibold'
          : 'hover:bg-slate-200 text-slate-800'}"
      >
        <div class="flex items-center gap-2">
          <Icons name="check" size={14} className={selectedCategory === 'finished' ? 'text-white' : 'text-emerald-600'} />
          <span>Finished</span>
        </div>
        <span class="font-mono text-[10px] {selectedCategory === 'finished' ? 'text-blue-100' : 'text-slate-500'}">
          {counts.finished}
        </span>
      </button>
    </div>
  </div>
</aside>

