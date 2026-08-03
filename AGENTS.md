# AERIS

**AERIS**(에리스)는 프로그래밍 언어와 그 관련 인프라를 개발하는 프로젝트다.
정식 언어 명칭은 **AERIS Standard**지만, 대부분의 문맥에서는 AERIS라고 부른다.
문맥에 따라 AERIS는 언어 자체 또는 언어 구현에 필요한 인프라 전체를 가리킬 수 있다.
다중 패러다임 범용 언어이며, 다른 주요 프로그래밍 언어를 대체할 수 있는 수준을 장기 목표로 한다.
핵심 철학은 **"절대 외부 입력, 개발자를 신뢰하지 않는다"**이다.
컴파일은 증명된 사실 또는 공리에만 기반해야 한다.
가능한 것은 최대한 컴파일타임에 처리하고 최적화하며, 런타임 처리는 차선책으로 둔다.
성능과 확장성도 핵심 가치다.
기본 배포 형태는 바이너리 컴파일이다.
부트스트랩 이전에는 Linux x86_64만 지원하고, 이후 여러 환경을 지원한다.
JIT과 REPL 지원은 후속 단계에서 추가한다.
구현은 LLVM 기반이며 Rust의 Inkwell을 사용한다.

## 모듈

- [AERIS CLI - `./apps/cli`]()
- [AERIS Daemon - `./apps/daemon`]()
- [AERIS Core - `./crates/core`](./agents/modules/core.md)
- [AERIS Core / UI - `./crates/core/ui`](./agents/modules/core/ui.md)
- [AERIS Standard - `./crates/std`]()
- [AERIS Standard / UI - `./crates/std/ui`]()
- [Example / JSON - `./examples/json`]()
