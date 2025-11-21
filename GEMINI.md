# AERIS

다중 패러다임 범용 프로그래밍 언어.

## 특징 및 기능

### 컴파일러 플러그인

컴파일러에 플러그인을 추가해서 문법, Syntax Highting 지원 등 언어를 확장할 수 있음.

### 다중 런타임

네이티브 컴파일, LLVM 기반 인터프리터, REPL 뿐만 아니라 다른 프로그램에 이식할 수 있음.

### Premise

Premise(전제) 정의를 통해 컴파일타임에 값의 도메인 보장을 할 수 있음.  
명시적으로 런타임 검사를 요구하지 않는 이상 런타임 검사 없이 컴파일타임 정보만으로 검사함.

`premise <expression>` 형식으로 작성하며 `<expression>`이 true임을 보장함.

premise 정의 예시

```aeris
fn greet(name: String) {
  premise name.match(/^\w+$/);
  println("Hello, ${name}");
}

fn main() {
  greet("AERIS"); // Ok
  greet("!@#$%^&*"); // Compile Error
}
```

premise 런타임 검사 예시

```aeris
import std.io.stdin

fn greet(name: String) {
  premise name.match(/^\w+$/);
  println("Hello, ${name}");
}

fn main() {
  let name = stdin.read_line();
  greet(sat name);
}
```

# 프로젝트 개요

AERIS의 부트스트랩 컴파일러를 Rust로 작성함.  
이후 AERIS로 AERIS 컴파일러 재작성 예정.

## Core

`core`에 위치하며, AERIS 컴파일러가 의존성을 가지는 UI 프레임워크, AERIS IR 등의 코어 기능을 작성함.  
기본적으로 AERIS를 위한것이지만 AERIS에 종속되는것을 지양함.

### UI 프레임워크

Syntax, Syntax Highlighting, Language Server 등을 UI(User Interface)라고 봄.  
해당 관점에 따라 언어 UI 제공에 편의성을 제공하는 프레임워크.

### AERIS IR

TODO

## AERIS Standard

`standard`에 위치하며, AERIS 표준 언어와 도구 모음을 제공함.  
AERIS 표준 언어를 AERIS Standard라 칭하지만 확정 아님.  
이에 따라 추후 디렉토리가 변경될 가능성 있음.

### AERIS Standard CLI

### AERIS Standard Core

### AERIS Standard UI
