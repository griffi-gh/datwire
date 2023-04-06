<script lang="ts">
  export let type: string = "text";
  export let name: string | undefined = undefined;
  export let placeholder: string = "";
  export let label: string | undefined = undefined;
  export let id: string = name + '$' + (100000 + Math.floor(Math.random() * 899999)).toString();
  export let required: boolean = false;
  export let value: any = undefined;
  function handleInput(event: any) {
    value = event.target.value;
  }
</script>

<div class="flex flex-col-reverse m-1">
  <input 
    class="
      peer rounded-lg border px-3 py-1 mt-0.5 focus:outline-none
      border-gray-500 bg-gray-100 
      placeholder:text-gray-400 focus:bg-victoria-200 focus:border-victoria-400 focus:ring focus:ring-victoria-100 focus:placeholder:text-victoria-400
      valid:border-green-700 valid:bg-green-100 valid:focus:border-green-700 valid:focus:bg-green-200 valid:focus:ring-green-100 valid:text-green-900
      invalid:[&:not(:placeholder-shown)]:border-pink-500 invalid:[&:not(:placeholder-shown)]:bg-pink-100 invalid:[&:not(:placeholder-shown)]:focus:border-pink-600 invalid:[&:not(:placeholder-shown)]:focus:bg-pink-200 invalid:[&:not(:placeholder-shown)]:focus:ring-pink-100 invalid:[&:not(:placeholder-shown)]:text-pink-900
      transition-colors
    " 
    type={type}
    name={name}
    placeholder={placeholder + " "}
    id={id}
    required={required}
    on:input={handleInput}
    {...$$restProps}
  >
  {#if label}
    <label for={id} class="pl-1 uppercase font-semibold text-xs text-gray-900 peer-placeholder-shown:!text-gray-900 peer-invalid:text-pink-800 peer-valid:text-green-700 transition-colors">{label}</label>
  {/if}
</div>
