<script>
  import DialogPage from "$components/DialogPage.svelte";
  import InputBox from "$components/InputBox.svelte";
</script>

<DialogPage>
  <h1 class="text-2xl text-center m-4">Welcome back</h1>
  <form class="flex flex-col">
    <InputBox type="email" name="email" placeholder="user@mail.com" label="Email" autocomplete="email" required />
    <InputBox type="password" name="password" placeholder="Tr0ub4dor&3" label="Password" pattern=".........*" autocomplete="current-password" required />
    <button type="submit">Login</button>
  </form>
  <a href="/register" class="text-blue text-center underline block">
    Don't have an account?
  </a>
</DialogPage>
