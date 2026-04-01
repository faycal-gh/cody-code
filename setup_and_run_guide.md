# Setup and Run Guide: Claw Code

This guide covers the public Rust workspace only.

## Prerequisites

- Rust stable toolchain
- Cargo
- On Windows, Visual C++ Build Tools so `link.exe` is available

Install Rust:

```powershell
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
.\rustup-init.exe -y --default-toolchain stable
```

Install Visual C++ Build Tools if needed:

```powershell
Invoke-WebRequest -Uri "https://aka.ms/vs/17/release/vs_buildtools.exe" -OutFile "vs_buildtools.exe"
.\vs_buildtools.exe --quiet --wait --norestart --nocache --installPath C:\BuildTools --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended
```

## Build and Run

```powershell
cd C:\Users\msii\Desktop\claw-code\rust
cargo build --release
```

Run the CLI:

```powershell
.\target\release\claw.exe
```

You can also run it without building a release binary first:

```powershell
cd C:\Users\msii\Desktop\claw-code\rust
cargo run -p claw-cli --bin claw
```

For faster local iteration while developing, the debug binary is also fine:

```powershell
cd C:\Users\msii\Desktop\claw-code\rust
cargo build
.\target\debug\claw.exe
```

## Command Not Found Fix

If `claw` is not recognized, that usually means the binary is not on your `PATH` yet. You can still run it directly:

```cmd
target\debug\claw.exe
```

or:

```cmd
target\release\claw.exe
```

To make `claw` available in the current `cmd.exe` session:

```cmd
set PATH=%PATH%;C:\Users\msii\Desktop\claw-code\rust\target\debug
claw
```

To make it available in the current PowerShell session:

```powershell
$env:Path += ";C:\Users\msii\Desktop\claw-code\rust\target\debug"
claw
```

If you prefer the release build, replace `debug` with `release` in the commands above.

## Verification

Useful verification commands:

```powershell
cd C:\Users\msii\Desktop\claw-code\rust
cargo fmt --all --check
cargo test --workspace
```

Examples:

```powershell
$env:GROQ_API_KEY="gsk-..."
.\target\release\claw.exe --model openai/gpt-oss-120b --base-url https://api.groq.com/openai/v1

$env:GEMINI_API_KEY="AIza..."
.\target\release\claw.exe --model gemini-3-flash-preview --base-url https://generativelanguage.googleapis.com/v1beta/openai

$env:HF_TOKEN="hf_..."
.\target\release\claw.exe --model Qwen/Qwen2.5-Coder-32B-Instruct --base-url https://router.huggingface.co/v1
```

## Attribution

This repository started from a clone of:

`https://github.com/instructkr/claw-code`

and was then modified and cleaned up independently before public publishing.
