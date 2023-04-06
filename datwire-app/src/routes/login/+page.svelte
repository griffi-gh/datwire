<script lang="ts">
  import DialogPage from "$components/DialogPage.svelte";
  import TextInput from "$components/TextInput.svelte";
  import ActionButton from "$components/ActionButton.svelte";
  import { _ } from 'svelte-i18n';
</script>

<svelte:head>
  <title>Datwire - Login</title>
</svelte:head>

<DialogPage>
  <h1 class="text-2xl text-center m-4">{$_('login_flow.welcome')}</h1>
  <form class="flex flex-col w-[20rem]">
    <TextInput type="email" name="email" placeholder="user@mail.com" label={$_('common.email')} autocomplete="email" required />
    <TextInput type="password" name="password" placeholder="Tr0ub4dor&3" label={$_('common.password')} minLength={8} autocomplete="current-password" required/>
    <div class="my-2 flex flex-col">
      <ActionButton type="submit">{$_('login_flow.btn_login')}</ActionButton>
    </div>
  </form>
  <a href="/register" class="text-center underline block">
    {$_('login_flow.link_register')}
  </a>
</DialogPage>
