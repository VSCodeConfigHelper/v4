// Copyright (C) 2022 Guyutongxue
// 
// This file is part of vscch4.
// 
// vscch4 is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// vscch4 is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with vscch4.  If not, see <http://www.gnu.org/licenses/>.

import { writable } from 'svelte/store';

export type Compiler = {
  setup: string;
  path: string;
  version: string;
  package_string: string;
  version_text: string;
};
export type OptionsBase = {
  compatibleMode: boolean;
  activeLanguage: string;
  activeStandard: string | null;
  asciiCheck: boolean;
  removeExtensions: boolean;
  addToPath: boolean;
  openVscode: boolean;
  collectData: boolean;
  desktopShortcut: boolean;
};

export type OptionsProfile = OptionsBase & {
  useGnu: boolean;
  pedantic: boolean;
  activeWarning: string;
  activeOptLevel: string;
  werror: boolean;
  acpOutput: boolean;
  staticStd: boolean;
  customArgs: string[];
}

export type Options = OptionsBase & {
  args: string[];
}

export const DEFAULT_PROFILE: OptionsProfile = {
  compatibleMode: false,
  activeLanguage: "C++",
  activeStandard: null,
  asciiCheck: false,
  removeExtensions: false,
  addToPath: true,
  openVscode: true,
  collectData: true,
  desktopShortcut: false,
  useGnu: false,
  pedantic: false,
  activeWarning: "default",
  activeOptLevel: "default",
  werror: false,
  acpOutput: false,
  staticStd: false,
  customArgs: []
};

export const NEWBIE_PROFILE: OptionsProfile = {
  compatibleMode: false,
  activeLanguage: "C++",
  activeStandard: null,
  asciiCheck: true,
  removeExtensions: true,
  addToPath: true,
  openVscode: true,
  collectData: true,
  desktopShortcut: true,
  useGnu: false,
  pedantic: true,
  activeWarning: "extra",
  activeOptLevel: "default",
  werror: true,
  acpOutput: true,
  staticStd: false,
  customArgs: []
};

export const vscode = writable<string | null>(null);
export const compiler = writable<Compiler | null>(null);
export const workspace = writable<string | null>(null);
export const options = writable<Options | null>(null);
export const done = writable<boolean | null>(null);
