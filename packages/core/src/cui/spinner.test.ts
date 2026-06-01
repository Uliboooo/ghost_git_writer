import { expect, test, describe, spyOn, beforeEach, afterEach } from "bun:test";
import { spinner } from "./spinner";

describe("spinner", () => {
  let stdoutWriteSpy: any;

  beforeEach(() => {
    stdoutWriteSpy = spyOn(process.stdout, "write").mockImplementation(() => true);
  });

  afterEach(() => {
    stdoutWriteSpy.mockRestore();
  });

  test("should show spinner and then 'Done' on success", async () => {
    const promise = new Promise((resolve) => setTimeout(() => resolve("success"), 150));
    const result = await spinner(promise, "Testing");

    expect(result).toBe("success");
    expect(stdoutWriteSpy).toHaveBeenCalledWith(expect.stringContaining("Testing"));
    expect(stdoutWriteSpy).toHaveBeenCalledWith(expect.stringContaining("✔ Done"));
  });

  test("should show spinner and then 'Error' on failure", async () => {
    const promise = new Promise((_, reject) => setTimeout(() => reject(new Error("fail")), 150));
    
    try {
      await spinner(promise, "Testing");
    } catch (e) {
      expect((e as Error).message).toBe("fail");
    }

    expect(stdoutWriteSpy).toHaveBeenCalledWith(expect.stringContaining("Testing"));
    expect(stdoutWriteSpy).toHaveBeenCalledWith(expect.stringContaining("✖ Error"));
  });
});
