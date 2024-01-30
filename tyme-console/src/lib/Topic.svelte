<script>
  import { createEventDispatcher } from "svelte";

  export let text;
  export let index;
  export let topicIndex;

  const dispatch = createEventDispatcher();

  $: checked = index === topicIndex;

  const changeTopic = () => {
    dispatch("changeTopic", index);
  };
</script>

<div class="md:w-full md:h-14 h-full md:mb-2 mr-2">
  <button
    class:active={checked}
    class="w-full h-full px-2"
    on:click={changeTopic}>{text}</button
  >
</div>

<style>
  button {
    display: inline-block;
    transition: all 0.2s ease-in;
    position: relative;
    overflow: hidden;
    z-index: 1;
    color: #090909;
    padding: 0em 1.7em;
    cursor: pointer;
    font-size: 18px;
    border-radius: 0.3em;
    background: #e8e8e8;
  }

  button:active {
    color: #666;
  }

  button:before {
    content: "";
    position: absolute;
    left: 50%;
    transform: translateX(-50%) scaleY(1) scaleX(1.25);
    top: 100%;
    width: 140%;
    height: 180%;
    background-color: rgba(0, 0, 0, 0.05);
    border-radius: 50%;
    display: block;
    transition: all 0.5s 0.1s cubic-bezier(0.55, 0, 0.1, 1);
    z-index: -1;
  }

  button:after {
    content: "";
    position: absolute;
    left: 55%;
    transform: translateX(-50%) scaleY(1) scaleX(1.45);
    top: 180%;
    width: 160%;
    height: 190%;
    background-color: #009087;
    border-radius: 50%;
    display: block;
    transition: all 0.5s 0.1s cubic-bezier(0.55, 0, 0.1, 1);
    z-index: -1;
  }

  button:hover {
    color: #ffffff;
  }

  button:hover:before {
    top: -35%;
    background-color: #009087;
    transform: translateX(-50%) scaleY(1.3) scaleX(0.8);
  }

  button:hover:after {
    top: -45%;
    background-color: #009087;
    transform: translateX(-50%) scaleY(1.3) scaleX(0.8);
  }
  .active {
    background-color: #009087;
  }
</style>
