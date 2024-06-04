use std::sync::{Arc};
use std::sync::atomic::AtomicU32;
use tokio::sync::Mutex;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let arc_db = Arc::new(Mutex::new(db));
    let arc_db2 = arc_db.clone();
    let arc_db3 = arc_db.clone();

    let task_a = tokio::task::spawn(async move {
        let mut db = arc_db.lock().await;
        db[4] = 50;
        assert_eq!(db[4], 50);
    });
    let task_b = tokio::task::spawn(async move {
        let mut db = arc_db2.lock().await;
        db[4] = 100;
        assert_eq!(db[4], 100);
    });

    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();

    print!("{:?}", arc_db3.lock().await);


    let lock = RwLock::new(5);
    {
        let r1 = lock.read().await;
        let r2 = lock.read().await;
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    {
        let mut w = lock.write().await;
        *w += 1;
        assert_eq!(*w, 6);
    }


    // let atomic_forty_two = AtomicU32::new(42);
    // let arc_data = Arc::new(atomic_forty_two);

    let mut some_var = AtomicU32::new(10);
    *some_var.get_mut() = 5;
    assert_eq!(*some_var.get_mut(), 5);
}