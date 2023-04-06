<script lang="ts">
  import DialogPage from "../../components/DialogPage.svelte";
  import TextInput from "$components/TextInput.svelte";
  import ActionButton from "$components/ActionButton.svelte";

  let _test = false;
</script>

<svelte:head>
  <title>Datwire - Join</title>
</svelte:head>

<DialogPage>
  <h1 class="text-2xl text-center m-4">Join Datwire</h1>
  <form class="flex flex-col">
    <TextInput type="email" name="email" placeholder="user@mail.com" label="Email" required />
    <TextInput type="password" name="password" placeholder="Tr0ub4dor&3" label="Password" pattern=".&#123;8,}" autocomplete="new-password" required />
    <TextInput type="password" name="repeat_password" placeholder="Tr0ub4dor&3" label="Repeat password" autocomplete="new-password" required />
    <div class="my-2 flex flex-col">
      <ActionButton type="submit" loading={_test} on:click={(() => _test = true)}>Create new account</ActionButton>
    </div>
  </form>
  <a href="/login" class="text-blue text-center underline block">
    Log in instead
  </a>
</DialogPage>
