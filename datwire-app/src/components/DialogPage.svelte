<script lang="ts">
  import { fly } from 'svelte/transition';
</script>

<div class="grid place-items-center">
  <div 
    class="" 
    in:fly="{{ y: -50, duration: 250, delay: 300 }}"
    out:fly="{{ y: 50, duration: 250 }}" 
  >
    <slot/>
  </div> 
</div>
