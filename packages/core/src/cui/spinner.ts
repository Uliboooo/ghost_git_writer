export function spinner<T>(promise: Promise<T>, text = "Processing") {
  const frames = ["-", "\\", "|", "/"];
  return withSpinner(promise, text);

  async function withSpinner<T>(
    promise: Promise<T>,
    text: string
  ): Promise<T> {
    let i = 0;

    process.stdout.write("\x1b[?25l"); // hide cursor

    const interval = setInterval(() => {
      const frame = frames[i = (i + 1) % frames.length];
      process.stdout.write(`\r${frame} ${text}`);
    }, 100);

    try {
      try {
        const result = await promise;
        clearInterval(interval);
        process.stdout.write(`\r✔ Done\n`);
        return result;
      } catch (err) {
        clearInterval(interval);
        process.stdout.write(`\r✖ Error\n`);
        throw err;
      }
    } finally {
      process.stdout.write("\x1b[?25h"); // show cursor
    }
  }

}

