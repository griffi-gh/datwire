<script lang="ts">
  import DialogPage from "$components/DialogPage.svelte";
  import TextInput from "$components/TextInput.svelte";
  import ActionButton from "$components/ActionButton.svelte";
</script>

<svelte:head>
  <title>Datwire - Login</title>
</svelte:head>

<DialogPage>
  <h1 class="text-2xl text-center m-4">Welcome back</h1>
  <form class="flex flex-col w-[20rem]">
    <TextInput type="email" name="email" placeholder="user@mail.com" label="Email" autocomplete="email" required />
    <TextInput type="password" name="password" placeholder="Tr0ub4dor&3" label="Password" pattern=".&#123;8,}" autocomplete="current-password" required/>
    <div class="my-2 flex flex-col">
      <ActionButton type="submit">Login</ActionButton>
    </div>
  </form>
  <a href="/register" class="text-blue text-center underline block">
    Don't have an account yet?
  </a>
</DialogPage>
