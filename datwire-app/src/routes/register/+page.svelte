<script lang="ts">
  import DialogPage from "../../components/DialogPage.svelte";
  import InputBox from "$components/InputBox.svelte";
  let password = "";
</script>

<DialogPage>
  <h1 class="text-2xl text-center m-4">Join Datwire</h1>
  <form class="flex flex-col" autocomplete="off">
    <InputBox type="email" name="email" placeholder="user@mail.com" label="Email" required autocomplete="off" />
    <InputBox type="password" name="password" placeholder="Tr0ub4dor&3" label="Password" pattern=".........*" autocomplete="new-password" required bind:value={ password } />
    <InputBox type="password" name="repeat_password" placeholder="Tr0ub4dor&3" label="Repeat password" autocomplete="new-password" required pattern={ (password.length >= 8) ? password.replace((/[.*+?^${}()|[\]\\]/g), '\\$&') : "(?!)" } />
    <button type="submit">Login</button>
    <h1>{password}</h1>
  </form>
  <a href="/login">
    Log in instead
  </a>
</DialogPage>
