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
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";
  import Icon from "@iconify/svelte";

  import type { VerifyResult } from "./global";
  import { workspace } from "./config_store";

  let path = "";
  let verifyResult: VerifyResult = { type: "Ok" };

  $: workspace.update(() =>
    verifyResult.type !== "Err" && path !== "" ? path : null
  );

  async function browse() {
    const result = await open({
      multiple: false,
      directory: true,
    });
    if (typeof result === "string") {
      path = result;
    }
    verify();
  }

  async function verify() {
    verifyResult = await invoke("workspace_verify", { path });
  }
</script>

<div class="form-control space-y-3">
  <h3 class="text-3xl font-bold">选择工作文件夹</h3>
  <div>
    VS Code
    的配置大多在特定文件夹下生效，方便您为不同的语言和不同的需求个性化配置。因此，请选择一个
    <em>工作文件夹</em> ，您将来的程序和代码<strong>都需要</strong>存放在此处。
  </div>
  <div class="flex space-x-2">
    <input
      type="text"
      placeholder="工作文件夹路径"
      class="flex-grow input input-bordered"
      bind:value={path}
      on:input={verify}
    />
    <button class="btn btn-ghost btn-circle" on:click={browse}>
      <Icon icon="mdi:folder-open" width={20} />
    </button>
  </div>
  {#if verifyResult.type !== "Ok"}
    <div
      class="alert flex-row justify-start items-center p-2"
      class:alert-warning={verifyResult.type === "Warn"}
      class:alert-error={verifyResult.type === "Err"}
    >
      <Icon class="shrink-0" icon="mdi:alert-circle" width={20} />
      <span class="!mt-0 ml-2 inline">
        {verifyResult.message}
      </span>
    </div>
  {/if}
</div>
