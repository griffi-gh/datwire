<script lang="ts">
  import { fly } from 'svelte/transition';
</script>

<style lang="postcss">
  :global(html) {
    overflow: hidden;
  }
  :global(html) {
    background: url(/bg-generic.jpg);
    background-position: center;
    background-size: cover;
  }
</style>

<div class="grid place-items-center min-h-screen">
  <div 
    class="bg-victoria-50 p-5 rounded-lg shadow-xl shadow-gray-400" 
    in:fly="{{ y: -20, duration: 500, delay: 550 }}"
    out:fly="{{ y: -20, duration: 500 }}" 
  >
    <slot/>
  </div> 
</div>
