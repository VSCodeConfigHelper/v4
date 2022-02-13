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
  import { fade } from "svelte/transition";
  import Icon from "@iconify/svelte";

  // TITLE

  let tabTitles = ["通用", "语言", "编译", "功能", "行为"];
  let activeTab = 2;

  // PAGE 0 通用
  let compatibleMode = false;

  // PAGE 1 语言
  const languages = ["C++", "C"];
  let activeLanguage = "C++";
  const standards: Record<string, string[]> = {
    "C++": ["C++23", "C++20", "C++17", "C++14", "C++11", "C++03", "C++98"],
    C: ["C23", "C17", "C11", "C99", "C89"],
  };
  let activeStandard: string | null = null;
  $: if (
    activeStandard &&
    !standards[activeLanguage].includes(activeStandard)
  ) {
    activeStandard = null;
  }
  let useGnuEnabled = true;
  let useGnu = false;
  let pedantic = true;

  // PAGE 2 编译
  const warnings = [
    { name: "默认", value: "default" },
    { name: "全部", value: "all" },
    { name: "更多", value: "extra" },
  ];
  let activeWarning = "default";
  const optLevels = [
    { name: "无", value: "default" },
    { name: "1", value: "1" },
    { name: "2", value: "2" },
    { name: "3", value: "3" },
    { name: "仅速度", value: "speed" },
    { name: "仅体积", value: "size" },
  ];
  let activeOptLevel = "default";
  let werror = false;
  let acpOutputEnabled = true;
  let acpOutput = false;
  let staticStd = false;
  let __2_input_focused = false;
  let __2_input_element: HTMLInputElement;
  function __2_handle_click(e: MouseEvent) {
    __2_input_element.focus();
    e.stopPropagation();
  }
  $: generatedArgs = ((): string[] => {
    return [];
  })();
  let customArgs: string[] = [];
  let __2_current = "";
  function __2_handle_keypress(e: KeyboardEvent) {
    if (e.key === "Enter" || e.key === " ") {
      if (__2_current && !customArgs.includes(__2_current)) {
        customArgs = [...customArgs, __2_current];
      }
      __2_current = "";
      e.preventDefault();
    }
  }
  function __2_handle_keydown(e: KeyboardEvent) {
    if (e.key === "Backspace") {
      console.log(__2_current.length);
      if (__2_current.length === 0) {
        customArgs = customArgs.slice(0, -1);
      }
    }
  }
</script>

<div class="form-control space-y-3">
  <h3 class="text-3xl font-bold">配置选项</h3>
  <div>您可以在这里调整您的个性化设置。</div>
  <div class="w-full flex flex-col overflow-x-auto">
    <div class="mx-auto flex flex-row text-sm font-bold relative">
      {#each tabTitles as t, i}
        <div class="shrink-0 z-10 w-16" on:click={() => (activeTab = i)}>
          <div
            class="text-center text-black rounded-xl"
            class:non-active={i !== activeTab}
          >
            {t}
          </div>
        </div>
      {/each}
      <div
        class="absolute top-0 h-full w-16 transition-all ease-in-out"
        style:left={`${activeTab * 64}px`}
      >
        <div class="z-0 full rounded-xl bg-white" />
        <div class="z-20 full rounded-xl bg-white mix-blend-difference" />
        <div class="z-30 full rounded-xl bg-primary mix-blend-screen" />
      </div>
    </div>
  </div>
  {#if activeTab === 0}
    <div class="flex flex-row w-full justify-around">
      <div>
        <div class="text-center font-bold mb-3">导入已有配置</div>
        <div class="flex flex-col space-y-2">
          <button class="btn btn-sm glass text-black font-normal">
            默认配置
          </button>
          <button class="btn btn-sm glass text-black font-normal">
            新手配置
          </button>
          <button class="btn btn-sm glass text-black font-normal"
            >上次配置
          </button>
        </div>
      </div>
      <div class="divider divider-vertical mx-0" />
      <div class="flex flex-col items-center">
        <div>
          <span class="font-bold mb-3">使用兼容模式</span>
          <span class="dropdown dropdown-hover dropdown-end">
            <div tabindex="0">
              <Icon icon="mdi:help-circle-outline" />
            </div>
            <div
              tabindex="0"
              class="dropdown-content card compact bg-base-100 shadow-lg w-52"
            >
              <div class="card-body">
                <h2 class="card-title">什么是兼容模式</h2>
                <p>
                  本工具可以生成更美观、易用的配置，但这与网上的大多数教程并不兼容。
                  <br />
                  以兼容模式运行时，工具将使用更常见的配置方法。
                </p>
              </div>
            </div>
          </span>
        </div>
        <div class="flex-grow flex justify-center items-center">
          <input
            type="checkbox"
            class="toggle toggle-lg toggle-primary"
            bind:checked={compatibleMode}
          />
        </div>
      </div>
    </div>
  {:else if activeTab === 1}
    <div>
      <span>您将使用 </span>
      <select
        class="select select-sm select-bordered"
        bind:value={activeLanguage}
      >
        {#each languages as l}
          <option value={l}>{l}</option>
        {/each}
      </select>
      <span> 编写代码，采用 </span>
      <select
        class="select select-sm select-bordered"
        bind:value={activeStandard}
      >
        <option value={null}>最新可用</option>
        {#each standards[activeLanguage] as s}
          <option value={s}>{s}</option>
        {/each}
      </select>
      <span> 的语言标准。</span>
    </div>
    <div class="font-bold">更多选项</div>
    <div class="grid grid-cols-2 gap-2">
      <div class="flex flex-row items-center space-x-2">
        <input
          type="checkbox"
          disabled={!useGnuEnabled}
          class="toggle toggle-sm toggle-primary"
          bind:checked={useGnu}
        />
        <div>GNU 方言</div>
      </div>
      <div class="flex flex-row items-center space-x-2">
        <input
          type="checkbox"
          class="toggle toggle-sm toggle-primary"
          bind:checked={pedantic}
        />
        <div>严格执行标准</div>
      </div>
    </div>
  {:else if activeTab === 2}
    <div class="font-bold">常用编译参数</div>
    <div class="grid grid-cols-2 gap-x-6 gap-y-2">
      <div class="flex flex-row justify-between items-center space-x-2">
        <div>警告级别</div>
        <select
          class="select select-sm select-bordered"
          bind:value={activeWarning}
        >
          {#each warnings as w}
            <option value={w.value}>{w.name}</option>
          {/each}
        </select>
      </div>
      <div class="flex flex-row justify-between items-center space-x-2">
        <div>优化级别</div>
        <select
          class="select select-sm select-bordered"
          bind:value={activeOptLevel}
        >
          {#each optLevels as o}
            <option value={o.value}>{o.name}</option>
          {/each}
        </select>
      </div>
      <div class="flex flex-row justify-between items-center space-x-2">
        <div>视警告为错误</div>
        <input
          type="checkbox"
          class="toggle toggle-sm toggle-primary"
          bind:checked={werror}
        />
      </div>
      <div class="flex flex-row justify-between items-center space-x-2">
        <div>静态链接</div>
        <input
          type="checkbox"
          class="toggle toggle-sm toggle-primary"
          bind:checked={staticStd}
        />
      </div>
    </div>
    <div class="font-bold">自定义...</div>
    <div
      class="input input-sm input-bordered cursor-text"
      on:click={__2_handle_click}
      class:input-focus={__2_input_focused}
    >
      <span class="space-x-1 cursor-pointer">
        {#each generatedArgs as a (a)}
          <span class="badge badge-primary" transition:fade={{ duration: 200 }}>
            {a}
          </span>
        {/each}
        {#each customArgs as a, i (a)}
          <span
            class="badge badge-info"
            transition:fade={{ duration: 200 }}
            on:click={() =>
              (customArgs = [
                ...customArgs.slice(0, i),
                ...customArgs.slice(i + 1),
              ])}
          >
            {a}
          </span>
        {/each}
      </span>
      <input
        type="text"
        bind:this={__2_input_element}
        on:focus={() => (__2_input_focused = true)}
        on:blur={() => (__2_input_focused = false)}
        class="outline-none bg-transparent"
        bind:value={__2_current}
        on:keypress={__2_handle_keypress}
        on:keydown={__2_handle_keydown}
      />
    </div>
  {:else if activeTab === 3}
    <div />
  {:else if activeTab === 4}
    <div />
  {/if}
</div>

<style>
  .full {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
  }

  .non-active {
    cursor: pointer;
  }
  .non-active:hover {
    color: hsla(var(--p));
  }

  .input-focus {
    outline: 2px solid transparent;
    outline-offset: 2px;
    box-shadow: 0 0 0 2px hsl(var(--b1)), 0 0 0 4px hsla(var(--bc) / 0.2);
  }
</style>
