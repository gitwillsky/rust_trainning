"use strict";

({ print, fetch }) => {
  globalThis.console = {
    log: (args) => {
      print(args);
    },
  };
  globalThis.fetch = fetch;
};
