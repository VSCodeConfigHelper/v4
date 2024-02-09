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
  import { getVersion } from "@tauri-apps/api/app";
  import { open } from "@tauri-apps/api/shell";
  import { listen } from "@tauri-apps/api/event";
  import { confirm } from '@tauri-apps/api/dialog';
  import Icon from "@iconify/svelte";
  import compareVersions from "compare-versions";

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

  function goBack() {
    if (step === LEN - 1) {
      window.location.reload();
    } else {
      go(-1);
    }
  }

  function showDonateModal() {
    const donateModal = document.getElementById(
      "donate-modal"
    ) as HTMLInputElement;
    donateModal.checked = true;
  }

  let version = "0.0.0";
  let checkUpdateString = "检查更新";
  async function checkUpdate() {
    checkUpdateString = "检查更新中...";
    const result = await fetch(`https://api.guyutongxue.site/vscch/installer`).then((r) =>
      r.json()
    );
    const latestVersion: string = result.name;
    if (compareVersions(latestVersion, version) > 0) {
      if (await confirm(`新版本 ${latestVersion} 可用。是否前往下载？`)) {
        open(`https://vscch.guyutongxue.site`);
      }
    } else {
      alert(`已是最新版本（${latestVersion}）。`);
    }
    checkUpdateString = "检查更新";
  }

  let errorId: number | null = null;
  listen("log_sent", (r) => {
    errorId = r.payload as number;
  });


  onMount(async () => {
    for (const i in STEPS) {
      const subscribe = STEPS[i].resultWritable?.subscribe((v) => {
        results[i] = v;
      });
      if (subscribe) {
        subscribers.push(subscribe);
      }
    }
    version = await getVersion();
  });

  onDestroy(() => {
    for (const i in subscribers) {
      subscribers[i]();
    }
  });
</script>

<div class="fixed top-4 left-4 z-10">
  版本 {version}
  <button class="btn btn-link btn-sm" on:click={checkUpdate}>
    {checkUpdateString}
  </button>
</div>
<div class="fixed top-4 right-4 z-10 flex flex-row gap-4">
  <button
    class="btn btn-sm glass tooltip tooltip-bottom"
    on:click={showDonateModal}
    data-tip="显示捐助"
  >
    <Icon icon="mdi:dollar" />
  </button>
  <button
    class="btn btn-sm glass tooltip tooltip-bottom"
    on:click={changeBackground}
    data-tip="更换背景"
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
          class:invisible={step === 0}
          on:click={goBack}
        >
          {step === LEN - 1 ? "重新开始" : "上一步"}
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

<input type="checkbox" id="donate-modal" class="modal-toggle" />
<label for="donate-modal" class="modal">
  <label class="modal-box relative" for="">
    <label
      for="donate-modal"
      class="btn btn-ghost btn-sm btn-circle absolute right-2 top-2"
    >
      ✕
    </label>
    <h3 class="text-lg text-center">
      如果这个软件对你有帮助，可以给作者买一杯咖啡
    </h3>
    <div class="py-4 flex flex-row justify-around">
      <img
        class="h-32"
        src="https://s2.loli.net/2022/07/23/V4sKCIwMaBir18e.jpg"
        alt="alipay"
      />
      <img
        class="h-32"
        src="https://s2.loli.net/2022/07/23/WCTq3BgJ81P2xIz.jpg"
        alt="wechat"
      />
    </div>
  </label>
</label>
<input type="checkbox" id="log-modal" class="modal-toggle" checked={errorId !== null} />
<div class="modal">
  <div class="modal-box">
    <h3 class="font-bold text-lg">错误日志已发送</h3>
    <p class="py-4">
      您可以将标识码 <code>{errorId}</code>
      发送至
      <a href="mailto:guyutongxue@163.com">guyutongxue@163.com</a
      >，开发者会尽快帮您解决问题。
    </p>
    <div class="modal-action">
      <button class="btn" on:click={() => errorId = null}>知道了</button>
    </div>
  </div>
</div>

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
