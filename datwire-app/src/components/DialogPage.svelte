<script lang="ts">
  import { fly } from 'svelte/transition';
</script>

<div class="grid place-items-center">
  <div 
    class="" 
    in:fly="{{ y: -10, duration: 500, delay: 550 }}"
    out:fly="{{ y: -10, duration: 500 }}" 
  >
    <slot/>
  </div> 
</div>
