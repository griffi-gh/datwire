<script lang="ts">
  import { PUBLIC_API_HOST } from '$env/static/public';
  import { submitFormJson } from '$utils/request.ts';
  import DialogPage from "../../components/DialogPage.svelte";
  import TextInput from "$components/TextInput.svelte";
  import ActionButton from "$components/ActionButton.svelte";
  import ExclamationCircle from 'svelte-heros-v2/ExclamationCircle.svelte';
  import { goto } from '$app/navigation';
  import { _ } from 'svelte-i18n';

  let loading = false;
  let password = "";
  let error: string | null = null;

  async function onSubmit(event: SubmitEvent) {
    loading = true;
    const form = event.currentTarget! as HTMLFormElement;
    try {
      const res = await submitFormJson(form);
      console.log(res);
      loading = false;
      if (res.success) {
        goto('/login');
      } else if (res.error) {
        throw new Error($_('error.register')[res.error]);
      }
    } catch(err: any) {
      console.error(err);
      error = err.toString();
      loading = false;
    }
  }

  function escapeRegex(string: string) {
    return string.replace(/[/\-\\^$*+?.()|[\]{}]/g, '\\$&');
  }
</script>

<svelte:head>
  <title>Datwire - Join</title>
</svelte:head>

<DialogPage>
  <h1 class="text-2xl text-center m-4">{$_('register_flow.welcome')}</h1>
  <form method="post" action={`${PUBLIC_API_HOST}/api/register`} class="flex flex-col w-[50vw] min-w-[20rem] max-w-[40rem]" on:submit|preventDefault={onSubmit}>
    <TextInput name="handle" placeholder="ViperGamingX" label={$_('handle')} required />
    <TextInput type="email" name="email" placeholder="user@mail.com" label={$_('email')} required />
    <div class="flex flex-wrap [&>*]:flex-1">
      <TextInput type="password" name="password" placeholder="Tr0ub4dor&3" label={$_('password')} minLength={8} autocomplete="new-password" required bind:value={password} />
      <TextInput type="password" name="repeat_password" placeholder="Tr0ub4dor&3" label={$_('password_repeat')} minLength={8} pattern={escapeRegex(password)} autocomplete="new-password" required />
    </div>
    {#if error != null}
      <p class="text-pink-700 font-bold px-3 align-middle flex items-center">
        <ExclamationCircle class="inline-block w-[1.5em] h-[1.5em]"/>{error}
      </p>
    {/if}
    <div class="my-2 flex flex-col">
      <ActionButton type="submit" loading={loading} disabled={loading}>
        {$_('register_flow.btn_create')}
      </ActionButton>
    </div>
  </form>
  <a href="/login" class="text-blue text-center underline block">
    {$_('register_flow.link_login')}
  </a>
</DialogPage>
