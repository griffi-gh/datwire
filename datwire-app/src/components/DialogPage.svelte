<script lang="ts">
  import { send, receive } from "$utils/crossfade.ts";
</script>

<style lang="postcss">
  :global(.ABONGA) {
    overflow: hidden;
    background: url(/bg-generic.jpg);
    background-position: center;
    background-size: cover;
  }
</style>

<div class="ABONGA grid place-items-center min-h-screen w-screen absolute">
  <div
    class="bg-victoria-50 p-5 rounded-lg shadow-lg shadow-victoria-200 !opacity-100"
    in:receive|local={{ key: "dialog-component" }}
    out:send|local={{ key: "dialog-component" }}
  >
    <slot/>
  </div> 
</div>
