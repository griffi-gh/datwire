<script lang="ts">
  import { fly } from 'svelte/transition';
</script>

<style lang="scss">
  :global(body) {
    background: pink;
  }
  .dialog-bg {
    
  }
  .dialog, .dialog-bg {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
  }
</style>

<div class="dialog-bg">
  <div 
    class="dialog" 
    in:fly="{{ y: -50, duration: 250, delay: 300 }}"
    out:fly="{{ y: -50, duration: 250 }}" 
  >
    <slot/>
  </div> 
</div>
