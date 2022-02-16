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
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  import { vscode, compiler, workspace, options, done } from "./config_store";

  type TaskResult = {
    type: "Ok",
    name: string;
  } | {
    type: "Error";
    name: string;
    message: string;
  };

  let working = true;
  let success = false;

  $: done.update(() => !working);

  listen("task_finish", (r) => {
    const p = r.payload as TaskResult;
    if (p.type === "Ok") {

    } else {
      alert(p.message);
      working = false;
    }
  });

  onMount(async () => {
    const taskNum= await invoke("task_init", {
      args: {
        vscode: $vscode,
        compiler: $compiler,
        workspace: $workspace,
        options: $options,
      }
    });
  });
</script>

<div class="form-control space-y-3">
  <h3 class="text-3xl font-bold">
    {#if working}
      <div class="flex flex-row items-center">
        <span class="btn btn-ghost btn-circle loading" />
        <span>正在配置</span>
      </div>
    {:else}
    {#if success}
      配置完成！
    {:else}
      配置失败。
    {/if}
    {/if}
  </h3>
</div>
