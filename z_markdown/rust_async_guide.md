# Rust 비동기 프로그래밍 가이드

## 1. Rust 비동기 프로그래밍의 기본 개념

Rust의 비동기 프로그래밍은 효율적인 I/O 바운드 작업 처리를 위해 설계되었으며, 다음과 같은 핵심 개념을 기반으로 합니다.

### 1.1 Future 트레이트

`Future` 트레이트는 Rust 비동기 프로그래밍의 핵심입니다. 이는 "나중에 완료될 수 있는 작업"을 나타냅니다.

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

- **Output**: Future가 완료되면 생성되는 값의 타입
- **poll**: Future의 진행 상태를 확인하는 메서드
- **Poll**: `Ready(T)` 또는 `Pending` 값을 반환하는 열거형

### 1.2 async/await 문법

Rust는 `async`/`await` 문법을 통해 비동기 코드를 동기 코드처럼 작성할 수 있게 해줍니다.

```rust
async fn read_file(path: &str) -> Result<String, std::io::Error> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}
```

- **async fn**: `Future`를 반환하는 함수를 정의합니다.
- **await**: `Future`가 완료될 때까지 현재 비동기 함수의 실행을 일시 중단합니다.

## 2. 비동기 코드의 내부 동작

### 2.1 상태 머신 변환

컴파일러는 `async` 함수나 블록을 상태 머신으로 변환합니다. 이 상태 머신은 `Future` 트레이트를 구현합니다.

```rust
// 이 비동기 함수는
async fn example(s: String) -> String {
    let a = step_one(s).await;
    let b = step_two(a).await;
    b
}

// 내부적으로 다음과 같은 상태 머신으로 변환됩니다
enum ExampleFuture {
    Start(String),
    WaitingOnStepOne(StepOneFuture),
    WaitingOnStepTwo(StepTwoFuture),
    Done,
}

impl Future for ExampleFuture {
    type Output = String;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        // 상태에 따라 다른 동작 수행
        // ...
    }
}
```

### 2.2 클로저와 캡처

`async` 블록은 내부적으로 클로저처럼 동작하며, 사용된 변수를 캡처합니다.

```rust
async fn process(data: String) {
    // `data`는 이 비동기 블록의 상태 머신에 캡처됩니다
    let processed = data + " processed";
    println!("{}", processed);
}
```

이 캡처 메커니즘 때문에, 비동기 함수의 매개변수는 `Send` 트레이트를 구현해야 합니다. 왜냐하면 상태 머신이 다른 스레드로 이동할 수 있기 때문입니다.

## 3. 실행자(Executor)와 런타임

### 3.1 실행자의 역할

실행자는 `Future`를 폴링하고 완료될 때까지 관리하는 역할을 합니다. Rust 표준 라이브러리는 실행자를 제공하지 않으므로, 대부분 외부 크레이트(Tokio, async-std 등)를 사용합니다.

```rust
#[tokio::main]
async fn main() {
    // Tokio 실행자가 이 비동기 함수를 실행합니다
}
```

### 3.2 다중 스레드 실행

대부분의 비동기 런타임은 기본적으로 다중 스레드 실행자를 사용합니다. 이는 `Future`가 여러 스레드에서 실행될 수 있음을 의미합니다.

```rust
// 기본적으로 다중 스레드 실행자 사용
#[tokio::main]
async fn main() {
    // ...
}

// 명시적으로 단일 스레드 실행자 지정
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // ...
}
```

### 3.3 작업 훔치기(Work Stealing)

다중 스레드 실행자는 일반적으로 작업 훔치기 알고리즘을 사용합니다. 이는 한 스레드에서 시작된 `Future`가 다른 스레드에서 완료될 수 있음을 의미합니다.

```
스레드 1: Future A 시작 → 블로킹 → 다른 작업 처리
스레드 2: (유휴 상태) → Future A 훔치기 → Future A 완료
```

이러한 작업 훔치기 메커니즘 때문에, 비동기 코드에서 사용되는 모든 데이터는 스레드 간에 안전하게 이동할 수 있어야 합니다(`Send` 트레이트 구현).

## 4. Send와 Sync 트레이트

### 4.1 Send 트레이트

`Send` 트레이트는 타입이 스레드 간에 안전하게 전송될 수 있음을 나타냅니다.

```rust
pub unsafe auto trait Send {}
```

- **auto trait**: 명시적으로 구현하지 않아도, 모든 필드가 `Send`를 구현하면 자동으로 구현됩니다.
- **unsafe**: 안전하지 않은 구현을 방지하기 위해 `unsafe` 키워드가 사용됩니다.

### 4.2 비동기 코드에서의 Send 요구사항

비동기 함수에서 `Send` 트레이트가 필요한 이유:

1. **상태 머신의 이동**: 비동기 함수가 생성한 상태 머신은 다른 스레드로 이동할 수 있습니다.
2. **캡처된 변수**: 상태 머신은 함수의 매개변수를 포함한 모든 변수를 캡처합니다.
3. **스레드 안전성**: 캡처된 변수가 다른 스레드로 이동하므로, 이들은 `Send`를 구현해야 합니다.

```rust
async fn safe(s: String) {} // String은 Send를 구현하므로 안전
async fn unsafe(rc: std::rc::Rc<String>) {} // Rc는 Send를 구현하지 않으므로 다중 스레드 환경에서 사용 불가
```

### 4.3 Send가 아닌 타입 처리

`Send`를 구현하지 않는 타입을 비동기 코드에서 사용하는 방법:

1. **단일 스레드 실행자 사용**:
   ```rust
   #[tokio::main(flavor = "current_thread")]
   async fn main() {
       let rc = std::rc::Rc::new(String::from("hello"));
       process(rc).await; // 단일 스레드에서는 Rc 사용 가능
   }
   ```

2. **async_trait에서 ?Send 사용**:
   ```rust
   #[async_trait::async_trait(?Send)]
   trait MyTrait {
       async fn method(&self, rc: std::rc::Rc<String>);
   }
   ```

3. **즉시 소비**:
   ```rust
   async fn process(data: impl Into<String>) {
       let s = data.into(); // 즉시 String으로 변환하여 소비
       // 이후 코드에서는 s만 사용
   }
   ```

## 5. 비동기 트레이트

### 5.1 async_trait 매크로

Rust는 현재 트레이트에서 직접 `async fn`을 지원하지 않습니다. 이를 해결하기 위해 `async_trait` 매크로를 사용합니다.

```rust
use async_trait::async_trait;

#[async_trait]
trait Repository {
    async fn save(&self, item: &Item) -> Result<(), Error>;
    async fn find_by_id(&self, id: Id) -> Option<Item>;
}
```

### 5.2 내부 동작

`async_trait` 매크로는 비동기 메서드를 다음과 같이 변환합니다:

```rust
trait Repository {
    fn save<'life0, 'async_trait>(
        &'life0 self,
        item: &'life0 Item,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'async_trait>>
    where
        'life0: 'async_trait;
    
    fn find_by_id<'life0, 'async_trait>(
        &'life0 self,
        id: Id,
    ) -> Pin<Box<dyn Future<Output = Option<Item>> + Send + 'async_trait>>
    where
        'life0: 'async_trait;
}
```

여기서 `+ Send`가 추가되는 것을 볼 수 있습니다. 이는 반환된 `Future`가 스레드 간에 안전하게 이동할 수 있어야 함을 의미합니다.

### 5.3 트레이트 매개변수와 Send

트레이트 메서드의 매개변수가 `Send`를 구현해야 하는 이유:

1. **캡처 메커니즘**: 매개변수는 생성된 `Future`에 캡처됩니다.
2. **스레드 간 이동**: 이 `Future`는 다른 스레드로 이동할 수 있습니다.
3. **안전성 보장**: 따라서 모든 캡처된 변수는 `Send`를 구현해야 합니다.

```rust
#[async_trait]
trait Repository {
    // String은 Send를 구현하므로 안전
    async fn find_by_query(&self, query: String) -> Result<(), Error>;
    
    // impl Into<String>은 Send를 구현하지 않는 타입을 포함할 수 있으므로 불안전
    // async fn find_by_query(&self, query: impl Into<String>) -> Result<(), Error>;
}
```

## 6. 실용적인 비동기 패턴

### 6.1 동시성 처리

여러 비동기 작업을 동시에 실행하는 방법:

```rust
use futures::future::join_all;

async fn process_items(items: Vec<Item>) -> Vec<Result<(), Error>> {
    let futures = items.into_iter().map(process_item);
    join_all(futures).await
}

async fn process_item(item: Item) -> Result<(), Error> {
    // 아이템 처리 로직
    Ok(())
}
```

### 6.2 타임아웃 처리

비동기 작업에 타임아웃 적용:

```rust
use tokio::time::{timeout, Duration};

async fn process_with_timeout(data: String) -> Result<(), Error> {
    let result = timeout(Duration::from_secs(5), process(data)).await;
    match result {
        Ok(process_result) => process_result,
        Err(_) => Err(Error::Timeout),
    }
}
```

### 6.3 취소 처리

비동기 작업 취소:

```rust
use tokio::select;
use tokio::sync::oneshot;

async fn cancellable_process(
    data: String,
    cancel_rx: oneshot::Receiver<()>,
) -> Result<(), Error> {
    select! {
        result = process(data) => result,
        _ = cancel_rx => Err(Error::Cancelled),
    }
}
```

## 7. 헥사고날 아키텍처에서의 비동기 프로그래밍

헥사고날 아키텍처에서 비동기 프로그래밍을 적용할 때 고려해야 할 사항:

### 7.1 포트 정의

포트(인터페이스)는 비동기 메서드를 포함할 수 있으며, 이는 `async_trait` 매크로를 사용하여 정의합니다:

```rust
#[async_trait]
pub trait ItemRepositoryPort {
    async fn save(&self, item: &Item) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: ItemId) -> Option<Item>;
    async fn delete(&self, id: ItemId) -> Result<(), DomainError>;
    async fn update(&self, id: ItemId) -> Result<(), DomainError>;
    async fn find_by_query(&self, query: String) -> Result<(), DomainError>;
}
```

### 7.2 어댑터 구현

어댑터는 포트를 구현하며, 실제 비동기 작업을 수행합니다:

```rust
#[async_trait]
impl ItemRepositoryPort for ItemRepositoryImpl {
    async fn save(&self, item: &Item) -> Result<(), DomainError> {
        sqlx::query("INSERT INTO item (id, name) VALUES ($1, $2)")
            .bind(item.id.value())
            .bind(&item.name.value())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    
    // 다른 메서드 구현...
}
```

### 7.3 유스케이스 구현

유스케이스는 포트를 통해 비동기 작업을 조율합니다:

```rust
#[async_trait]
impl ItemServicePort for ItemService<impl ItemRepositoryPort> {
    async fn create_item(&self, item: Item) -> Result<Item, DomainError> {
        // 아이템 저장
        self.repo.save(&item).await?;
        
        // 이벤트 발행
        let event = ItemCreatedEvent::new(
            item.id.value().clone().unwrap_or_default(),
            item.name.value().clone().unwrap_or_default(),
        );
        
        self.event_publisher.publish_item_created(event).await;
        
        Ok(item)
    }
}
```

## 8. 결론

Rust의 비동기 프로그래밍 모델은 강력하고 효율적이지만, 몇 가지 제약과 복잡성을 수반합니다. 특히 `Send` 트레이트 요구사항은 처음에는 직관적이지 않을 수 있지만, 이는 Rust의 안전성 보장을 위한 중요한 부분입니다.

비동기 함수의 매개변수가 `Send`를 구현해야 하는 이유는, 비동기 블록이 어떤 스레드에서 실행될지 모르는 상황에서, 매개변수가 다른 스레드로 캡처되어 이동할 가능성이 있기 때문입니다.

이러한 제약은 때로는 불편하게 느껴질 수 있지만, 컴파일 시점에 많은 동시성 버그를 방지하고, 더 안전하고 예측 가능한 코드를 작성할 수 있게 해줍니다.
