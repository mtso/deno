// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.
import {
  assertEquals,
  unitTest,
} from "./test_util.ts";

unitTest(function greetName(): void {
  const testName = "mtso";
  const greeting = new Deno.hello(testName);
  assertEquals(greeting, "Hello, " + testName);
});
