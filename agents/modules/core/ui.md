# AERIS Core / UI

# `cluster!` 파싱 구현 계획

## 목표와 범위

`cluster!`는 Rust 타입에 대응하는 **Shard**와 매크로 내부에서만 쓰이는
**Lambda**로 문법을 정의한다. 첫 구현의 목표는 생성된 Shard의
`parse(&str)` API로 입력 전체를 파싱하고 Rust AST를 만드는 것이다.

첫 구현에 포함한다.

- 문자열/문자 리터럴과 문자 집합(`{'a'..'z'}`, 제외 집합 포함)
- Shard 및 Lambda 참조
- 순서열, 괄호 대안(`Term`), 모든 반복 수식어 및 lazy 표기
- `struct`/`enum`에 대응하는 결과 타입 생성
- 외부에서 `use`로 가져온 Shard 참조와 Rust 타입 검증
- first set 등을 이용한 런타임 파싱 최적화
- GLR식으로 가능한 결과를 유지하고, 공개 API에서는 첫 완성 트리 선택

하이라이팅, 상세 오류, 복구 지점, 진단용 trait 확장은 후속 단계다.

## 문법과 이름 해석

- `struct FooShard`와 `enum BarShard`는 각각 실제 Rust 타입을 생성한다.
- `Digits (Digit+)`처럼 타입 선언이 없는 항목은 Lambda다. Lambda는
  `cluster!` 블록 안에서만 보이고, 별도의 Rust 타입을 만들지 않는다.
- 같은 이름의 Lambda와 외부 Shard가 모두 보이면 Lambda를 먼저 해석한다.
- 외부 Shard는 `use foo::FooShard;` 후 문법 안에서 `FooShard`로 쓴다.
  `foo::FooShard` 같은 경로 표기는 첫 구현에서 지원하지 않는다.
- 각 외부 Shard 참조에는 매크로가 다음과 동등한 타입 가드를 생성하여,
  Rust의 이름 해석과 trait 구현 검증을 이용한다.

  ```rust
  fn guard<S: ::aeris::ui::Shard>() {}
  const _: () = { guard::<FooShard>() };
  ```

## 생성 타입과 캡처 규칙

입력의 일부를 보관하므로 생성 타입은 입력 수명을 가진다.

```rust
impl<'input> JSONNumber<'input> {
    pub fn parse(input: &'input str) -> Result<Self, ::aeris::ui::Error>;
}
```

- 리터럴과 문자 집합은 매치한 부분 문자열 `&'input str`을 만든다.
- `?`는 `Option<T>`다.
- `*`, `+`, `[N]`, `[N,]`, `[,N]`, `[N,M]`은 `Vec<T>`다.
- Shard 참조는 대응되는 `FooShard<'input>` 값이다.
- 괄호 대안(`Term`)은 Lambda와 같이 취급하고, 내부 구조를 노출하지
  않는다. 따라서 `foo: (Digit)`은 `&'input str`, `foo: (Digit)*`은
  `Vec<&'input str>`, `foo: ((Digit)*)`은 `&'input str`이다.
- 대안 내부의 세부 AST가 필요하면 해당 부분을 별도의 Rust `struct` 또는
  `enum` Shard로 명시한다.
- `struct`의 named entry는 생성 struct의 필드가 된다. tuple sequence와
  enum variant의 payload 모양은 위 캡처 규칙을 일관되게 적용해 생성한다.

## AST 파서 정비

`core/ui/proc/src/ast.rs`는 문법 표면을 파싱한다.

1. `TermAST::parse`는 반복문 종료 뒤 마지막 `entries`를 `alts`에 넣어야
   한다. 현재 저장된 수정은 이를 처리한다.
2. 빈 대안은 지원하지 않는다. 따라서 `()`와 `(A |)`는 유효한 대안을
   만들지 않으며, 필요하면 명시적인 epsilon 문법을 후속 기능으로 설계한다.
3. AST/HIR에는 span을 유지해 매크로 오류가 해당 문법 요소를 가리키게 한다.
4. HIR 단계에서 Lambda 우선 이름 해석, 외부 Shard 참조 수집, 재귀 참조
   분석, 수식어의 최소/최대 소비량을 정규화한다.
5. HIR 단계에서 반복 가능한 빈 매치(`(A?)*` 등)를 감지해 컴파일 오류로
   막는다. 이는 무한 파싱 루프를 방지한다.

## 런타임 모델

`Shard` trait은 개별 Shard의 최적화된 문법을 한 번 만들고 재사용하도록
확장한다. 공개 형태는 구현 시 조정 가능하지만 의미는 다음과 같다.

```rust
pub trait Shard<'input>: Sized {
    fn optimized() -> &'static OptimizedShard<Self>;
    fn parse(input: &'input str) -> Result<Self, Error>;
}
```

- `optimized()`는 `OnceLock` 등으로 Shard별로 캐시된 불변 문법을 돌려준다.
- 캐시는 정규화된 노드, 대안 순서, 반복 경계, first set, nullable 정보와
  외부 Shard 호출 지점을 포함한다.
- 재귀 문법에서 초기화가 서로의 `optimized()`를 재귀 호출해 교착하지 않도록,
  캐시 구성은 참조 대상을 즉시 최적화하지 않는 그래프/핸들 방식으로 한다.
- 실행 중 파서는 lexer를 따로 두지 않고 문자열 offset을 직접 이동한다.
  문자 리터럴/집합은 UTF-8 경계를 지키며 매치한다.
- first set과 nullable 정보로 불가능한 대안을 파싱 전에 건너뛴다.
- Lambda와 Term은 별도 AST 값을 만들지 않고 시작/끝 offset만 전달해
  `&str` 슬라이스를 만든다.

## GLR 실행과 결정성

- 파서는 `(현재 노드, 입력 offset, 부분 결과)`의 여러 상태를 유지한다.
- 대안, 재귀, 반복에서 여러 경로가 가능하면 모두 계속 평가한다.
- 중복 상태는 위치와 문법 상태 기준으로 합쳐 폭발을 줄인다. 결과 값이
  달라지는 경우에는 필요한 backpointer/값을 보존한다.
- 공개 `parse`는 입력 전체를 소비한 결과 중 **첫 번째** 트리를 반환한다.
- 첫 번째의 의미를 안정적으로 만들기 위해 문법에 쓴 대안 순서와 순서열
  순서를 보존한다. 일반 반복은 greedy, `?`가 붙은 lazy 반복은 가능한
  짧은 결과를 먼저 탐색한다.
- 전체 입력을 소비한 결과가 없으면 `Error`를 반환한다. 상세 기대 토큰,
  위치, 복구 정보는 후속 단계에서 확장한다.

## 구현 순서

1. `aeris-ui-lib`에 입력 위치, 매치 결과, `Error`, 최적화 문법 노드,
   `Shard` trait의 최소 런타임 인터페이스를 둔다.
2. proc macro의 AST를 HIR로 낮춘다. 이름 해석과 유효성 검사, 결과 타입
   추론, 외부 Shard 타입 가드를 여기서 생성한다.
3. HIR로부터 수명 파라미터가 있는 struct/enum, `Shard` 구현,
   `optimized()` 캐시, `parse()` 진입점을 생성한다.
4. 리터럴/집합/순서열/대안/Shard 호출을 실행하는 최소 파서를 만든다.
5. 반복 수식어와 greedy/lazy 결과 순서를 추가하고, nullable 반복을
   컴파일 단계에서 거부한다.
6. 재귀 Lambda, 재귀 Shard, 외부 Shard를 추가한다.
7. first set/nullable 계산과 상태 병합을 적용하고 벤치마크로 확인한다.
8. JSON 문법을 통합 예제로 삼아 숫자, 문자열, 배열, 객체, 값 전체를
   파싱하는 테스트를 작성한다.

## 검증 항목

- `TermAST`가 한 개/여러 개 대안의 마지막 대안을 보존하는 단위 테스트
- `Option<&str>`, `Vec<&str>`, nested Term의 `&str` 타입 생성 컴파일 테스트
- struct 필드, tuple sequence, enum variant의 값 생성 테스트
- Lambda 우선 이름 해석과 외부 Shard 타입 가드 컴파일 테스트
- 재귀 문법, 중첩 반복, lazy/greedy 순서, 모호 문법의 첫 트리 선택 테스트
- 잘못된 문자, 부분 소비, 빈 매치 반복에 대한 실패 테스트
- 캐시 초기화가 반복 parse에서 재사용되고 재귀 초기화 교착이 없는지 확인

## 후속 확장

`Shard`와 최적화 문법에 메타데이터 슬롯을 남겨 아래 기능을 추가한다.

- 구문 하이라이팅 정보
- 기대 토큰과 소스 위치를 갖는 오류 메시지
- 복구 지점/동기화 규칙
- 우선순위, 결합성, 명시적 모호성 제어
- epsilon 또는 빈 대안 문법
