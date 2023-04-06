<script lang="ts">
  export let type: "text" | "email" | "password" = "text";
  export let name: string | null = null;
  export let label: string | null = null;
  export let placeholder: string = "";
  export let id: string = name + '$' + (100000 + Math.floor(Math.random() * 899999)).toString();
  export let required: boolean = false;
  export let disabled: boolean = false;
  export let autocomplete: string = ({
    'email': 'email',
    'password': 'current-password',
    'text': 'on'
  })[type];
  export let pattern: string | null = null;
  export let minLength: number | null = null;
  export let maxLength: number | null = null;
  export let value: string = "";
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
    {...{ type }}
    name={name}
    placeholder={placeholder + " "}
    id={id}
    required={required}
    disabled={disabled}
    pattern={pattern}
    autocomplete={autocomplete}
    minlength={minLength}
    maxlength={maxLength}
    bind:value
    on:input
    on:change
    on:focus
  >
  {#if label}
    <label for={id} class="pl-1 uppercase font-semibold text-xs text-gray-900 peer-placeholder-shown:!text-gray-900 peer-invalid:text-pink-800 peer-valid:text-green-700 transition-colors">{label}</label>
  {/if}
</div>
