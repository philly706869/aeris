# AERIS

다중 패러다임 범용 프로그래밍 언어.

## 특징

### 컴파일러 플러그인

컴파일러에 플러그인을 추가해서 문법, Syntax Highting 지원 등 언어를 확장할 수 있음.

### 다중 런타임

네이티브 컴파일, LLVM 기반 인터프리터, REPL 뿐만 아니라 다른 프로그램에 이식할 수 있음.

### 강력한 컴파일타임

Premise(전제) 정의를 통해 컴파일타임에 값의 보장을 할 수 있음.  
런타임 검사 또는 증명 없는 보장은 절대 이뤄지지 않음: 전제에 대한 절대적인 신뢰성을 확보함.  
수학적, 논리적 증명을 통해 런타임에 수행하지 않아도 되는 검사를 자동으로 생략함: 강력한 최적화 가능.

#### 전제

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
  let success = greet(satisfy name);
}
```
