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

<div class="md:w-full md:h-14 w-20 h-full md:mb-2 mr-2">
  <button class:active={checked} class="w-full h-full" on:click={changeTopic}>
    <span class="circle1" />
    <span class="circle2" />
    <span class="circle3" />
    <span class="circle4" />
    <span class="circle5" />
    <span class="text">{text}</span>
  </button>
</div>

<style>
  button {
    font-family: Arial, Helvetica, sans-serif;
    font-weight: bold;
    color: white;
    background-color: #5e5e5e;
    border: none;
    border-radius: 0.3rem;
    position: relative;
    cursor: pointer;
    overflow: hidden;
  }

  button span:not(:nth-child(6)) {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    height: 50px;
    width: 50px;
    background-color: #7faef5;

    border-radius: 50%;
    transition: 0.6s ease;
  }

  button span:nth-child(6) {
    position: relative;
  }

  button span:nth-child(1) {
    transform: translate(-6rem, -5rem);
  }

  button span:nth-child(2) {
    transform: translate(-6rem, 1.8rem);
  }

  button span:nth-child(3) {
    transform: translate(-0.8rem, 1.8rem);
  }

  button span:nth-child(4) {
    transform: translate(3.5rem, 2rem);
  }

  button span:nth-child(5) {
    transform: translate(3.5rem, -5rem);
  }

  button:hover span:not(:nth-child(6)) {
    transform: translate(-50%, -50%) scale(4);
    transition: 1.5s ease;
  }

  button:active span:not(:nth-child(6)) {
    transform: translate(-50%, -50%) scale(4);
    transition: 1.5s ease;
  }

  button.active span:not(:nth-child(6)) {
    transform: translate(-50%, -50%) scale(4);
    background-color: #0c66ed;
  }
</style>
