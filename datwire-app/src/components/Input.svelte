<script lang="ts">
  export let type: string = "text";
  export let name: string | undefined = undefined;
  export let placeholder: string | undefined = undefined;
</script>

<input 
  class="border-2 rounded-full border-cyan-900 px-1.5 py-0.25" 
  type={type} 
  name={name} 
  placeholder={placeholder}
>
