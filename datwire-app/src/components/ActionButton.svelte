<script lang="ts">
  import ArrowPath from 'svelte-heros-v2/ArrowPath.svelte';
  export let type: "submit" | "reset" | "button" = "button";
  export let name: string | null = null;
  export let disabled: boolean = false;
  export let loading: boolean = false;
</script>

<button 
  class="
    flex flex-row justify-center items-center text-center align-middle
    px-2 py-1.5 m-1 rounded-xl
    bg-victoria-500 text-white 
    hover:bg-victoria-600
    focus:bg-victoria-800 focus:text-victoria-100 focus:outline-none focus:ring focus:ring-victoria-200 
    disabled:bg-victoria-300 disabled:text-gray-50 disabled:bg-opacity-80 
    transition-colors
    [&>*]:inline-block
  "
  type={type} 
  name={name}
  disabled={disabled}
  aria-disabled={disabled}
  on:click
  on:dblclick
  on:focus
>
  {#if loading}
    <ArrowPath strokeWidth="2" class="w-[1.25em] h-[1.25em] mr-1 animate-spin" />
  {/if}
  <slot />
</button>
