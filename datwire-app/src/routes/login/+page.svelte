<script>
  import DialogPage from "$components/DialogPage.svelte";
  import TextInput from "$components/TextInput.svelte";
</script>

<svelte:head>
  <title>Datwire - Register</title>
</svelte:head>

<DialogPage>
  <h1 class="text-2xl text-center m-4">Welcome back</h1>
  <form class="flex flex-col">
    <TextInput type="email" name="email" placeholder="user@mail.com" label="Email" autocomplete="email" required />
    <TextInput type="password" name="password" placeholder="Tr0ub4dor&3" label="Password" pattern=".&#123;8,}" autocomplete="current-password" required/>
    <button type="submit">Login</button>
  </form>
  <a href="/register" class="text-blue text-center underline block">
    Don't have an account?
  </a>
</DialogPage>
