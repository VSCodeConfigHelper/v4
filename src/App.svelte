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
  import { vscode, compiler, workspace, options } from "./config_store";
  import type { Unsubscriber, Writable } from "svelte/store";
  import { onDestroy, onMount } from "svelte";
  import Icon from "@iconify/svelte";

  import Begin from "./Begin.svelte";
  import Vscode from "./Vscode.svelte";
import Compiler from "./Compiler.svelte";

  type Step = {
    label: string;
    component: any;
    resultWritable?: Writable<any>;
  };
  const steps: Step[] = [
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
      component: null,
      resultWritable: workspace,
    },
    {
      label: "配置选项",
      component: null,
      resultWritable: options,
    },
    {
      label: "完成",
      component: null,
    },
  ];
  let step = 0;

  let results: any[] = [];
  const subscribers: Unsubscriber[] = [];

  async function changeBackground() {
    const n = new Date().getTime();
    document.body.style.setProperty(
      "--background",
      `url("https://www.dmoe.cc/random.php?v=${n}")`
    );
  }

  onMount(() => {
    for (const i in steps) {
      const subscribe = steps[i].resultWritable?.subscribe((v) => {
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
<main class="flex flex-row justify-center items-center w-full h-full">
  <div class="card glass shadow-lg hover:shadow-lg w-9/12 max-w-3xl">
    <div class="card-body">
      <svelte:component this={steps[step].component} />
      <div class="justify-end card-actions">
        <button
          class="btn btn-ghost"
          class:invisible={step === 0}
          on:click={() => step--}
        >
          上一步
        </button>
        <button
          class="btn btn-primary"
          disabled={results[step] === null}
          on:click={() => step++}
        >
          下一步
        </button>
      </div>
    </div>
  </div>
</main>
<footer class="fixed glass bottom-0 w-full py-2">
  <ul class="w-full steps">
    {#each steps as s, i}
      <li class="step" class:step-primary={step >= i}>{s.label}</li>
    {/each}
  </ul>
</footer>

<style>
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
