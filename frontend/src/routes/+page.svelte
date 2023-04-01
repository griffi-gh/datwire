<script lang="ts">
  let meows: string[] = [];
  function addMeow() {
    const o = Math.floor(Math.random() * 10) + 1;
    meows.push(`me${"o".repeat(o)}w`)
    meows = meows;
  }
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<p>Amount {meows.length}</p>
<button type="button" on:click={addMeow} disabled={meows.length >= 30}>
  {#if meows.length == 0}
    Add your first meow
  {:else if meows.length >= 30}
    Sorry, that's just too much
  {:else}
    Add moarrr meows!
  {/if}
</button>
{#if meows.length >= 10}
  <p>That's a lot of meows...</p>
{/if}
{#if meows.length >= 20}
  <p>Please just stop</p>
{/if}
{#if meows.length >= 30}
  <p>Fuck you</p>
{/if}
<ul>
  {#each meows as meow}
    <li>{meow}</li>
  {/each}
</ul>
