<script lang="ts">
  import Icons from './Icons.svelte';

  export let onAddUrl: () => void;
  export let onResume: () => void;
  export let onStop: () => void;
  export let onStopAll: () => void;
  export let onDelete: () => void;
  export let onOptions: () => void;
  export let onScheduler: () => void;

  export let canResume: boolean = false;
  export let canStop: boolean = false;
  export let canDelete: boolean = false;

  export let searchQuery: string = '';
</script>

<div class="h-16 bg-gradient-to-b from-slate-50 to-slate-100 border-b border-slate-300 px-3 flex items-center justify-between select-none">
  <!-- IDM Action Buttons -->
  <div class="flex items-center gap-1">
    <!-- Add URL -->
    <button
      type="button"
      on:click={onAddUrl}
      class="flex flex-col items-center justify-center w-14 h-13 rounded border border-transparent hover:border-slate-300 hover:bg-white active:bg-slate-200 transition-colors text-slate-700"
      title="Add new download URL"
    >
      <div class="text-blue-600">
        <Icons name="add-url" size={20} strokeWidth={2.2} />
      </div>
      <span class="text-[10px] font-medium mt-0.5 leading-none">Add URL</span>
    </button>

    <div class="w-px h-8 bg-slate-300 mx-1"></div>

    <!-- Resume -->
    <button
      type="button"
      on:click={onResume}
      disabled={!canResume}
      class="flex flex-col items-center justify-center w-14 h-13 rounded border border-transparent hover:border-slate-300 hover:bg-white active:bg-slate-200 disabled:opacity-40 disabled:hover:bg-transparent disabled:hover:border-transparent transition-colors text-slate-700"
      title="Resume selected download"
    >
      <div class="text-emerald-600">
        <Icons name="resume" size={18} strokeWidth={2.2} />
      </div>
      <span class="text-[10px] font-medium mt-0.5 leading-none">Resume</span>
    </button>

    <!-- Stop (Pause) -->
    <button
      type="button"
      on:click={onStop}
      disabled={!canStop}
      class="flex flex-col items-center justify-center w-14 h-13 rounded border border-transparent hover:border-slate-300 hover:bg-white active:bg-slate-200 disabled:opacity-40 disabled:hover:bg-transparent disabled:hover:border-transparent transition-colors text-slate-700"
      title="Stop selected download"
    >
      <div class="text-amber-600">
        <Icons name="stop" size={18} strokeWidth={2.2} />
      </div>
      <span class="text-[10px] font-medium mt-0.5 leading-none">Stop</span>
    </button>

    <!-- Stop All -->
    <button
      type="button"
      on:click={onStopAll}
      class="flex flex-col items-center justify-center w-14 h-13 rounded border border-transparent hover:border-slate-300 hover:bg-white active:bg-slate-200 transition-colors text-slate-700"
      title="Stop all active downloads"
    >
      <div class="text-red-600">
        <Icons name="stop-all" size={18} strokeWidth={2.2} />
      </div>
      <span class="text-[10px] font-medium mt-0.5 leading-none">Stop All</span>
    </button>

    <div class="w-px h-8 bg-slate-300 mx-1"></div>

    <!-- Delete -->
    <button
      type="button"
      on:click={onDelete}
      disabled={!canDelete}
      class="flex flex-col items-center justify-center w-14 h-13 rounded border border-transparent hover:border-slate-300 hover:bg-white active:bg-slate-200 disabled:opacity-40 disabled:hover:bg-transparent disabled:hover:border-transparent transition-colors text-slate-700"
      title="Delete selected download"
    >
      <div class="text-slate-600">
        <Icons name="delete" size={18} strokeWidth={2.2} />
      </div>
      <span class="text-[10px] font-medium mt-0.5 leading-none">Delete</span>
    </button>

    <div class="w-px h-8 bg-slate-300 mx-1"></div>

    <!-- Options -->
    <button
      type="button"
      on:click={onOptions}
      class="flex flex-col items-center justify-center w-14 h-13 rounded border border-transparent hover:border-slate-300 hover:bg-white active:bg-slate-200 transition-colors text-slate-700"
      title="Options & Settings"
    >
      <div class="text-slate-600">
        <Icons name="settings" size={18} strokeWidth={2.2} />
      </div>
      <span class="text-[10px] font-medium mt-0.5 leading-none">Options</span>
    </button>

    <!-- Scheduler -->
    <button
      type="button"
      on:click={onScheduler}
      class="flex flex-col items-center justify-center w-14 h-13 rounded border border-transparent hover:border-slate-300 hover:bg-white active:bg-slate-200 transition-colors text-slate-700"
      title="Scheduler"
    >
      <div class="text-slate-600">
        <Icons name="clock" size={18} strokeWidth={2.2} />
      </div>
      <span class="text-[10px] font-medium mt-0.5 leading-none">Scheduler</span>
    </button>
  </div>

  <!-- Search / Quick Filter -->
  <div class="relative w-64">
    <span class="absolute inset-y-0 left-0 pl-2.5 flex items-center pointer-events-none text-slate-400">
      <Icons name="search" size={13} />
    </span>
    <input
      type="text"
      bind:value={searchQuery}
      placeholder="Find download..."
      class="w-full pl-8 pr-3 py-1 text-xs bg-white border border-slate-300 rounded text-slate-900 placeholder:text-slate-400 focus:outline-none focus:border-blue-500"
    />
  </div>
</div>

