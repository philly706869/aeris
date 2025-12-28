# AERIS Programming Language

AERIS에 포함된 멀티 패러다임 범용 프로그래밍 언어.
AERIS는 `에리스`라고 발음.
Aeris가 아닌 AERIS가 정식 이름.
AERIS Programming Language 또는 AERIS는 일반적으로 AERIS Standard Language를 지칭함.

## Progress

알바 버전 (구현 거의 없음).
현재 AERIS UI 모듈 개발중.

## Details

### 메모리 모델

Rust와 비슷함.
GC 미사용.

### 동시성 모델

아직 확실한 계획은 없지만 Rust와 비슷할것으로 예상.

### 타입 시스템

매우 강력한 타입 시스템.
`premise` 기능으로 값에 대한 전제 보장을 할 수 있음.

### 핵심 기능 및 문법 초안

대부분의 언어에 있는 기본적인 기능은 생략

#### Premise

premise 선언으로 전제 정의 가능.

```aeris
fn print_num(n: U32) {
  premise 0 < n and n < 100;
  print(n);
}

fn main {
  print_num(50) // ok
  print_num(150) // *compile* error
}
```

premise 전파, premise 런타임 검사 등 추가적인 기능 지원.

#### Syntax Define

UI 플러그인으로 문법 확장 가능.
UI Server에서 Syntax / Sementic Highlighting 가능.

#### Context Parameter

Kotlin의 Context Parameter 기능과 유사한 기능.
상세 스펙은 미정.

## Schedules

추후 개발 예정인 프로젝트  
\*는 거의 확정, 그 외는 취소될 수 있음

- \*AERIS Web Framework
- \*AERIS REPL
- AERIS Shell
- AERIS Linker

## Modules

### Overview

- AERIS
  - [AERIS Core](#aeris-core)
    - [AERIS UI](#aeris-ui)
    - [AERIS Code](#aeris-code)
  - AERIS Standard
    - [AERIS Standard Core](#aeris-standard-core)
      - [AERIS Standard UI](#aeris-standard-ui)
      - [AERIS Standard Code](#aeris-standard-code)
    - [AERIS Standard UI Server](#aeris-standard-ui-server)
    - [AERIS Standard CLI](#aeris-standard-cli)

### AERIS Core

`core/Cargo.toml`

AERIS Standard 구현에 필요한 도구 모음.  
AERIS Standard에 종속적이지 않음.

### AERIS UI

`core/ui/Cargo.toml`

Syntax(Sementic) Highlighting, Parsing 등 텍스트 코드의 전반적인 처리를 담당.  
RE 언어를 처리 가능한 파서 프레임워크 지원.
RE, CSG는 명령적, CFG, RG 선언적으로 정의하는 편의 기능 지원.
선언적 문법 정의 기능은 매크로 기능 적극 활용.

### AERIS Code

`core/code/Cargo.toml`

실행 가능한 코드 관리, 생성을 담당.

### AERIS Standard Core

`standard/core/Cargo.toml`

AERIS 표준 언어 구현 모음.

### AERIS Standard UI

`standard/core/ui/Cargo.toml`

AERIS 표준 언어의 UI 관련 구현 모음.

### AERIS Standard Code

`standard/core/code/Cargo.toml`

AERIS 표준 언어의 코드 생성 관련 구현 모음.

### AERIS Standard UI Server

`standard/ui-server/Cargo.toml`

AERIS UI 서버.
Language 서버와 역할이 같음.

### AERIS Standard CLI

`standard/cli/Cargo.toml`

AERIS 컴파일러 및 관련 도구를 실행하기 위한 명령줄 인터페이스.
