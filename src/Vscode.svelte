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

  import { vscode } from "./config_store";
  import type { VerifyResult } from "./global";

  let path: string = "";

  let verifyResult: VerifyResult = { type: "Ok" };
  let scanResult = false;
  let state: "scanned" | "verified" | "working" = "working";

  $: vscode.update(() =>
    (state === "scanned" && scanResult) ||
    (state === "verified" && verifyResult.type === "Ok")
      ? path
      : null
  );

  async function browse() {
    const result = await open({
      multiple: false,
    });
    if (typeof result === "string") {
      path = result;
    }
    verify();
  }

  async function scan() {
    state = "working";
    const result = await invoke<string | null>("vscode_scan");
    if (result !== null) {
      path = result;
      scanResult = true;
    } else {
      scanResult = false;
    }
    state = "scanned";
  }

  async function verify() {
    state = "working";
    const result = await invoke<VerifyResult>("vscode_verify", {
      path: path,
    });
    verifyResult = result;
    state = "verified";
  }
  onMount(() => {
    scan();
  });
</script>

<div class="form-control space-y-3">
  <div class="flex flex-row justify-between items-center">
    <h3 class="text-3xl font-bold">确认 VS Code</h3>
    <button class="btn btn-xs btn-link font-normal" on:click={scan}>
      重新检测
    </button>
  </div>
  <div>首先，请确认您的 VS Code 安装情况。</div>
  <div
    class="alert flex-row justify-start items-center p-2"
    class:alert-success={state !== "working" && $vscode !== null}
    class:alert-error={state !== "working" && $vscode === null}
    class:alert-info={state === "working"}
  >
    {#if state === "working"}
      <span class="btn btn-ghost btn-sm btn-circle loading" />
    {:else}
      <Icon
        class="shrink-0"
        icon={$vscode !== null ? "mdi:check-circle" : "mdi:alert-circle"}
        width={20}
      />
    {/if}
    <span class="!mt-0 ml-2 inline">
      {#if state === "scanned"}
        {#if scanResult}
          检测到您安装的 VS Code 路径如下。如没有任何问题，请点击下一步。
        {:else}
          <span>
            没有检测到已安装的 VS Code，请您前往
            <a
              href="https://code.visualstudio.com"
              target="_blank"
              class="text-primary"
            >
              此处
            </a>
            下载安装。如果您安装了，请在下方提供其安装路径。
          </span>
        {/if}
      {:else if state === "verified"}
        {#if verifyResult.type === "Ok"}
          此路径下存在 VS Code。点击下一步以继续。
        {:else}
          此路径下没有检测到 VS Code：{verifyResult.message}。
        {/if}
      {:else}
        正在检测中...
      {/if}
    </span>
  </div>
  <div class="flex space-x-2">
    <input
      type="text"
      placeholder="VS Code 安装路径"
      class="flex-grow input input-bordered"
      bind:value={path}
      on:input={verify}
    />
    <button class="btn btn-ghost btn-circle" on:click={browse}>
      <Icon icon="mdi:folder-open" width={20} />
    </button>
  </div>
</div>
