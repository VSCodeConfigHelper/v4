/// <reference types="svelte" />
/// <reference types="vite/client" />

export type VerifyResult<T = undefined> =
  (T extends undefined
    ? {
      type: "Ok"
    }
    : {
      type: "Ok";
      value: T;
    })
  | {
    type: "Err";
    message: string;
  };
