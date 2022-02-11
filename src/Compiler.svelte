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

  type CompilerSetup = {
    name: string;
    description: string;
  };
  let setups: CompilerSetup[] = [];
  let setupNo = 0;
  let hiddenFocus: HTMLElement;

  async function scan() {}

  function changeSetup(i: number) {
    setupNo = i;
    setTimeout(() => hiddenFocus.focus(), 100);
  }

  onMount(async () => {
    setups = await invoke("compiler_setup_list");
  });
</script>

<div class="form-control">
  <h3 class="text-3xl font-bold pb-3">选择编译器</h3>
  <div class="pb-3">
    现在，您需要选择一个编译器来编译您的代码。
    <button class="w-0 h-0" bind:this={hiddenFocus} />
    <div class="dropdown">
      <div tabindex="0" class="btn btn-xs btn-link font-normal" on:click={scan}>
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
</div>
