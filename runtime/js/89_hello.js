"use strict";

((window) => {
  function greet(name) {
    const { message } = Deno.core.opSync("op_hello_greet", { name });
    return message;
  }

  window.__bootstrap.hello = {
    greet,
  };
})(this);
