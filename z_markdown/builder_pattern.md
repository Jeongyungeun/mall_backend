# Builder Pattern in Rust

## 개요

빌더 패턴(Builder Pattern)은 복잡한 객체의 생성 과정과 표현 방법을 분리하여 다양한 구성의 인스턴스를 생성할 수 있게 하는 생성 패턴입니다. 이 패턴은 특히 다음과 같은 상황에서 유용합니다:

- 객체 생성 과정이 복잡할 때
- 객체에 선택적 매개변수가 많을 때
- 객체 생성 시 불변성(immutability)을 유지하고 싶을 때
- 가독성 높은 객체 생성 코드를 작성하고 싶을 때

## Rust에서의 빌더 패턴 구현

아래 예제는 Rust에서 빌더 패턴을 구현하는 방법을 보여줍니다.

```rust
fn main() {
    // 빌더 패턴 사용 예제
    let item = Item::builder()
        .with_id("item-001")
        .with_name("테스트 아이템")
        .build();

    println!("생성된 아이템: {:?}", item);
}

/// 간단한 패턴
/// buildable trait는 builder를 반환하는 책임이 있다. 목적객체의 빈 인스턴스를 반환해야한다.
/// builder 자체는 chaining initialize가 가능하게 fluent interface를 제공한다.
/// 객체는 두가지 필요. Item, ItemBuilder
/// 트레이트도 두가지 필요. Builder, Buildable,
///

trait Builder<T> {
    fn new() -> Self;
    fn build(self) -> T;
}

#[derive(Debug, Clone)]
pub struct Item {
    item_id: String,
    item_name: String,
}

trait Buildable<Target, B: Builder<Target>> {
    fn builder() -> B;
}

impl Buildable<Item, ItemBuilder> for Item {
    fn builder() -> ItemBuilder {
        ItemBuilder::new()
    }
}

// 객체의 생성과 체이닝생성을 담당한다.
#[derive(Clone)]
pub struct ItemBuilder {
    item: Item,
}

impl ItemBuilder {
    // 소유권을 가져와서 수정 후 반환 (self를 소비)
    fn with_id(mut self, id: &str) -> Self {
        self.item.item_id = id.to_string();
        self
    }

    // 소유권을 가져와서 수정 후 반환 (self를 소비)
    fn with_name(mut self, name: &str) -> Self {
        self.item.item_name = name.to_string();
        self
    }
}

impl Builder<Item> for ItemBuilder {
    fn new() -> Self {
        Self {
            item: Item {
                item_id: "".to_string(),
                item_name: "".to_string(),
            },
        }
    }

    fn build(self) -> Item {
        self.item
    }
}


## 빌더 패턴 관계도

아래는 `Buildable` 트레이트, `Builder` 트레이트, 그리고 `ItemBuilder` 구조체 간의 관계를 보여주는 다이어그램입니다:

```
+-------------------+        +-------------------+
| trait Builder<T>  |        | trait Buildable   |
+-------------------+        +-------------------+
| fn new() -> Self  |        | fn builder() -> B |
| fn build(self)->T |        +-------------------+
+-------------------+                ^
         ^                           |
         |                           |
         |                           |
         |                      +---------+
         |                      |  Item   |
         |                      +---------+
         |                      | item_id |
         |                      | name    |
         |                      +---------+
         |                           ^
         |                           |
         |                           | 생성
+-------------------+                |
|   ItemBuilder     |<---------------+
+-------------------+
| item: Item        |
+-------------------+
| with_id()         |
| with_name()       |
+-------------------+
```

### 관계 설명

1. **Builder<T> 트레이트**:
   - 제네릭 타입 `T`에 대한 빌더 인터페이스를 정의합니다.
   - `new()`: 빌더 인스턴스를 생성합니다.
   - `build(self)`: 최종 객체 `T`를 생성합니다.

2. **Buildable<Target, B> 트레이트**:
   - 타겟 객체(`Target`)가 빌더(`B`)를 생성할 수 있게 합니다.
   - `B`는 `Builder<Target>` 트레이트를 구현해야 합니다.
   - `builder()`: 빌더 인스턴스를 반환합니다.

3. **Item 구조체**:
   - `Buildable<Item, ItemBuilder>` 트레이트를 구현합니다.
   - `builder()` 메서드를 통해 `ItemBuilder` 인스턴스를 생성합니다.

4. **ItemBuilder 구조체**:
   - `Builder<Item>` 트레이트를 구현합니다.
   - 내부에 `Item` 인스턴스를 가지고 있습니다.
   - `with_id()`, `with_name()` 등의 메서드를 통해 `Item`의 속성을 설정합니다.
   - `build()` 메서드를 통해 최종 `Item` 객체를 반환합니다.

### 호출 흐름

1. 클라이언트는 `Item::builder()`를 호출합니다.
2. `Item::builder()`는 `Buildable` 트레이트의 구현을 통해 `ItemBuilder::new()`를 호출합니다.
3. `ItemBuilder::new()`는 빈 `Item` 인스턴스를 가진 `ItemBuilder`를 생성합니다.
4. 클라이언트는 `with_id()`, `with_name()` 등의 메서드를 체이닝하여 `Item`의 속성을 설정합니다.
5. 마지막으로 `build()`를 호출하여 완성된 `Item` 객체를 얻습니다.

## 구성 요소 설명

### 1. 핵심 트레이트

#### `Builder<T>` 트레이트
```rust
trait Builder<T> {
    fn new() -> Self;
    fn build(self) -> T;
}
```
- `new()`: 빌더 인스턴스를 생성합니다.
- `build(self)`: 최종 객체를 생성하고 반환합니다. `self`를 소비하므로 빌더는 한 번만 사용할 수 있습니다.

#### `Buildable<Target, B>` 트레이트
```rust
trait Buildable<Target, B: Builder<Target>> {
    fn builder() -> B;
}
```
- 타겟 객체(`Target`)가 빌더(`B`)를 생성할 수 있게 합니다.
- 제네릭 매개변수 `B`는 `Builder<Target>` 트레이트를 구현해야 합니다.

### 2. 구체적인 구현

#### `Item` 구조체
```rust
#[derive(Debug, Clone)]
pub struct Item {
    item_id: String,
    item_name: String,
}
```
- 생성하려는 최종 객체입니다.
- `Debug`와 `Clone` 트레이트를 구현합니다.

#### `ItemBuilder` 구조체
```rust
#[derive(Clone)]
pub struct ItemBuilder {
    item: Item,
}
```
- `Item` 객체를 생성하기 위한 빌더입니다.
- 내부에 `Item` 인스턴스를 가지고 있습니다.

#### `Buildable` 구현
```rust
impl Buildable<Item, ItemBuilder> for Item {
    fn builder() -> ItemBuilder {
        ItemBuilder::new()
    }
}
```
- `Item`에 대한 `Buildable` 트레이트 구현입니다.
- `Item::builder()`를 호출하면 `ItemBuilder` 인스턴스를 반환합니다.

#### `Builder` 구현
```rust
impl Builder<Item> for ItemBuilder {
    fn new() -> Self {
        Self {
            item: Item {
                item_id: "".to_string(),
                item_name: "".to_string(),
            },
        }
    }

    fn build(self) -> Item {
        self.item
    }
}
```
- `ItemBuilder`에 대한 `Builder<Item>` 트레이트 구현입니다.
- `new()`: 빈 `Item` 인스턴스를 가진 빌더를 생성합니다.
- `build(self)`: 빌더가 가진 `Item` 인스턴스를 반환합니다.

#### 빌더 메서드
```rust
impl ItemBuilder {
    fn with_id(mut self, id: &str) -> Self {
        self.item.item_id = id.to_string();
        self
    }

    fn with_name(mut self, name: &str) -> Self {
        self.item.item_name = name.to_string();
        self
    }
}
```
- `with_id`, `with_name`: 각 필드를 설정하는 메서드입니다.
- 각 메서드는 `self`를 소비하고 수정된 `Self`를 반환하여 메서드 체이닝을 가능하게 합니다.
- 이런 방식을 "Fluent Interface"라고 합니다.

## 빌더 패턴의 장점

1. **가독성**: 메서드 체이닝을 통해 객체 생성 코드가 명확하고 읽기 쉬워집니다.
2. **유연성**: 필요한 필드만 설정할 수 있어 다양한 구성의 객체를 생성할 수 있습니다.
3. **불변성**: 객체가 한 번 생성되면 변경되지 않도록 할 수 있습니다.
4. **단계적 생성**: 복잡한 객체를 단계적으로 생성할 수 있습니다.

## 빌더 패턴의 확장

위 예제는 기본적인 빌더 패턴 구현이지만, 다음과 같은 방식으로 확장할 수 있습니다:

1. **필수 필드와 선택적 필드 구분**: 타입 시스템을 활용하여 필수 필드가 설정되지 않으면 `build()`를 호출할 수 없게 만들 수 있습니다.
2. **유효성 검사**: `build()` 메서드에서 객체의 유효성을 검사하고 `Result`나 `Option`을 반환할 수 있습니다.
3. **기본값 설정**: 선택적 필드에 기본값을 제공할 수 있습니다.

## 결론

빌더 패턴은 복잡한 객체 생성을 단순화하고 코드의 가독성을 높이는 강력한 디자인 패턴입니다. Rust에서는 트레이트 시스템과 소유권 모델을 활용하여 타입 안전하고 표현력 있는 빌더 패턴을 구현할 수 있습니다.
