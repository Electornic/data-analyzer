# 데이터 분석기 (Data Analyzer)

Rust로 구현된 대화형 데이터 분석 도구입니다. CSV 및 Excel 파일을 읽어 다양한 통계 분석과 시각화를 제공합니다.

## 🚀 주요 기능

### 📊 데이터 분석
- **기초통계량 계산**: 평균, 중앙값, 표준편차, 최솟값, 최댓값, 사분위수
- **빈도 분석**: 범주형 데이터의 빈도 분포 분석
- **데이터 요약**: 데이터셋의 행/열 개수, 데이터 타입 정보

### 📈 시각화
- **Box Plot**: 데이터의 분포와 이상치 시각화
- **QQ Plot**: 정규분포 적합성 검정을 위한 QQ 플롯
- **Histogram**: 데이터의 분포 히스토그램

### 📁 파일 처리
- **CSV 파일 읽기/쓰기**: 한글 데이터 완벽 지원
- **Excel 파일 읽기**: .xlsx, .xls 파일 지원
- **데이터 추출**: 특정 행/열 추출 및 새 파일로 저장
- **표본 추출**: 무작위 표본 추출 기능

### 🖥️ 사용자 인터페이스
- **대화형 인터페이스**: 명령어 기반 대화형 실행
- **한글 지원**: 완전한 한글 인터페이스 및 데이터 처리
- **데모 모드**: 샘플 데이터를 이용한 기능 시연

## 📋 시스템 요구사항

- **Rust**: 1.70 이상
- **운영체제**: Windows, macOS, Linux

## 🛠️ 설치 방법

### 1. Rust 설치
```bash
# Rust 설치 (https://rustup.rs/)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 프로젝트 클론 및 빌드
```bash
git clone <repository-url>
cd data-analyzer
cargo build --release
```

## 🎯 사용 방법

### 프로그램 실행
```bash
cargo run
```

프로그램을 실행하면 다음과 같은 인사말과 함께 대화형 인터페이스가 시작됩니다:
```
안녕하세요. 데이터 분석 프로그램입니다

데이터 분석기 (Data Analyzer)
사용 가능한 명령어:
  analyze <파일경로>  - CSV 또는 Excel 파일 분석
  demo               - 샘플 데이터로 데모 실행
  help               - 도움말 표시
  exit               - 프로그램 종료

명령어를 입력하세요 (help, analyze, demo, exit):
```

### 명령어 사용법

#### 1. 파일 분석
```bash
analyze data.csv
analyze 데이터.xlsx
```

#### 2. 데모 실행
```bash
demo
```
샘플 데이터를 생성하고 모든 분석 기능을 시연합니다.

#### 3. 도움말
```bash
help
```

#### 4. 프로그램 종료
```bash
exit
# 또는
quit
# 또는
종료
```

## 📊 분석 결과 예시

### 기초통계량
```
=== 기초통계량 (나이) ===
개수: 100
평균: 42.35
중앙값: 42.00
표준편차: 12.89
최솟값: 20.00
최댓값: 64.00
1사분위수: 32.00
3사분위수: 53.00
```

### 빈도 분석
```
=== 빈도 분석 (등급) ===
A: 23 (23.0%)
B: 28 (28.0%)
C: 25 (25.0%)
D: 24 (24.0%)
```

### 생성되는 파일들
- `boxplot_나이.png`: 나이 데이터의 박스 플롯
- `qqplot_점수.png`: 점수 데이터의 QQ 플롯
- `histogram_나이.png`: 나이 데이터의 히스토그램
- `sample_data_random_sample.csv`: 무작위 표본 추출 결과
- `sample_data_column_이름.csv`: 특정 열 추출 결과

## 📁 프로젝트 구조

```
data-analyzer/
├── src/
│   ├── main.rs              # 메인 프로그램 및 대화형 인터페이스
│   ├── data_analyzer.rs     # 데이터 분석 로직
│   └── read_file.rs         # 파일 읽기/쓰기 기능
├── script/
│   └── Data-Analyer-Spec.md # 프로젝트 명세서
├── test_demo.sh             # 데모 테스트 스크립트
├── test_interactive.sh      # 대화형 인터페이스 테스트 스크립트
├── Cargo.toml              # 프로젝트 설정 및 의존성
└── README.md               # 프로젝트 문서
```

## 📦 의존성

- **csv** (1.3): CSV 파일 처리
- **calamine** (0.22): Excel 파일 읽기
- **serde** (1.0): 데이터 직렬화/역직렬화
- **plotters** (0.3): 그래프 생성
- **rand** (0.8): 무작위 표본 추출
- **statrs** (0.16): 통계 계산
- **anyhow** (1.0): 에러 처리

## 🧪 테스트

### 대화형 인터페이스 테스트
```bash
./test_interactive.sh
```

### 데모 기능 테스트
```bash
./test_demo.sh
```

## 💡 사용 예시

### 1. CSV 파일 분석
```bash
# 프로그램 실행
cargo run

# 명령어 입력
명령어를 입력하세요: analyze sales_data.csv
```

### 2. 데모 실행
```bash
# 프로그램 실행
cargo run

# 데모 명령어 입력
명령어를 입력하세요: demo
```

데모를 실행하면 다음과 같은 작업이 자동으로 수행됩니다:
1. 샘플 데이터 생성 (100개 행, 5개 열)
2. 숫자 데이터 통계 분석 (나이, 점수)
3. 범주형 데이터 빈도 분석 (이름, 등급, 도시)
4. 시각화 파일 생성
5. 표본 추출 및 열 추출 예시

## 🔧 개발자 정보

- **언어**: Rust (Edition 2024)
- **버전**: 0.1.0
- **라이선스**: MIT (추후 추가 예정)

## 🤝 기여하기

1. 이 저장소를 포크합니다
2. 새로운 기능 브랜치를 생성합니다 (`git checkout -b feature/새기능`)
3. 변경사항을 커밋합니다 (`git commit -am '새 기능 추가'`)
4. 브랜치에 푸시합니다 (`git push origin feature/새기능`)
5. Pull Request를 생성합니다

## 📞 지원

문제가 발생하거나 질문이 있으시면 이슈를 생성해 주세요.

---

**데이터 분석기**로 효율적인 데이터 분석을 시작해보세요! 🚀