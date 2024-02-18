use std::fs::File;
use std::io::Read;

use glommio::{executor, Latency, LocalExecutorBuilder, Placement, Shares};
use memmap2::Mmap;

fn main() {
    LocalExecutorBuilder::new(Placement::Fixed(0))
        .spawn(|| async move {
            let tq1 =
                executor().create_task_queue(Shares::Static(2), Latency::NotImportant, "test1");
            let tq2 =
                executor().create_task_queue(Shares::Static(1), Latency::NotImportant, "test2");
            let t1 = glommio::spawn_local_into(async move { read() }, tq1).unwrap();
            let t2 = glommio::spawn_local_into(async move { read() }, tq2).unwrap();

            t1.await;
            t2.await;
        })
        .unwrap();
}

fn read() -> Result<()> {
    let mut file = File::open("LICENSE")?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let mmap = unsafe { Mmap::map(&file)? };

    assert_eq!(&contents[..], &mmap[..]);
}
