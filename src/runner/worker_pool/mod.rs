use list::WorkerList;
use std::{
    net::{SocketAddr, TcpStream},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc,
    },
};
use worker::Worker;

mod list;
mod worker;

pub(super) struct WorkerPool {
    /// The list of active workers
    workers: WorkerList,

    /// The minimum number of spare workers
    min_spare_workers: usize,

    /// The maximum number of spare workers
    max_spare_workers: usize,

    /// The current count of the number of spare worker threads
    ///
    /// This is used by worker threads to determine if they should kill themselves or join the
    /// spare queue
    spare_worker_count: Arc<AtomicUsize>,

    /// The queue of workers ready to accept a connection
    spare_worker_queue: Receiver<usize>,

    /// A sender for the spare worker queue
    spare_worker_queue_sender: SyncSender<usize>,

    /// The queue of workers that died and can be cleared from the list
    dead_worker_queue: Receiver<usize>,

    /// A sender for the dead worker queue
    dead_worker_queue_sender: SyncSender<usize>,
}

impl WorkerPool {
    /// Creates a new [`WorkerPool`] with `initial_workers` workers to begin
    pub(super) fn new(
        max_workers: usize,
        initial_workers: usize,
        min_spare_workers: usize,
        max_spare_workers: usize,
    ) -> Self {
        // Create shared state
        let spare_worker_count = Arc::new(AtomicUsize::new(initial_workers));
        let (spare_worker_queue_sender, spare_worker_queue) = sync_channel(max_workers);
        let (dead_worker_queue_sender, dead_worker_queue) = sync_channel(max_workers);

        // Spawn initial workers
        let mut workers = WorkerList::new(max_workers);
        for _ in 0..initial_workers {
            let worker_id = workers
                .spawn(
                    max_spare_workers,
                    spare_worker_count.clone(),
                    spare_worker_queue_sender.clone(),
                    dead_worker_queue_sender.clone(),
                )
                .unwrap();
            spare_worker_queue_sender.send(worker_id).unwrap();
        }

        WorkerPool {
            workers,
            min_spare_workers,
            max_spare_workers,
            spare_worker_count,
            spare_worker_queue,
            spare_worker_queue_sender,
            dead_worker_queue,
            dead_worker_queue_sender,
        }
    }

    /// Send `connection` to a worker to be handled
    ///
    /// Returns `true` if there is a thread to handle the connection
    pub(super) fn accept(&mut self, connection: (TcpStream, SocketAddr)) -> bool {
        let result = self.do_accept(connection);

        self.clean_dead_workers();
        self.spawn_minimum_workers();

        result
    }

    /// Actually sends the `connection` to or spawns a worker thread to handle to the connection
    fn do_accept(&mut self, connection: (TcpStream, SocketAddr)) -> bool {
        // Try to send to an already spawned worker
        match self.spare_worker_queue.try_recv() {
            Ok(worker) => {
                self.spare_worker_count.fetch_sub(1, Ordering::Release);
                self.workers.get(worker).send_connection(connection);
                return true;
            }
            Err(_) => {}
        }

        // Try to spawn a worker
        match self.workers.spawn(
            self.max_spare_workers,
            self.spare_worker_count.clone(),
            self.spare_worker_queue_sender.clone(),
            self.dead_worker_queue_sender.clone(),
        ) {
            Some(worker) => {
                self.workers.get(worker).send_connection(connection);
                return true;
            }
            None => {}
        }

        // Try clearing a dead worker then spawning one
        if self.clean_dead_worker() {
            match self.workers.spawn(
                self.max_spare_workers,
                self.spare_worker_count.clone(),
                self.spare_worker_queue_sender.clone(),
                self.dead_worker_queue_sender.clone(),
            ) {
                Some(worker) => {
                    self.workers.get(worker).send_connection(connection);
                    return true;
                }
                None => {}
            }
        }

        false
    }

    /// Clears any dead worker threads from the list
    fn clean_dead_workers(&mut self) {
        while self.clean_dead_worker() {}
    }

    /// Clears one dead worker from the queue, returning true if there is one
    fn clean_dead_worker(&mut self) -> bool {
        match self.dead_worker_queue.try_recv() {
            Ok(worker) => {
                self.workers.free(worker);
                true
            }
            Err(_) => false,
        }
    }

    /// Spawns more workers if the minimum hasn't been reached
    fn spawn_minimum_workers(&mut self) {
        while self.spare_worker_count.load(Ordering::Acquire) < self.min_spare_workers {
            self.spare_worker_count.fetch_add(1, Ordering::Release);
            let worker_id = self
                .workers
                .spawn(
                    self.max_spare_workers,
                    self.spare_worker_count.clone(),
                    self.spare_worker_queue_sender.clone(),
                    self.dead_worker_queue_sender.clone(),
                )
                .unwrap();
            self.spare_worker_queue_sender.send(worker_id).unwrap();
        }
    }
}
