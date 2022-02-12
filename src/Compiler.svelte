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
  import { onMount } from "svelte";
  import Icon from "@iconify/svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/dialog";
  import { compiler, type Compiler } from "./config_store";
  import type { VerifyResult } from "./global";

  type CompilerSetup = {
    id: string;
    name: string;
    description: string;
    how_to_install: string;
    can_verify: boolean;
    can_install: boolean;
  };

  let setups: CompilerSetup[] = [];
  let setupNo = 0;
  $: setup = setups[setupNo];
  let hiddenFocus: HTMLElement;

  let compilers: Compiler[] = [];
  let compilerNo = 0;

  let useNew = false;
  let newPath = "";
  let verifyResult: VerifyResult<Compiler> | null = null;

  $: compiler.update(() =>
    useNew
      ? verifyResult?.type === "Ok"
        ? verifyResult.value
        : null
      : compilers[compilerNo]
  );

  async function changeSetup(i: number) {
    setupNo = i;
    await scan();
    setTimeout(() => hiddenFocus.focus(), 100);
  }

  async function browse() {
    const result = await open({
      multiple: false,
      directory: true,
    });
    if (typeof result === "string") {
      newPath = result;
    }
    verify();
  }

  async function install() {
    await invoke("compiler_install", { setupNo });
  }

  async function scan() {
    compilers = await invoke("compiler_scan", { setupNo });
    useNew = compilers.length === 0;
  }

  async function verify() {
    verifyResult = await invoke<VerifyResult<Compiler>>("compiler_verify", {
      setupNo,
      path: newPath,
    });
  }

  onMount(async () => {
    setups = await invoke("compiler_setup_list");
    await scan();
  });
</script>

<button class="w-0 h-0" bind:this={hiddenFocus} />
<div class="form-control space-y-3">
  <div class="flex flex-row justify-between items-center">
    <h3 class="text-3xl font-bold">选择编译器</h3>
    <div class="dropdown dropdown-end">
      <div tabindex="0" class="btn btn-xs btn-link font-normal">
        更改编译器类型...
      </div>
      <ul
        tabindex="0"
        class="dropdown-content shadow-lg menu menu-compact bg-base-100 rounded-box w-60"
      >
        {#each setups as s, i}
          <li>
            <a href={"#"} class="!px-2" on:click={() => changeSetup(i)}>
              <div class:invisible={i !== setupNo}>
                <Icon icon="mdi:check" />
              </div>
              <div class="pl-2 flex flex-col items-start">
                {s.name}
                <small>{s.description}</small>
              </div>
            </a>
          </li>
        {/each}
      </ul>
    </div>
  </div>
  {#if setups.length > 0}
    {#if !useNew}
      <div>
        检测到下列 {setup.name}，请选择其中一个来编译您的代码。
      </div>
      <div class="overflow-x-auto rounded-lg bg-base-100">
        <table class="table table-compact min-w-full">
          <thead>
            <tr>
              <th />
              <th>路径</th>
              <th>版本</th>
              <th>打包信息</th>
            </tr>
          </thead>
          <tbody>
            {#each compilers as c, i}
              <tr
                class="hover"
                on:click|stopPropagation={() => (compilerNo = i)}
              >
                <th>
                  <input
                    type="radio"
                    class="checkbox checkbox-sm translate-y-0.5 cursor-default"
                    checked={compilerNo === i}
                    name="compilerNo"
                  />
                </th>
                <td>{c.path}</td>
                <td>{c.version}</td>
                <td>{c.package_string}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
      <div>
        <button
          class="btn btn-sm btn-link font-normal"
          on:click={() => ((useNew = true), (verifyResult = null))}
        >
          或者，使用新的编译器...
        </button>
      </div>
    {:else}
      <div class="flex flex-row items-center space-x-4">
        <div class="flex-grow">
          {#if compilers.length === 0}
            未检测到已安装的 {setup.name}。
          {/if}
          {#if setup.can_install}
            您可以点击右侧按钮{/if}{@html setup.how_to_install}
        </div>
        {#if setup.can_install}
          <button
            class="btn btn-info btn-lg btn-circle shadow-lg"
            on:click={install}
          >
            <Icon icon="mdi:download" width={35} />
          </button>
        {/if}
      </div>
      {#if setup.can_verify}
        <div class="flex space-x-2">
          <input
            type="text"
            class="flex-grow input input-bordered"
            bind:value={newPath}
            on:input={verify}
          />
          <button class="btn btn-ghost btn-circle" on:click={browse}>
            <Icon icon="mdi:folder-open" width={20} />
          </button>
        </div>
        {#if verifyResult !== null}
          <div
            class="alert flex-row justify-start items-center p-2"
            class:alert-success={verifyResult.type === "Ok"}
            class:alert-error={verifyResult.type !== "Ok"}
          >
            <Icon
              class="shrink-0"
              icon={verifyResult.type === "Ok"
                ? "mdi:check-circle"
                : "mdi:alert-circle"}
              width={20}
            />
            <span class="!mt-0 ml-2 inline">
              {#if verifyResult.type === "Ok"}
                检测到 {setup.name}，版本
                <code>{verifyResult.value.version}</code>
                ，打包信息
                <code>{verifyResult.value.package_string}</code>
              {:else}
                该路径下没有 {setup.name}（{verifyResult.message}）
              {/if}
            </span>
          </div>
        {/if}
      {:else}
        <div>
          <button class="btn btn-sm btn-link font-normal" on:click={scan}>
            重新检测
          </button>
        </div>
      {/if}
      {#if compilers.length > 0}
        <div>
          <button class="btn btn-sm btn-link font-normal" on:click={scan}>
            或者，使用已有的编译器...
          </button>
        </div>
      {/if}
    {/if}
  {:else}
    <div class="pb-3">目前暂不支持此操作系统的配置。</div>
  {/if}
</div>
