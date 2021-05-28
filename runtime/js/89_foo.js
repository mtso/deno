"use strict";
((window) => {

  // const { InnerBody } = window.__bootstrap.fetchBody;
  // const { Response, fromInnerRequest, toInnerResponse, newInnerRequest } =
  //   window.__bootstrap.fetch;
  // const errors = window.__bootstrap.errors.errors;
  // const core = window.Deno.core;
  // const { ReadableStream } = window.__bootstrap.streams;

  function hello() {
    // const n: number = 1;
    const res = Deno.core.opSync("op_foo_hello", "hello");
    return res;
  }

  window.__bootstrap.foo = {
    hello,
  };
})(this);
