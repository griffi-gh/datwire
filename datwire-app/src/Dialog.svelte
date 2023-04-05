<script lang="ts">
  import { fly } from 'svelte/transition';
</script>

<style lang="scss">
  :global(html) {
    overflow: hidden;
  }
  :global(body) {
    background: linear-gradient(to bottom, purple, black);
    font-family: 'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif
  }
  :global(a) {
    color: lightblue;
  }
  .dialog-bg {
    min-height: 100%;
  }
  .dialog-bg, .dialog {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
  }
  .dialog {
    background: gray;
    color: white;
    padding: 2rem;
    border-radius: 2rem;
  }
</style>

<div class="dialog-bg">
  <div 
    class="dialog" 
    in:fly="{{ y: -50, duration: 250, delay: 300 }}"
    out:fly="{{ y: 50, duration: 250 }}" 
  >
    <slot/>
  </div> 
</div>
