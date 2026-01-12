cargo +stage1 clean
try {
Remove-Item .\mir_dump -Recurse -Force -ErrorAction Stop
} catch {}
pwsh -Command { $env:RUSTC_LOG="rustc_mir_transform::coroutine=trace"; $env:RUSTFLAGS="-Z dump-mir=coroutine_before|coroutine_resume|coroutine_custom"; cargo +stage1 build --release 2> mir_coroutine_witnesses.log }
