use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;
use std::thread;
use tensorflow::Graph;
use tensorflow::Operation;
use tensorflow::SavedModelBundle;
use tensorflow::Session;
use tensorflow::SessionOptions;
use tensorflow::SessionRunArgs;
use tensorflow::Tensor;

use crate::preprocess;

pub struct Classify {
    session: Session,
    op_in: Operation,
    op_out: Operation,
    is_black_gold: Arc<AtomicBool>,
}

impl Classify {
    pub fn new() -> anyhow::Result<Self> {
        let mut graph = Graph::new();

        let session = SavedModelBundle::load(
            &SessionOptions::new(),
            ["serve"],
            &mut graph,
            "model/trained",
        )?
        .session;

        let op_in = graph.operation_by_name_required("serving_default_input_2")?;
        let op_out = graph.operation_by_name_required("StatefulPartitionedCall")?;

        Ok(Self {
            session,
            op_in,
            op_out,
            is_black_gold: Arc::new(AtomicBool::new(false)),
        })
    }

    fn classify(&self, buf: Vec<u8>) -> anyhow::Result<()> {
        let data: Vec<_> = buf.iter().map(|x| *x as f32).collect();
        let input = Tensor::<f32>::new(&[1, 320, 180, 3]).with_values(&data)?;

        let mut predict = SessionRunArgs::new();
        predict.add_feed(&self.op_in, 0, &input);
        let output = predict.request_fetch(&self.op_out, 0);
        self.session.run(&mut predict)?;

        let res: f32 = predict.fetch(output)?[0];
        tracing::info!("res {}", res);
        if res < 0.3 {
            self.is_black_gold.store(true, Ordering::Relaxed);
        } else {
            self.is_black_gold.store(false, Ordering::Relaxed);
        }

        Ok(())
    }

    pub fn start(self) -> (SyncSender<Vec<u8>>, Arc<AtomicBool>) {
        let is_black_gold2 = self.is_black_gold.clone();
        let (tx, rx) = std::sync::mpsc::sync_channel::<Vec<u8>>(1);
        thread::spawn(move || {
            while let Ok(buf) = rx.recv() {
                let buf = preprocess::resize(buf);
                self.classify(buf).expect("classify");
            }
        });

        (tx, is_black_gold2)
    }
}
