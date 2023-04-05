<script>
  import DialogPage from "$components/DialogPage.svelte";
  import Input from "$components/Input.svelte";
</script>

<DialogPage>
  <h1>Login</h1>
  <form class="flex flex-col">
    <Input type="email" name="email" placeholder="Email" />
    <Input type="password" name="password" placeholder="Password" />
    <button type="submit">Login</button>
  </form>
  <a href="/register">
    Don't have an account?
  </a>
</DialogPage>
