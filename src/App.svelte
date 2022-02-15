<!--
 Copyright (C) 2022 Guyutongxue
 
 This file is part of vscch4.
 
 vscch4 is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.
 
 vscch4 is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with vscch4.  If not, see <http://www.gnu.org/licenses/>.
-->
<script lang="ts">
  import type { Unsubscriber, Writable } from "svelte/store";
  import { onDestroy, onMount } from "svelte";
  import { process } from "@tauri-apps/api";
  import Icon from "@iconify/svelte";

  import Begin from "./Begin.svelte";
  import Vscode from "./Vscode.svelte";
  import Compiler from "./Compiler.svelte";
  import Workspace from "./Workspace.svelte";
  import Options from "./Options.svelte";
  import { vscode, compiler, workspace, options, done } from "./config_store";
  import Done from "./Done.svelte";

  type Step = {
    label: string;
    readonly component: any;
    resultWritable?: Writable<any>;
  };
  const STEPS: Step[] = [
    {
      label: "开始",
      component: Begin,
    },
    {
      label: "VS Code",
      component: Vscode,
      resultWritable: vscode,
    },
    {
      label: "编译器",
      component: Compiler,
      resultWritable: compiler,
    },
    {
      label: "工作文件夹",
      component: Workspace,
      resultWritable: workspace,
    },
    {
      label: "配置选项",
      component: Options,
      resultWritable: options,
    },
    {
      label: "完成",
      component: Done,
      resultWritable: done,
    },
  ];
  const LEN = STEPS.length;
  let step = 0;

  let results: any[] = [];
  const subscribers: Unsubscriber[] = [];

  let card: HTMLElement;
  async function go(d: number) {
    if (step + d === LEN) {
      process.exit(0);
      return;
    }
    card.style.setProperty("--direction", `${50 * d}px`);
    card.classList.add("slide");
    await new Promise((resolve) => setTimeout(resolve, 160));
    step += d;
    await new Promise((resolve) => setTimeout(resolve, 160));
    card.classList.remove("slide");
    card.style.setProperty("--direction", null);
  }

  async function changeBackground() {
    const n = new Date().getTime();
    document.body.style.setProperty(
      "--background",
      `url("https://www.dmoe.cc/random.php?v=${n}")`
    );
  }

  onMount(() => {
    for (const i in STEPS) {
      const subscribe = STEPS[i].resultWritable?.subscribe((v) => {
        results[i] = v;
      });
      if (subscribe) {
        subscribers.push(subscribe);
      }
    }
  });

  onDestroy(() => {
    for (const i in subscribers) {
      subscribers[i]();
    }
  });
</script>

<div class="fixed top-2 right-2 z-10">
  <button
    class="btn btn-sm glass tooltip tooltip-left"
    on:click={changeBackground}
    data-tip="换一张背景"
  >
    <Icon icon="mdi:refresh" />
  </button>
</div>
<main class="flex flex-row justify-center items-center w-full h-full pb-20">
  <div
    id="card"
    bind:this={card}
    class="card glass shadow-lg hover:shadow-lg w-9/12 max-w-3xl"
  >
    <div class="card-body">
      {#each STEPS as s, i (i)}
        {#if i <= step}
          <div class:hidden={i != step}>
            <svelte:component this={s.component} />
          </div>
        {/if}
      {/each}
      <div class="justify-end card-actions !mt-3">
        <button
          class="btn btn-ghost"
          class:invisible={step === 0 || step === LEN - 1}
          on:click={() => go(-1)}
        >
          上一步
        </button>
        <button
          class="btn btn-primary"
          disabled={results[step] === null}
          on:click={() => go(1)}
        >
          {#if step === LEN - 1}
            完成
          {:else}
            下一步
          {/if}
        </button>
      </div>
    </div>
  </div>
</main>
<footer class="fixed glass bottom-0 w-full">
  <ul class="w-full steps h-20 items-center">
    {#each STEPS as s, i}
      <li
        class="step before:transition after:transition"
        class:step-primary={step >= i}
        on:click={() => step !== LEN - 1 && step > i && go(i - step)}
      >
        {s.label}
      </li>
    {/each}
  </ul>
</footer>

<style>
  main {
    perspective: 1000px;
  }

  :global(.slide) {
    animation: 320ms ease-in-out slide-animation;
  }

  @keyframes slide-animation {
    from {
      transform: translateX(0);
      opacity: 1;
    }
    49% {
      transform: translateX(calc(var(--direction) * -1));
      opacity: 0;
    }
    51% {
      transform: translateX(var(--direction));
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  /* bg img with opacity */
  main:before {
    content: " ";
    display: block;
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    opacity: 0.5;
    background: no-repeat center/cover var(--background);
  }
</style>
