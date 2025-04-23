# 헥사고날 아키텍처 가이드

## 1. 헥사고날 아키텍처 개요

헥사고날 아키텍처(Hexagonal Architecture)는 애플리케이션의 핵심 로직을 외부 요소로부터 분리하여 유지보수성, 테스트 용이성, 확장성을 높이는 소프트웨어 아키텍처 패턴입니다. 이 아키텍처는 포트와 어댑터 패턴(Ports and Adapters Pattern)이라고도 불립니다.

### 1.1 주요 원칙

- **도메인 중심 설계**: 비즈니스 로직이 아키텍처의 중심에 위치
- **의존성 역전 원칙**: 내부 계층이 외부 계층에 의존하지 않음
- **포트와 어댑터 패턴**: 인터페이스(포트)와 구현체(어댑터)의 분리
- **관심사 분리**: 각 계층이 자신의 책임에만 집중

## 2. 프로젝트 구조

### 2.1 계층 구조

```
src/
├── domain/              # 도메인 계층
│   ├── model/           # 도메인 모델
│   ├── port/            # 인터페이스(포트)
│   │   ├── primary/     # 애플리케이션이 외부에 제공하는 인터페이스
│   │   └── secondary/   # 애플리케이션이 외부 시스템에 요청하는 인터페이스
│   ├── event/           # 도메인 이벤트
│   └── event_handler/   # 이벤트 핸들러
├── application/         # 애플리케이션 계층
│   └── usecase/         # 유스케이스 구현
├── driven/              # 주도되는(Driven) 어댑터
├── driving/             # 주도하는(Driving) 어댑터
│   └── rest_handler/    # REST API 핸들러
├── config/              # 설정 및 의존성 주입
└── errors/              # 에러 처리 시스템
```

### 2.2 주요 컴포넌트

1. **도메인 모델**: `domain/model/item.rs`
2. **포트 정의**:
   - Primary 포트: `domain/port/primary/item_service_port.rs`
   - Secondary 포트: `domain/port/secondary/item_repository_port.rs`
3. **유스케이스 구현**: `application/usecase/item.rs`
4. **어댑터 구현**:
   - Driving 어댑터: `driving/rest_handler/item_handler.rs`
   - Driven 어댑터: `driven/item_repository_impl.rs`

## 3. 에러 처리 전략

헥사고날 아키텍처에서는 계층별 에러 타입을 정의하고 변환하는 방식을 채택합니다.

### 3.1 에러 타입

1. **DomainError**: 도메인 계층의 에러
   ```rust
   pub enum DomainError {
       SaveError(String),
       DeleteError(String),
   }
   ```

2. **DatabaseError**: 데이터베이스 관련 에러
   ```rust
   pub enum DatabaseError {
       Connection(String),
       Query(String),
       Duplicate(String),
       NotFound(String),
       Other(String),
   }
   ```

3. **AppError**: 애플리케이션 전체 에러
   ```rust
   pub enum AppError {
       Domain(DomainError),
       Database(DatabaseError),
       ExternalService(String),
       Validation(String),
       Authentication(String),
       Authorization(String),
       Other(String),
   }
   ```

### 3.2 에러 변환 메커니즘

From 트레이트를 구현하여 계층 간 에러 변환:

```rust
impl From<SqlxError> for DatabaseError {
    fn from(error: SqlxError) -> Self {
        // 변환 로직
    }
}

impl From<DatabaseError> for DomainError {
    fn from(error: DatabaseError) -> Self {
        // 변환 로직
    }
}

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        AppError::Domain(error)
    }
}
```

### 3.3 HTTP 응답으로 변환

```rust
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Domain(DomainError::SaveError(e)) => {
                HttpResponse::BadRequest().json(json_error("Save error", e))
            }
            // 다른 케이스들...
        }
    }
}
```

## 4. 이벤트 기반 아키텍처

헥사고날 아키텍처와 함께 이벤트 기반 아키텍처를 사용하여 시스템 컴포넌트 간의 결합도를 낮추고 확장성을 높일 수 있습니다.

### 4.1 도메인 이벤트

도메인 이벤트는 "도메인 내에서 발생한 중요한 사건"을 나타냅니다.

```rust
// domain/event/item_created.rs
pub struct ItemCreatedEvent {
    pub event_id: Uuid,
    pub item_id: String,
    pub name: String,
    pub price: u32,
    pub occurred_at: DateTime<Utc>,
}
```

### 4.2 이벤트 핸들러

이벤트 핸들러는 이벤트가 발생했을 때 수행해야 할 작업을 정의합니다.

```rust
// domain/event_handler/item_event_handler.rs
#[async_trait]
pub trait ItemEventHandler: Send + Sync {
    async fn handle_item_created(&self, event: ItemCreatedEvent);
    async fn handle_item_price_changed(&self, event: ItemPriceChangedEvent);
}
```

### 4.3 이벤트 발행 포트

```rust
// domain/port/secondary/event_publisher_port.rs
#[async_trait]
pub trait EventPublisherPort: Send + Sync {
    async fn publish_item_created(&self, event: ItemCreatedEvent);
    async fn publish_item_price_changed(&self, event: ItemPriceChangedEvent);
}
```

### 4.4 요청-응답 패턴과 이벤트 기반 패턴의 조합

헥사고날 아키텍처에서는 두 패턴을 상호 보완적으로 사용하는 것이 이상적입니다:

1. **요청-응답 패턴 (Primary Ports)**:
   - 외부에서 애플리케이션으로의 요청 처리
   - 사용자 인터페이스와의 상호작용
   - 즉각적인 피드백이 필요한 작업

2. **이벤트 기반 패턴 (Domain Events)**:
   - 도메인 상태 변경에 따른 부수 효과 처리
   - 시스템 내부 컴포넌트 간 통신
   - 비동기적으로 처리해도 되는 작업

```rust
pub async fn create_item(&self, item: Item) -> Result<Item, DomainError> {
    // 1. 요청-응답 패턴: 아이템 저장 및 즉시 응답
    self.repo.save(&item).await?;
    
    // 2. 이벤트 기반 패턴: 부수 효과를 위한 이벤트 발행
    let event = ItemCreatedEvent::new(
        item.id.value().clone().unwrap_or_default(),
        item.name.value().clone().unwrap_or_default(),
    );
    
    self.event_publisher.publish_item_created(event).await;
    
    // 3. 요청에 대한 응답 반환
    Ok(item)
}
```

## 5. 결론

헥사고날 아키텍처와 이벤트 기반 아키텍처를 조합하면 다음과 같은 이점을 얻을 수 있습니다:

- **유지보수성**: 코드의 구조가 명확하고 변경이 용이함
- **테스트 용이성**: 각 계층을 독립적으로 테스트 가능
- **확장성**: 새로운 기능 추가가 기존 코드에 영향을 최소화
- **느슨한 결합**: 시스템 컴포넌트 간의 의존성 감소
- **도메인 중심**: 비즈니스 로직이 기술적 구현 세부사항에 영향받지 않음
