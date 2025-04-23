# 데이터베이스 마이그레이션 가이드

## 1. 마이그레이션 개요

데이터베이스 마이그레이션은 데이터베이스 스키마의 변경 사항을 관리하는 체계적인 방법입니다. 이를 통해 개발자는 데이터베이스 구조를 버전 관리하고, 변경 사항을 추적하며, 다양한 환경(개발, 테스트, 프로덕션)에서 일관된 데이터베이스 상태를 유지할 수 있습니다.

### 1.1 마이그레이션의 이점

- **버전 관리**: 데이터베이스 스키마 변경 사항을 코드로 관리
- **협업 용이성**: 팀원 간 데이터베이스 스키마 변경 사항 공유 및 충돌 방지
- **배포 자동화**: CI/CD 파이프라인에 통합하여 자동 배포 가능
- **롤백 가능**: 문제 발생 시 이전 상태로 되돌릴 수 있음

## 2. SQLx 마이그레이션 기본 사용법

이 프로젝트에서는 Rust의 SQLx 라이브러리를 사용하여 데이터베이스 마이그레이션을 관리합니다.

### 2.1 마이그레이션 파일 구조

마이그레이션 파일은 일반적으로 다음과 같은 형식을 따릅니다:

```sql
-- 마이그레이션 설명 (주석)
--! Up
-- 업그레이드 쿼리들
CREATE TABLE IF NOT EXISTS example (
    id VARCHAR(64) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

--! Down
-- 다운그레이드 쿼리들 (롤백 시 실행)
DROP TABLE IF EXISTS example;
```

- **Up 섹션**: 마이그레이션 적용 시 실행될 쿼리
- **Down 섹션**: 마이그레이션 롤백 시 실행될 쿼리

### 2.2 마이그레이션 파일 명명 규칙

마이그레이션 파일은 다음과 같은 형식으로 이름을 지정합니다:

```
{타임스탬프}_{설명}.sql
```

예: `20250418212048_base_table.sql`

타임스탬프는 마이그레이션의 실행 순서를 결정하는 데 사용됩니다.

## 3. 마이그레이션 명령어

### 3.1 새 마이그레이션 생성

```bash
# SQLx CLI를 사용하여 새 마이그레이션 파일 생성
sqlx migrate add -r base_table
```

이 명령은 `migrations` 디렉토리에 `{타임스탬프}_base_table.sql` 파일을 생성합니다.

### 3.2 마이그레이션 실행

```bash
# 모든 보류 중인 마이그레이션 실행
sqlx migrate run --database-url "postgres://username:password@localhost/dbname"

# 또는 환경 변수 사용
export DATABASE_URL="postgres://username:password@localhost/dbname"
sqlx migrate run
```

### 3.3 마이그레이션 롤백

```bash
# 가장 최근 마이그레이션 롤백
sqlx migrate revert --database-url "postgres://username:password@localhost/dbname"
```

### 3.4 마이그레이션 상태 확인

```bash
# 적용된 마이그레이션과 보류 중인 마이그레이션 확인
sqlx migrate info --database-url "postgres://username:password@localhost/dbname"
```

## 4. 코드에서 마이그레이션 실행

애플리케이션 시작 시 자동으로 마이그레이션을 실행하려면 다음과 같이 코드를 작성할 수 있습니다:

```rust
use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::Migrator;
use std::path::Path;

async fn run_migrations(database_url: &str) -> Result<(), sqlx::Error> {
    // 데이터베이스 연결 풀 생성
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // 마이그레이션 실행
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(&pool).await?;

    Ok(())
}
```

## 5. 마이그레이션 모범 사례

### 5.1 마이그레이션 설계 원칙

1. **원자성**: 각 마이그레이션은 논리적으로 관련된 변경 사항만 포함해야 합니다.
2. **멱등성**: 마이그레이션은 여러 번 실행해도 동일한 결과를 보장해야 합니다 (`IF NOT EXISTS` 사용).
3. **가역성**: 가능한 모든 마이그레이션은 롤백 가능해야 합니다 (Down 섹션 구현).
4. **데이터 보존**: 스키마 변경 시 기존 데이터를 보존하는 방법을 고려해야 합니다.

### 5.2 주의사항

1. **프로덕션 데이터베이스 백업**: 프로덕션 환경에서 마이그레이션을 실행하기 전에 항상 백업을 수행하세요.
2. **대규모 테이블 변경**: 대규모 테이블의 구조를 변경할 때는 성능 영향을 고려하세요.
3. **외래 키 제약 조건**: 테이블 간 관계를 변경할 때는 외래 키 제약 조건을 적절히 처리해야 합니다.
4. **인덱스 관리**: 인덱스 추가/제거는 쿼리 성능에 영향을 미칠 수 있으므로 신중하게 계획하세요.

## 6. 실제 예제

### 6.1 기본 테이블 생성

```sql
--! Up
CREATE TABLE IF NOT EXISTS item (
    id VARCHAR(64) PRIMARY KEY,           -- ItemId
    name VARCHAR(255) NOT NULL,           -- ItemName
    price INTEGER,                        -- ItemPrice (u32 → INTEGER, NULL 허용)
    item_type VARCHAR(32) NOT NULL,       -- ItemType (enum → string 저장)
    item_images TEXT[],                   -- ItemImage (Vec<String> → TEXT 배열)
    description TEXT,                     -- ItemDescription (option)
    updated_at TIMESTAMPTZ NOT NULL,      -- DateTime<Utc>
    created_at TIMESTAMPTZ NOT NULL       -- DateTime<Utc>
);

--! Down
DROP TABLE IF EXISTS item;
```

### 6.2 테이블 수정 (컬럼 추가)

```sql
--! Up
ALTER TABLE item
ADD COLUMN stock INTEGER NOT NULL DEFAULT 0,
ADD COLUMN is_available BOOLEAN NOT NULL DEFAULT TRUE;

--! Down
ALTER TABLE item
DROP COLUMN stock,
DROP COLUMN is_available;
```

### 6.3 인덱스 추가

```sql
--! Up
CREATE INDEX idx_item_name ON item(name);
CREATE INDEX idx_item_type ON item(item_type);

--! Down
DROP INDEX IF EXISTS idx_item_name;
DROP INDEX IF EXISTS idx_item_type;
```

### 6.4 관계 테이블 생성

```sql
--! Up
CREATE TABLE IF NOT EXISTS category (
    id VARCHAR(64) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS item_category (
    item_id VARCHAR(64) REFERENCES item(id) ON DELETE CASCADE,
    category_id VARCHAR(64) REFERENCES category(id) ON DELETE CASCADE,
    PRIMARY KEY (item_id, category_id)
);

--! Down
DROP TABLE IF EXISTS item_category;
DROP TABLE IF EXISTS category;
```

## 7. 헥사고날 아키텍처에서의 데이터베이스 마이그레이션

헥사고날 아키텍처에서는 데이터베이스를 외부 시스템으로 간주하며, 데이터베이스 스키마는 driven 어댑터 계층에서 관리됩니다.

### 7.1 아키텍처 관점에서의 마이그레이션 관리

1. **도메인 모델과 데이터베이스 스키마 분리**: 도메인 모델은 데이터베이스 스키마와 독립적으로 설계됩니다.
2. **어댑터 책임**: 데이터베이스 어댑터는 도메인 모델과 데이터베이스 스키마 간의 매핑을 담당합니다.
3. **마이그레이션 자동화**: 애플리케이션 시작 시 자동으로 마이그레이션을 실행하여 스키마를 최신 상태로 유지합니다.

### 7.2 구현 예시

```rust
// config/database.rs
pub async fn init_database(config: &DatabaseConfig) -> Result<PgPool, Error> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.url)
        .await?;
    
    // 마이그레이션 실행
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    Ok(pool)
}

// main.rs
async fn main() -> Result<(), Error> {
    // 설정 로드
    let config = Config::from_env()?;
    
    // 데이터베이스 초기화 및 마이그레이션 실행
    let pool = init_database(&config.database).await?;
    
    // 리포지토리 어댑터 생성
    let item_repository = ItemRepositoryImpl::new(pool.clone());
    
    // 서비스 생성 및 의존성 주입
    let item_service = ItemService::new(item_repository);
    
    // 웹 서버 시작
    // ...
    
    Ok(())
}
```

## 8. 결론

데이터베이스 마이그레이션은 데이터베이스 스키마를 체계적으로 관리하고 변경 사항을 추적하는 데 필수적인 도구입니다. SQLx의 마이그레이션 기능을 활용하면 Rust 애플리케이션에서 효과적으로 데이터베이스 스키마를 관리할 수 있습니다.

헥사고날 아키텍처에서는 데이터베이스를 외부 시스템으로 간주하고, 도메인 모델과 데이터베이스 스키마를 분리하여 설계합니다. 이를 통해 비즈니스 로직이 데이터베이스 구현 세부사항에 의존하지 않도록 하고, 시스템의 유연성과 테스트 용이성을 높일 수 있습니다.
