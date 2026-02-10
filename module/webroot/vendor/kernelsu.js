// 兼容包装层：根据实际 KernelSU WebUI 注入对象名称做适配。
// 你可以在这里统一改映射，而不必动业务代码。

function getExec() {
  // 常见注入名（按需要增减）
  if (globalThis.kernelsu && typeof globalThis.kernelsu.exec === "function")
    return globalThis.kernelsu.exec.bind(globalThis.kernelsu);
  if (globalThis.ksu && typeof globalThis.ksu.exec === "function")
    return globalThis.ksu.exec.bind(globalThis.ksu);
  if (globalThis.KernelSU && typeof globalThis.KernelSU.exec === "function")
    return globalThis.KernelSU.exec.bind(globalThis.KernelSU);
  return null;
}

export async function exec(cmd) {
  const fn = getExec();
  if (!fn) {
    throw new Error(
      "KernelSU exec bridge not found (kernelsu/ksu/KernelSU). Please adjust webroot/vendor/kernelsu.js",
    );
  }
  // 期望返回 { errno, stdout, stderr } 或类似
  return await fn(cmd);
}
