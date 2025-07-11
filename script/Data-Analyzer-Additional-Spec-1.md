# 데이터 분석기

## 현재 스펙

1. 앱을 실행시킴
2. analyze를 입력할 경우 파일경로도 바로 입력 받아야함.
3. 입력받은 파일경로를 바탕으로 데이터를 스캔하여 기초 분석 진행

## 추가 스펙

앱을 실행시킨 이후 analyze 커맨드 입력시 다음과 같은 Flow로 진행

1. 파일 입력받기
2. 원하는 명령 선택
   - 기초 통계량 표시 (src/analyze/descriptive_statistics.rs)
   - 빈도 표시 (src/analyze/frequency_distribution.rs)
   - 그래프 그리기 (데이터를 직접 선택 가능해야 하며, 헤더(1열)를 추출하여 리스트 형식으로 보여주고 여러개를 골라 한번에 가능하게도 필요함) (src/analyze/draw_graph.rs)
     - 막대 그래프 (src/analyze/graph/bar_chart.rs)
     - 원 그래프 (src/analyze/graph/pi_chart.rs)
     - 히스토그램 (src/analyze/graph/histogram.rs)
     - 상자 그림 (src/analyze/graph/box_plot.rs)
     - 줄기와 잎 그림 (src/analyze/graph/stem_and_leaf.rs)
     - 점도표 (src/analyze/graph/dot_plot.rs)
   - t 검정 (데이터를 직접 선택 가능해야 함) (src/analyze/t_test.rs)
     - 일표본 t 검정 (src/analyze/ttest/one_sample_t_test.rs)
     - 독립표본 t 검정 (src/analyze/ttest/independent_samples_t_test.rs)
     - 대응표본 t 검정 (src/analyze/ttest/paired_samples_t_test.rs)

