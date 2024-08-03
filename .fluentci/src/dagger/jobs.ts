/**
 * @module rust
 * @description This module provides a set of functions to build, test, and run clippy on a Rust project 🦀
 */

import { dag, env, Directory, DirectoryID, File } from "../../deps.ts";

export enum Job {
  clippy = "clippy",
  test = "test",
  build = "build",
  llvmCov = "llvm_cov",
}

export const exclude = ["target", ".git", ".devbox", ".fluentci"];

export const getDirectory = async (
  src: string | Directory | undefined = ".",
) => {
  if (src instanceof Directory) {
    return src;
  }
  if (typeof src === "string") {
    try {
      const directory = dag.loadDirectoryFromID(src as DirectoryID);
      await directory.id();
      return directory;
    } catch (_) {
      return dag.host
        ? dag.host().directory(src)
        : dag.currentModule().source().directory(src);
    }
  }
  return dag.host
    ? dag.host().directory(src)
    : dag.currentModule().source().directory(src);
};

/**
 * Run clippy
 *
 * @function
 * @description Run clippy
 * @param {string | Directory | undefined} src
 * @returns {string}
 */
export async function clippy(
  src: string | Directory | undefined = ".",
): Promise<File | string> {
  const context = await getDirectory(src);
  const ctr = dag
    .pipeline(Job.clippy)
    .container()
    .from("rust:1.73-bookworm")
    .withExec(["apt-get", "update"])
    .withExec(["apt-get", "install", "-y", "build-essential", "pkg-config"])
    .withExec(["rustup", "component", "add", "clippy"])
    .withExec(["cargo", "install", "clippy-sarif", "--version", "0.3.0"])
    .withExec(["cargo", "install", "sarif-fmt", "--version", "0.3.0"])
    .withDirectory("/app", context, { exclude })
    .withWorkdir("/app")
    .withMountedCache("/app/target", dag.cacheVolume("target"))
    .withMountedCache("/root/cargo/registry", dag.cacheVolume("registry"))
    .withExec([
      "sh",
      "-c",
      "cargo clippy \
        --all-features \
        --message-format=json | clippy-sarif | tee rust-clippy-results.sarif | sarif-fmt",
    ])
    .withExec(["ls", "-la", "/app"]);

  const results = await ctr.file("/app/rust-clippy-results.sarif");
  results.export("./rust-clippy-results.sarif");
  await ctr.stdout();
  return results.id();
}

/**
 * Generate llvm coverage report
 *
 * @function
 * @description Generate llvm coverage report
 * @param {string | Directory | undefined} src
 * @returns {string}
 */
export async function llvmCov(
  src: string | Directory | undefined = ".",
): Promise<File | string> {
  const context = await getDirectory(src);
  const ctr = dag
    .pipeline(Job.llvmCov)
    .container()
    .from("rust:1.73-bookworm")
    .withExec(["apt-get", "update"])
    .withExec([
      "apt-get",
      "install",
      "-y",
      "build-essential",
      "wget",
      "pkg-config",
    ])
    .withExec(["rustup", "component", "add", "llvm-tools"])
    .withExec([
      "wget",
      "https://github.com/taiki-e/cargo-llvm-cov/releases/download/v0.5.36/cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz",
    ])
    .withExec(["tar", "xvf", "cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz"])
    .withExec(["mv", "cargo-llvm-cov", "/usr/local/bin"])
    .withDirectory("/app", context, { exclude })
    .withWorkdir("/app")
    .withMountedCache("/app/target", dag.cacheVolume("target"))
    .withMountedCache("/root/cargo/registry", dag.cacheVolume("registry"))
    .withExec([
      "sh",
      "-c",
      "cargo llvm-cov \
        --all-features \
        --lib \
        --workspace \
        --lcov \
        --output-path \
          lcov.info",
    ])
    .withExec(["ls", "-la", "/app"]);

  const lcov = ctr.file("/app/lcov.info");
  await lcov.export("./lcov.info");
  await ctr.stdout();
  return lcov.id();
}

/**
 * Run tests
 *
 * @function
 * @description Run tests
 * @param {string | Directory | undefined} src
 * @param {string[]} options
 * @returns {string}
 */
export async function test(
  src: string | Directory | undefined = ".",
  options: string[] = [],
): Promise<string> {
  const context = await getDirectory(src);
  const ctr = dag
    .pipeline(Job.test)
    .container()
    .from("rust:latest")
    .withExec(["apt-get", "update"])
    .withExec(["apt-get", "install", "-y", "libssl-dev"])
    .withDirectory("/app", context, { exclude })
    .withWorkdir("/app")
    .withMountedCache("/app/target", dag.cacheVolume("target"))
    .withMountedCache("/root/cargo/registry", dag.cacheVolume("registry"))
    .withExec(["cargo", "test", ...options]);

  return ctr.stdout();
}

/**
 * Build the project
 *
 * @function
 * @description Build the project
 * @param {string | Directory | undefined} src
 * @param {string} packageName
 * @param {string} target
 * @param {string[]} options
 * @returns {string}
 */
export async function build(
  src: string | Directory | undefined = ".",
  packageName?: string,
  target = "x86_64-unknown-linux-gnu",
  options: string[] = [],
): Promise<Directory | string> {
  const context = await getDirectory(src);
  const ctr = dag
    .pipeline(Job.build)
    .container()
    .from("rust:1.77.2")
    .withExec(["apt-get", "update"])
    .withExec(["apt-get", "install", "-y", "libssl-dev"])
    .withExec([
      "wget",
      "https://github.com/mozilla/sccache/releases/download/v0.7.7/sccache-v0.7.7-x86_64-unknown-linux-musl.tar.gz",
    ])
    .withExec([
      "tar",
      "-xvf",
      "sccache-v0.7.7-x86_64-unknown-linux-musl.tar.gz",
    ])
    .withExec([
      "mv",
      "sccache-v0.7.7-x86_64-unknown-linux-musl/sccache",
      "/usr/local/bin",
    ])
    .withEnvVariable("RUSTC_WRAPPER", "/usr/local/bin/sccache")
    .withDirectory("/app", context, { exclude })
    .withWorkdir("/app")
    .withMountedCache("/root/.cache/sccache", dag.cacheVolume("sccache"))
    .withMountedCache("/app/target", dag.cacheVolume("target"))
    .withMountedCache("/root/cargo/registry", dag.cacheVolume("registry"))
    .withExec(
      env.has("PACKAGE_NAME") || packageName
        ? [
            "cargo",
            "build",
            "--release",
            "-p",
            env.get("PACKAGE_NAME") || packageName!,
            "--target",
            target,
            ...options,
          ]
        : ["cargo", "build", "--release", "--target", target, ...options],
    )
    .withExec(["cp", "-r", `/app/target/${target}`, "/"]);

  const result = await ctr.stdout();

  console.log(result);
  await ctr.directory(`/${target}`).export("./target");
  return ctr.directory(`/${target}`).id();
}

export type JobExec =
  | ((src?: string | Directory | undefined) => Promise<Directory | string>)
  | ((src?: string | Directory | undefined) => Promise<File | string>)
  | ((src?: string | Directory | undefined) => Promise<string>);

export const runnableJobs: Record<Job, JobExec> = {
  [Job.clippy]: clippy,
  [Job.test]: test,
  [Job.build]: build,
  [Job.llvmCov]: llvmCov,
};

export const jobDescriptions: Record<Job, string> = {
  [Job.clippy]: "Run clippy",
  [Job.test]: "Run tests",
  [Job.build]: "Build the project",
  [Job.llvmCov]: "Generate llvm coverage report",
};
