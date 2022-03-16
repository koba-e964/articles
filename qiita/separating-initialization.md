## 内容

最近業務で扱っている以下のようなRustのコードをリファクタしました。(デフォルメしたので実際のコードとは結構違います)

```rust:executor.rs
// 様々な機能を実行するための構造体。「リペア機能」という機能も扱うが詳細は省く。
struct Executor {
    ...
    // リペア機能で使うメトリクス。Executorの起動時に初期化して、リペア機能の起動時にクローンを渡す。
    repair_metric_0: Counter,
    repair_metric_1: Counter,
    repair_metric_2: Gauge,
}

impl Executor {
    fn new(metric_builder: &MetricBuilder) -> Self {
        Executor {
            ...
            repair_metric_0: metric_builder.make_counter(),
            repair_metric_1: metric_builder.make_counter(),
            repair_metric_2: metric_builder.make_gauge(),
        }
    }
    fn repair(&self, object_id: ObjectId) {
        let repair_exec = RepairExecutor::new(&self, object_id);
        repair_exec.perform();
    }
}

// オブジェクトのリペア機能を扱う。詳細は省く。
struct RepairExecutor {
    repair_metric_0: Counter,
    repair_metric_1: Counter,
    repair_metric_2: Gauge,
    object_id: ObjectId,
}
impl RepairExecutor {
    fn new(exec: &Executor, object_id: ObjectId) -> Self {
        // Executorにあるリペア用のメトリクスを全部クローンする
        RepairExecutor {
            repair_metric_0: exec.repair_metric_0.clone(),
            repair_metric_1: exec.repair_metric_1.clone(),
            repair_metric_2: exec.repair_metric_2.clone(),
            object_id,
        }
    }
    fn perform(&self) { ... }
}
```
このコードは、以下の点でよくないです:
1. リペア機能をexecutor.rsで扱っており、executor.rsの責務が「実行」と「リペア機能の実行」の2種類になっているが、リペア機能の実行は別モジュールに分けるべきである。
2. 1.と関連して、リペア用のメトリクスをExecutor::newで初期化しているが、リペア機能用のメトリクスはExecutorの担当外なのでリペア機能側にやらせるべきである。
3. RepairExecutorの初期化時にExecutor構造体内部のデータに触っているが、これは密結合なのでやめた方が良い。

以上の点を直すと以下のようになります。

```rust:executor.rs
use repair::{RepairMetrics, RepairExecutor};

struct Executor {
    ...
    // リペア機能で使うメトリクス。Executorの起動時に初期化して、リペア機能の起動時にクローンを渡す。
    repair_metrics: RepairMetrics,
}

impl Executor {
    fn new(metric_builder: &MetricBuilder) -> Self {
        Executor {
            ...
            repair_metrics: RepairMetrics::new(metric_builder),
        }
    }
    fn repair(&self, object_id: ObjectId) {
        let repair_exec = RepairExecutor::new(
            self.repair_metrics.clone(),
            object_id,
        );
        repair_exec.perform();
    }
}

```

```rust:repair.rs
#[derive(Clone)]
struct RepairMetrics {
    repair_metric_0: Counter,
    repair_metric_1: Counter,
    repair_metric_2: Gauge,
}

impl RepairMetrics {
    fn new(metric_builder: &MetricBuilder) -> Self {
        RepairMetrics {
            repair_metric_0: metric_builder.make_counter(),
            repair_metric_1: metric_builder.make_counter(),
            repair_metric_2: metric_builder.make_gauge(),
        }
    }
}

struct RepairExecutor {
    repair_metrics: RepairMetrics,
    object_id: ObjectId,
}
impl RepairExecutor {
    fn new(repair_metrics: RepairMetrics, object_id: ObjectId) -> Self {
        RepairExecutor {
            repair_metrics: repair_metrics,
            object_id,
        }
    }
    fn perform(&self) { ... }
}
```

## 感想
こういう、初期化処理を親モジュールの初期化時にしないといけないけど、ベタ書きすると煩雑になってしまう時に、別モジュールに初期化処理を生やして親ではそれを呼ぶだけ、みたいなパターンは、どこかでまとめられているような気がするのですが、調べても名前がわかりませんでした。ご存知の方は教えてください。(「関心事の分離」の一形態ではあると思いますが)
少なくとも[GoFのデザインパターン](https://ja.wikipedia.org/wiki/%E3%83%87%E3%82%B6%E3%82%A4%E3%83%B3%E3%83%91%E3%82%BF%E3%83%BC%E3%83%B3_(%E3%82%BD%E3%83%95%E3%83%88%E3%82%A6%E3%82%A7%E3%82%A2))にはなかったと思います。

## コード
変更部分: https://github.com/frugalos/frugalos/compare/3a33cc9f5b1cc32eb4f2ee72093a5b89ffdece11...0b68455ed15039aaf6fcc7cf47edf5c262b2016c
