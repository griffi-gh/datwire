<script lang="ts">
  import { PUBLIC_API_HOST } from '$env/static/public';
  import DialogPage from "../../components/DialogPage.svelte";
  import TextInput from "$components/TextInput.svelte";
  import ActionButton from "$components/ActionButton.svelte";
  import ExclamationCircle from 'svelte-heros-v2/ExclamationCircle.svelte';
    import { goto } from '$app/navigation';

  let loading = false;
  let password = "";
  let error: string | null = null;

  async function onSubmit(event: SubmitEvent) {
    loading = true;
    const form = event.currentTarget! as HTMLFormElement;
    const formData = new FormData(form);
    const formDataObject = Object.fromEntries(formData.entries());
    const formDataJsonString = JSON.stringify(formDataObject);
    try {
      const res = await fetch(form.action, {
        method: form.method.toUpperCase(),
        credentials: 'omit',
        headers: {
          "Content-Type": "application/json",
          "Accept": "application/json"
        },
        body: formDataJsonString,
      }).then(res => res.json());
      console.log(res);
      if (res.success) {
        console.log("Yay!");
        goto('/login');
      } else if (res.error) {
        error = res.error.message;
      }
      loading = false;
    } catch(e: any) {
      console.error(e);
      error = e.toString();
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
  <h1 class="text-2xl text-center m-4">Join Datwire</h1>
  <form method="post" action={`${PUBLIC_API_HOST}/api/register`} class="flex flex-col w-[50vw] min-w-[20rem] max-w-[40rem]" on:submit|preventDefault={onSubmit}>
    <TextInput name="handle" placeholder="ViperGamingX" label="Handle" required />
    <TextInput type="email" name="email" placeholder="user@mail.com" label="Email" required />
    <div class="flex flex-wrap [&>*]:flex-1">
      <TextInput type="password" name="password" placeholder="Tr0ub4dor&3" label="Password" minLength={8} autocomplete="new-password" required bind:value={password} />
      <TextInput type="password" name="repeat_password" placeholder="Tr0ub4dor&3" label="Repeat password" minLength={8} pattern={escapeRegex(password)} autocomplete="new-password" required />
    </div>
    {#if error != null}
      <p class="text-pink-700 font-bold px-3 align-middle flex items-center">
        <ExclamationCircle class="inline-block w-[1.5em] h-[1.5em]"/>{error}
      </p>
    {/if}
    <div class="my-2 flex flex-col">
      <ActionButton type="submit" loading={loading} disabled={loading}>Create new account</ActionButton>
    </div>
  </form>
  <a href="/login" class="text-blue text-center underline block">
    Log in instead?
  </a>
</DialogPage>
