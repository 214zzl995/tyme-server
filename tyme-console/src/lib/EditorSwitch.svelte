<script>
  import { onMount } from "svelte";
  export let value;

  $: checked = value === "MarkDown";

  onMount(() => {
    const editorType = localStorage.getItem("editorType");
    if (editorType) {
      value = editorType;
    }else{
      value = "MarkDown";
    }
  });

  $: {
    if (value !== undefined) {
      localStorage.setItem("editorType", value);
    }
  }
</script>

<div>
  <input
    type="checkbox"
    {checked}
    id="favorite"
    name="favorite-checkbox"
    value="favorite-button"
    on:click={() => {
      value = checked ? "Json" : "MarkDown";
    }}
  />
  <label for="favorite" class="container">
    <div class="action">
      <span class="option-1">Json</span>
      <span class="option-2">MarkDown</span>
    </div>
  </label>
</div>

<style>
  label {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
    background-color: white;
    display: flex;
    align-items: center;
    gap: 14px;
    cursor: pointer;
    user-select: none;
    border-radius:0.5rem;
    color: black;
    font-size: 0.7rem;
    font-weight: 600;
    text-align: center;
  }

  input {
    display: none;
  }

  input:checked + label {
    fill: hsl(0deg 100% 50%);
    stroke: hsl(0deg 100% 50%);
    animation: heartButton 1s;
  }

  input + label .action {
    position: relative;
    overflow: hidden;
    display: grid;
  }

  input + label .action span {
    grid-column-start: 1;
    grid-column-end: 1;
    grid-row-start: 1;
    grid-row-end: 1;
    transition: all 0.5s;
  }

  input + label .action span.option-1 {
    transform: translate(0px, 0%);
    opacity: 1;
  }

  input:checked + label .action span.option-1 {
    transform: translate(0px, -100%);
    opacity: 0;
  }

  input + label .action span.option-2 {
    transform: translate(0px, 100%);
    opacity: 0;
  }

  input:checked + label .action span.option-2 {
    transform: translate(0px, 0%);
    opacity: 1;
  }
</style>
