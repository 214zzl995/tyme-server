<script>
  export let value = "MarkDown";

  $: checked = value === "MarkDown";
</script>

<div>
  <input
    type="checkbox"
    {checked}
    id="favorite"
    name="favorite-checkbox"
    value="favorite-button"
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
    background-color: white;
    display: flex;
    align-items: center;
    gap: 14px;
    cursor: pointer;
    user-select: none;
    border-radius: 10px;
    color: black;
    font-size: 0.7rem;
    font-weight: 600;
    text-align: end;
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
