// Khai báo các crate chuẩn cần thiết cho hệ thống song song và bất đồng bộ
use std::{
    future::{Future, Pending, PollFn, Ready},
    pin::Pin,
    task::{Context, Poll, Waker},
    sync::{
        mpsc::{self, channel, Sender, Receiver},
        Arc, Mutex, RwLock,
        atomic::{AtomicBool, Ordering::{self, Relaxed}, AtomicUsize},
    },
    thread::{self, JoinHandle, sleep, spawn, yield_now, current},
    time::{self, Duration},
    collections::VecDeque,
};

// Định nghĩa các loại công việc (Work) có thể xảy ra trong hệ thống
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Work {
    Authentication, // Xác thực người dùng
    Support, // Hỗ trợ khách hàng
    Compensation, // Bồi thường cho khách hàng
    PayDebt, // Thanh toán tiền nợ
    PayCart, // Thanh toán giỏ hàng
    PlaceOrder, // Đặt hàng
}

// Định nghĩa các loại sự kiện (Event) có thể xảy ra trong hệ thống
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Event {
    NewJob, // Có một công việc mới được thêm vào
    JobDone, // Có một công việc đã hoàn thành
    JobFailed, // Có một công việc bị lỗi
    SystemShutdown, // Hệ thống cần được tắt
}

// Định nghĩa các loại hành động (Action) có thể được thực hiện khi một nhiệm vụ (Task) hoàn thành
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    UpdateState, // Cập nhật trạng thái của phòng (Room)
    NotifyManager, // Thông báo cho quản lý (Manager)
    NotifyCustomer, // Thông báo cho khách hàng
    RetryJob, // Thử lại công việc (Job)
}

// Định nghĩa một cấu trúc dữ liệu để biểu diễn một công việc cụ thể (Job)
#[derive(Debug, Clone)]
struct Job {
    work: Work, // Loại công việc
    data: String, // Dữ liệu cần thiết cho công việc
}

// Định nghĩa một cấu trúc dữ liệu để biểu diễn một nhiệm vụ cụ thể (Task)
#[derive(Debug, Clone)]
struct Task {
    job: Job, // Công việc liên quan
    method: fn(Job) -> Result<String, String>, // Phương thức để xử lý công việc
    action: Action, // Hành động khi nhiệm vụ hoàn thành
}

// Định nghĩa một cấu trúc dữ liệu để biểu diễn trạng thái của một phòng (Room)
#[derive(Debug, Clone)]
struct State {
    job: Option<Job>, // Công việc đang được xử lý trong phòng
    result: Option<String>, // Kết quả của công việc
    error: Option<String>, // Lỗi của công việc
}

// Định nghĩa một cấu trúc dữ liệu để biểu diễn một phòng (Room)
#[derive(Debug)]
struct Room {
    id: usize, // Mã định danh của phòng
    state: RwLock<State>, // Trạng thái của phòng, được bảo vệ bởi RwLock
}

// Định nghĩa một cấu trúc dữ liệu để biểu diễn một người làm việc (Worker)
#[derive(Debug)]
struct Worker {
    id: usize, // Mã định danh của người làm việc
    queue: Arc<Mutex<Receiver<Job>>>, // Hàng đợi chung để lấy công việc
    rooms: Arc<Vec<Room>>, // Danh sách các phòng để xử lý công việc
    handle: Option<JoinHandle<()>>, // Handle để quản lý luồng của người làm việc
}

// Định nghĩa một cấu trúc dữ liệu để biểu diễn một quản lý (Manager)
#[derive(Debug)]
struct Manager {
    queue: Arc<Mutex<Sender<Job>>>, // Hàng đợi chung để gửi công việc
    workers: Arc<Vec<Worker>>, // Danh sách các người làm việc
    events: Arc<Mutex<Receiver<Event>>>, // Hàng đợi chung để nhận sự kiện
    handlers: HashMap<Event, fn(Event) -> ()>, // Các hàm xử lý cho từng loại sự kiện
    shutdown: Arc<AtomicBool>, // Biến cờ để kiểm tra xem hệ thống có cần tắt không
}

// Định nghĩa một cấu trúc dữ liệu để biểu diễn một hệ thống (System)
#[derive(Debug)]
struct System {
    manager: Arc<Manager>, // Quản lý của hệ thống
    rooms: Arc<Vec<Room>>, // Danh sách các phòng của hệ thống
}

// Định nghĩa các phương thức để khởi tạo hệ thống
impl System {
    // Phương thức để tạo một hệ thống mới với số lượng phòng và người làm việc cho trước
    fn new(num_rooms: usize, num_workers: usize) -> Self {
        // Tạo một hàng đợi chung để gửi và nhận công việc
        let (job_sender, job_receiver) = channel();
        // Tạo một hàng đợi chung để gửi và nhận sự kiện
        let (event_sender, event_receiver) = channel();
        // Tạo một biến cờ để kiểm tra xem hệ thống có cần tắt không
        let shutdown = Arc::new(AtomicBool::new(false));
        // Tạo một danh sách các phòng với trạng thái ban đầu
        let rooms = Arc::new(
            (0..num_rooms)
                .map(|id| Room {
                    id,
                    state: RwLock::new(State {
                        job: None,
                        result: None,
                        error: None,
                    }),
                })
                .collect(),
        );
        // Tạo một danh sách các người làm việc và khởi tạo chúng
        let workers = Arc::new(
            (0..num_workers)
                .map(|id| {
                    Worker::new(
                        id,
                        Arc::clone(&job_receiver),
                        Arc::clone(&rooms),
                        Arc::clone(&event_sender),
                        Arc::clone(&shutdown),
                    )
                })
                .collect(),
        );
        // Tạo một quản lý và khởi tạo nó
        let manager = Arc::new(Manager::new(
            Arc::clone(&job_sender),
            Arc::clone(&workers),
            Arc::clone(&event_receiver),
            Arc::clone(&shutdown),
        ));
        // Trả về một hệ thống mới
        Self { manager, rooms }
    }

    // Phương thức để thêm một công việc mới vào hệ thống
    fn add_job(&self, job: Job) {
        // Gửi công việc vào hàng đợi chung
        self.manager.queue.lock().unwrap().send(job).unwrap();
        // Gửi một sự kiện mới vào hàng đợi chung
        self.manager
            .events
            .lock()
            .unwrap()
            .send(Event::NewJob)
            .unwrap();
    }

    // Phương thức để tắt hệ thống
    fn shutdown(&self) {
        // Đặt biến cờ shutdown thành true
        self.manager.shutdown.store(true, Ordering::Relaxed);
        // Gửi một sự kiện tắt hệ thống vào hàng đợi chung
        self.manager
            .events
            .lock()
            .unwrap()
            .send(Event::SystemShutdown)
            .unwrap();
        // Đợi cho tất cả các người làm việc hoàn tất công việc của họ
        for worker in self.manager.workers.iter() {
            worker.join().unwrap();
        }
    }
}

// Định nghĩa các phương thức để khởi tạo và xử lý công việc của người làm việc
impl Worker {
    // Phương thức để tạo một người làm việc mới
    fn new(
        id: usize,
        queue: Arc<Mutex<Receiver<Job>>>,
        rooms: Arc<Vec<Room>>,
        events: Arc<Mutex<Sender<Event>>>,
        shutdown: Arc<AtomicBool>,
    ) -> Self {
        // Tạo một người làm việc mới
        let worker = Self {
            id,
            queue,
            rooms,
            handle: None,
        };
        // Khởi tạo luồng của người làm việc
        worker.init(events, shutdown);
        // Trả về người làm việc mới
        worker
    }

    // Phương thức để khởi tạo luồng của người làm việc
    fn init(&self, events: Arc<Mutex<Sender<Event>>>, shutdown: Arc<AtomicBool>) {
        // Tạo một bản sao của người làm việc
        let worker = self.clone();
        // Tạo một handle để quản lý luồng
        let handle = spawn(move || {
            // Lặp lại cho đến khi hệ thống cần tắt
            while !shutdown.load(Ordering::Relaxed) {
                // Lấy một công việc từ hàng đợi chung
                if let Ok(job) = worker.queue.lock().unwrap().recv() {
                    // Chọn một phòng trống để xử lý công việc
                    if let Some(room) = worker.choose_room() {
                        // Xử lý công việc trong phòng
                        worker.process_job(job, room, &events);
                    } else {
                        // Nếu không có phòng trống, gửi một sự kiện báo lỗi
                        events
                            .lock()
                            .unwrap()
                            .send(Event::JobFailed)
                            .unwrap();
                    }
                }
            }
        });
        // Gán handle cho người làm việc
        self.handle = Some(handle);
    }

    // Phương thức để chọn một phòng trống để xử lý công việc
    fn choose_room(&self) -> Option<Arc<Room>> {
        // Duyệt qua danh sách các phòng
        for room in self.rooms.iter() {
            // Kiểm tra xem phòng có trống không
            if room.state.read().unwrap().job.is_none() {
                // Nếu có, trả về một Arc trỏ đến phòng đó
                return Some(Arc::clone(room));
            }
        }
        // Nếu không có phòng trống, trả về None
        None
    }

    // Phương thức để xử lý một công việc trong một phòng
    fn process_job(&self, job: Job, room: Arc<Room>, events: &Arc<Mutex<Sender<Event>>>) {
        // Tạo một danh sách các nhiệm vụ cần thiết để xử lý công việc
        let tasks = self.create_tasks(job);
        // Duyệt qua danh sách các nhiệm vụ
        for task in tasks {
            // Thực hiện nhiệm vụ trong phòng
            self.execute_task(task, &room, events);
        }
    }

    // Phương thức để tạo một danh sách các nhiệm vụ cần thiết để xử lý một công việc
fn create_tasks(&self, job: Job) -> Vec<Task> {
    // Tùy vào loại công việc, tạo các nhiệm vụ tương ứng
    match job.work {
        Work::Authentication => vec![
            Task {
                job,
                method: Self::verify_user, // Phương thức để xác thực người dùng
                action: Action::UpdateState, // Hành động khi nhiệm vụ hoàn thành
            },
            Task {
                job,
                method: Self::generate_token, // Phương thức để tạo mã token cho người dùng
                action: Action::NotifyCustomer, // Hành động khi nhiệm vụ hoàn thành
            },
        ],
        Work::Support => vec![
            Task {
                job,
                method: Self::read_message, // Phương thức để đọc tin nhắn của khách hàng
                action: Action::UpdateState, // Hành động khi nhiệm vụ hoàn thành
            },
            Task {
                job,
                method: Self::reply_message, // Phương thức để trả lời tin nhắn của khách hàng
                action: Action::NotifyCustomer, // Hành động khi nhiệm vụ hoàn thành
            },
        ],
        Work::Compensation => vec![
            Task {
                job,
                method: Self::calculate_amount, // Phương thức để tính số tiền bồi thường
                action: Action::UpdateState, // Hành động khi nhiệm vụ hoàn thành
            },
            Task {
                job,
                method: Self::transfer_amount, // Phương thức để chuyển tiền bồi thường cho khách hàng
                action: Action::NotifyCustomer, // Hành động khi nhiệm vụ hoàn thành
            },
        ],
        Work::PayDebt => vec![
            Task {
                job,
                method: Self::check_balance, // Phương thức để kiểm tra số dư của khách hàng
                action: Action::UpdateState, // Hành động khi nhiệm vụ hoàn thành
            },
            Task {
                job,
                method: Self::deduct_amount, // Phương thức để trừ số tiền nợ từ số dư của khách hàng
                action: Action::NotifyCustomer, // Hành động khi nhiệm vụ hoàn thành
            },
        ],
        Work::PayCart => vec![
            Task {
                job,
                method: Self::check_items, // Phương thức để kiểm tra các mặt hàng trong giỏ hàng
                action: Action::UpdateState, // Hành động khi nhiệm vụ hoàn thành
            },
            Task {
                job,
                method: Self::charge_amount, // Phương thức để thanh toán số tiền cho các mặt hàng
                action: Action::NotifyCustomer, // Hành động khi nhiệm vụ hoàn thành
            },
        ],
        Work::PlaceOrder => vec![
            Task {
                job,
                method: Self::confirm_order, // Phương thức để xác nhận đơn hàng
                action: Action::UpdateState, // Hành động khi nhiệm vụ hoàn thành
            },
            Task {
                job,
                method: Self::send_order, // Phương thức để gửi đơn hàng cho nhà cung cấp
                action: Action::NotifyCustomer, // Hành động khi nhiệm vụ hoàn thành
            },
        ],
    }
}

// Phương thức để thực hiện một nhiệm vụ trong một phòng
fn execute_task(&self, task: Task, room: &Arc<Room>, events: &Arc<Mutex<Sender<Event>>>) {
    // Gọi phương thức của nhiệm vụ và lấy kết quả
    let result = (task.method)(task.job);
    // Tùy vào hành động của nhiệm vụ, thực hiện các hành động tương ứng
    match task.action {
        Action::UpdateState => {
            // Cập nhật trạng thái của phòng theo kết quả
            room.state.write().unwrap().update(result);
        }
        Action::NotifyManager => {
            // Gửi một sự kiện cho quản lý theo kết quả
            events
                .lock()
                .unwrap()
                .send(result.to_event())
                .unwrap();
        }
        Action::NotifyCustomer => {
            // Gửi một thông báo cho khách hàng theo kết quả
            self.send_notification(result);
        }
        Action::RetryJob => {
            // Nếu có lỗi, thử lại công việc
            if let Err(error) = result {
                self.retry_job(task.job, error, events);
            }
        }
    }
}

// Phương thức để cập nhật trạng thái của phòng theo kết quả
impl State {
    fn update(&mut self, result: Result<String, String>) {
        // Nếu kết quả là Ok, cập nhật kết quả và xóa lỗi
        if let Ok(value) = result {
            self.result = Some(value);
            self.error = None;
        }
        // Nếu kết quả là Err, cập nhật lỗi và xóa kết quả
        if let Err(error) = result {
            self.error = Some(error);
            self.result = None;
        }
    }
}

// Phương thức để chuyển đổi kết quả thành sự kiện
impl Result<String, String> {
    fn to_event(self) -> Event {
        // Nếu kết quả là Ok, trả về sự kiện JobDone
        if self.is_ok() {
            Event::JobDone
        }
        // Nếu kết quả là Err, trả về sự kiện JobFailed
        else {
            Event::JobFailed
        }
    }
}

// Phương thức để gửi một thông báo cho khách hàng theo kết quả
impl Worker {
    fn send_notification(&self, result: Result<String, String>) {
        // Nếu kết quả là Ok, gửi một thông báo thành công
        if let Ok(message) = result {
            println!("Worker {} sent a success message: {}", self.id, message);
        }
        // Nếu kết quả là Err, gửi một thông báo lỗi
        if let Err(message) = result {
            println!("Worker {} sent an error message: {}", self.id, message);
        }
    }
}

// Phương thức để thử lại một công việc nếu có lỗi
impl Worker {
    fn retry_job(&self, job: Job, error: String, events: &Arc<Mutex<Sender<Event>>>) {
        // In ra lỗi và thông báo thử lại
        println!("Worker {} encountered an error: {}", self.id, error);
        println!("Worker {} is retrying the job: {:?}", self.id, job);
        // Tạo một danh sách các nhiệm vụ cần thiết để xử lý công việc
        let tasks = self.create_tasks(job);
        // Tìm một phòng trống để xử lý công việc
        if let Some(room) = self.choose_room() {
            // Duyệt qua danh sách các nhiệm vụ
            for task in tasks {
                // Thực hiện nhiệm vụ trong phòng
                self.execute_task(task, &room, events);
            }
        } else {
            // Nếu không có phòng trống, gửi một sự kiện báo lỗi
            events
                .lock()
                .unwrap()
                .send(Event::JobFailed)
                .unwrap();
        }
    }
}

// Định nghĩa các phương thức để khởi tạo và xử lý sự kiện của quản lý
impl Manager {
    // Phương thức để tạo một quản lý mới
    fn new(
        queue: Arc<Mutex<Sender<Job>>>,
        workers: Arc<Vec<Worker>>,
        events: Arc<Mutex<Receiver<Event>>>,
        shutdown: Arc<AtomicBool>,
    ) -> Self {
        // Tạo một quản lý mới
        let manager = Self {
            queue,
            workers,
            events,
            handlers: HashMap::new(),
            shutdown,
        };
        // Đăng ký các hàm xử lý cho từng loại sự kiện
        manager.register_handler(Event::NewJob, Self::handle_new_job);
        manager.register_handler(Event::JobDone, Self::handle_job_done);
        manager.register_handler(Event::JobFailed, Self::handle_job_failed);
        manager.register_handler(Event::SystemShutdown, Self::handle_system_shutdown);
        // Khởi tạo luồng của quản lý
        manager.init();
        // Trả về quản lý mới
        manager
    }

    // Phương thức để đăng ký một hàm xử lý cho một loại sự kiện
    fn register_handler(&mut self, event: Event, handler: fn(Event) -> ()) {
        // Thêm hàm xử lý vào bản đồ handlers
        self.handlers.insert(event, handler);
    }

    // Phương thức để khởi tạo luồng của quản lý
    fn init(&self) {
        // Tạo một bản sao của quản lý
        let manager = self.clone();
        // Tạo một handle để quản lý luồng
        let handle = spawn(move || {
            // Lặp lại cho đến khi hệ thống cần tắt
            while !manager.shutdown.load(Ordering::Relaxed) {
                // Nhận một sự kiện từ hàng đợi chung
                if let Ok(event) = manager.events.lock().unwrap().recv() {
                    // Tìm hàm xử lý tương ứng với sự kiện
                    if let Some(handler) = manager.handlers.get(&event) {
                        // Gọi hàm xử lý với sự kiện
                        handler(event);
                    }
                }
            }
        });
        // Gán handle cho quản lý
        self.handle = Some(handle);
    }

    // Phương thức để xử lý sự kiện NewJob
    fn handle_new_job(event: Event) {
        // In ra thông báo có một công việc mới được thêm vào
        println!("Manager received a new job event: {:?}", event);
    }

    // Phương thức để xử lý sự kiện JobDone
    fn handle_job_done(event: Event) {
        // In ra thông báo có một công việc đã hoàn thành
        println!("Manager received a job done event: {:?}", event);
    }

    // Phương thức để xử lý sự kiện JobFailed
    fn handle_job_failed(event: Event) {
        // In ra thông báo có một công việc bị lỗi
        println!("Manager received a job failed event: {:?}", event);
    }

    // Phương thức để xử lý sự kiện SystemShutdown
    fn handle_system_shutdown(event: Event) {
        // In ra thông báo hệ thống cần được tắt
        println!("Manager received a system shutdown event: {:?}", event);
    }
}

// Định nghĩa các phương thức cụ thể để xử lý các loại công việc
impl Worker {
    // Phương thức để xác thực người dùng
    fn verify_user(job: Job) -> Result<String, String> {
        // Giả sử dữ liệu của công việc là tên người dùng
        let username = job.data;
        // Kiểm tra xem tên người dùng có hợp lệ không
        if username.is_empty() {
            // Nếu không, trả về một lỗi
            Err("Invalid username".to_string())
        } else {
            // Nếu có, trả về một thông báo thành công
            Ok(format!("User {} verified", username))
        }
    }

    // Phương thức để tạo mã token cho người dùng
    fn generate_token(job: Job) -> Result<String, String> {
        // Giả sử dữ liệu của công việc là tên người dùng
        let username = job.data;
        // Tạo một mã token ngẫu nhiên
        let token = format!("{:x}", rand::random::<u64>());
        // Trả về một thông báo thành công với mã token
        Ok(format!("User {} received token {}", username, token))
    }

    // Phương thức để đọc tin nhắn của khách hàng
    fn read_message(job: Job) -> Result<String, String> {
        // Giả sử dữ liệu của công việc là nội dung tin nhắn
        let message = job.data;
        // Trả về một thông báo thành công với nội dung tin nhắn
        Ok(format!("Read message: {}", message))
    }

    // Phương thức để trả lời tin nhắn của khách hàng
    fn reply_message(job: Job) -> Result<String, String> {
        // Giả sử dữ liệu của công việc là nội dung tin nhắn
        let message = job.data;
        // Tạo một nội dung trả lời ngẫu nhiên
        let reply = format!("Thank you for your message: {}", message);
        // Trả về một thông báo thành công với nội dung trả lời
        Ok(format!("Sent reply: {}", reply))
    }

    // Phương thức để tính số tiền bồi thường
    fn calculate_amount(job: Job) -> Result<String, String> {
        // Giả sử dữ liệu của công việc là số tiền bị mất
        let amount = job.data.parse::<f64>().unwrap_or(0.0);
        // Tính số tiền bồi thường bằng cách nhân với một tỷ lệ ngẫu nhiên
        let rate = rand::random::<f64>() * 0.1 + 0.9;
        let compensation = amount * rate;
        // Trả về một thông báo thành công với số tiền bồi thường
        Ok(format!("Calculated compensation: {:.2}", compensation))
    }

    // Phương thức để chuyển tiền bồi thường cho khách hàng
    fn transfer_amount(job: Job) -> Result<String, String> {
        // Giả sử dữ liệu của công việc là số tiền bồi thường
        let amount = job.data.parse::<f64>().unwrap_or(0.0);
        // Giả sử việc chuyển tiền có thể thành công hoặc thất bại ngẫu nhiên
        let success = rand::random::<bool>();
        if success {
            // Nếu thành công, trả về một thông báo thành công với số tiền đã chuyển
            Ok(format!("Transferred amount: {:.2}", amount))
        } else {
            // Nếu thất bại, trả về một lỗi với số tiền chưa chuyển
            Err(format!("Failed to transfer amount: {:.2}", amount))
        }
    }

    // Phương thức để kiểm tra số dư của khách hàng
    fn check_balance(job: Job) -> Result<String, String> {
        // Giả sử dữ liệu của công việc là số tài khoản của khách hàng
        let account = job.data;
        // Giả sử số dư của khách hàng là một số ngẫu nhiên
        let balance = rand::random::<f64>() * 1000.0;
        // Trả về một thông báo thành công với số dư của khách hàng
        Ok(format!("Account {} has balance: {:.2}", account, balance))
    }

// Phương thức để trừ số tiền nợ từ số dư của khách hàng
fn deduct_amount(job: Job) -> Result<String, String> {
    // Giả sử dữ liệu của công việc là số tiền nợ
    let amount = job.data.parse::<f64>().unwrap_or(0.0);
    // Giả sử số dư của khách hàng là một số ngẫu nhiên
    let balance = rand::random::<f64>() * 1000.0;
    // Kiểm tra xem số dư có đủ để trừ số tiền nợ không
    if balance >= amount {
        // Nếu có, trừ số tiền nợ từ số dư và trả về một thông báo thành công
        let new_balance = balance - amount;
        Ok(format!(
            "Deducted amount: {:.2}, new balance: {:.2}",
            amount, new_balance
        ))
    } else {
        // Nếu không, trả về một lỗi với số tiền nợ và số dư
        Err(format!(
            "Insufficient balance: {:.2}, debt amount: {:.2}",
            balance, amount
        ))
    }
}

// Phương thức để kiểm tra các mặt hàng trong giỏ hàng
fn check_items(job: Job) -> Result<String, String> {
    // Giả sử dữ liệu của công việc là danh sách các mặt hàng trong giỏ hàng
    let items = job.data.split(',').collect::<Vec<&str>>();
    // Tính tổng số lượng và giá trị của các mặt hàng
    let mut total_quantity = 0;
    let mut total_value = 0.0;
    for item in items {
        // Giả sử mỗi mặt hàng có định dạng là "tên:số lượng:giá"
        let parts = item.split(':').collect::<Vec<&str>>();
        // Lấy tên, số lượng, và giá của mỗi mặt hàng
        let name = parts[0];
        let quantity = parts[1].parse::<usize>().unwrap_or(0);
        let price = parts[2].parse::<f64>().unwrap_or(0.0);
        // Cộng dồn số lượng và giá trị vào tổng
        total_quantity += quantity;
        total_value += quantity as f64 * price;
        // In ra thông tin của mỗi mặt hàng
        println!("Item: {}, quantity: {}, price: {:.2}", name, quantity, price);
    }
    // Trả về một thông báo thành công với tổng số lượng và giá trị của các mặt hàng
    Ok(format!(
        "Total items: {}, total value: {:.2}",
        total_quantity, total_value
    ))
}

// Phương thức để thanh toán số tiền cho các mặt hàng
fn charge_amount(job: Job) -> Result<String, String> {
    // Giả sử dữ liệu của công việc là số tiền cần thanh toán
    let amount = job.data.parse::<f64>().unwrap_or(0.0);
    // Giả sử việc thanh toán có thể thành công hoặc thất bại ngẫu nhiên
    let success = rand::random::<bool>();
    if success {
        // Nếu thành công, trả về một thông báo thành công với số tiền đã thanh toán
        Ok(format!("Charged amount: {:.2}", amount))
    } else {
        // Nếu thất bại, trả về một lỗi với số tiền chưa thanh toán
        Err(format!("Failed to charge amount: {:.2}", amount))
    }
}

// Phương thức để xác nhận đơn hàng
fn confirm_order(job: Job) -> Result<String, String> {
    // Giả sử dữ liệu của công việc là mã đơn hàng
    let order_id = job.data;
    // Trả về một thông báo thành công với mã đơn hàng
    Ok(format!("Order {} confirmed", order_id))
}

// Phương thức để gửi đơn hàng cho nhà cung cấp
fn send_order(job: Job) -> Result<String, String> {
    // Giả sử dữ liệu của công việc là mã đơn hàng
    let order_id = job.data;
    // Giả sử việc gửi đơn hàng có thể thành công hoặc thất bại ngẫu nhiên
    let success = rand::random::<bool>();
    if success {
        // Nếu thành công, trả về một thông báo thành công với mã đơn hàng
        Ok(format!("Order {} sent to supplier", order_id))
    } else {
        // Nếu thất bại, trả về một lỗi với mã đơn hàng
        Err(format!("Failed to send order {}", order_id))
    }
}

// Định nghĩa các bài kiểm tra đơn vị và tích hợp để kiểm tra tính chính xác và hiệu suất của mã nguồn
#[cfg(test)]
mod tests {
    use super::*;

    // Bài kiểm tra đơn vị cho phương thức verify_user
    #[test]
    fn test_verify_user() {
        // Tạo một công việc xác thực người dùng
        let job = Job {
            work: Work::Authentication,
            data: "Alice".to_string(),
        };
        // Gọi phương thức verify_user và kiểm tra kết quả
        let result = Worker::verify_user(job);
        assert_eq!(result, Ok("User Alice verified".to_string()));
    }

    // Bài kiểm tra đơn vị cho phương thức generate_token
    #[test]
    fn test_generate_token() {
        // Tạo một công việc tạo mã token cho người dùng
        let job = Job {
            work: Work::Authentication,
            data: "Alice".to_string(),
        };
        // Gọi phương thức generate_token và kiểm tra kết quả
        let result = Worker::generate_token(job);
        // Kiểm tra xem kết quả có phải là một thông báo thành công với mã token không
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("User Alice received token "));
    }

    // Bài kiểm tra đơn vị cho phương thức read_message
    #[test]
    fn test_read_message() {
        // Tạo một công việc đọc tin nhắn của khách hàng
        let job = Job {
            work: Work::Support,
            data: "Hello, I need help".to_string(),
        };
        // Gọi phương thức read_message và kiểm tra kết quả
        let result = Worker::read_message(job);
        assert_eq!(result, Ok("Read message: Hello, I need help".to_string()));
    }

    // Bài kiểm tra đơn vị cho phương thức reply_message
    #[test]
    fn test_reply_message() {
        // Tạo một công việc trả lời tin nhắn của khách hàng
        let job = Job {
            work: Work::Support,
            data: "Hello, I need help".to_string(),
        };
        // Gọi phương thức reply_message và kiểm tra kết quả
        let result = Worker::reply_message(job);
        assert_eq!(
            result,
            Ok("Sent reply: Thank you for your message: Hello, I need help".to_string())
        );
    }

    // Bài kiểm tra đơn vị cho phương thức calculate_amount
    #[test]
    fn test_calculate_amount() {
        // Tạo một công việc tính số tiền bồi thường
        let job = Job {
            work: Work::Compensation,
            data: "100.0".to_string(),
        };
        // Gọi phương thức calculate_amount và kiểm tra kết quả
        let result = Worker::calculate_amount(job);
        // Kiểm tra xem kết quả có phải là một thông báo thành công với số tiền bồi thường không
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("Calculated compensation: "));
    }

    // Bài kiểm tra đơn vị cho phương thức transfer_amount
#[test]
fn test_transfer_amount() {
    // Tạo một công việc chuyển tiền bồi thường
    let job = Job {
        work: Work::Compensation,
        data: "100.0".to_string(),
    };
    // Gọi phương thức transfer_amount và kiểm tra kết quả
    let result = Worker::transfer_amount(job);
    // Kiểm tra xem kết quả có phải là một thông báo thành công hoặc một lỗi không
    assert!(result.is_ok() || result.is_err());
    // Nếu là một thông báo thành công, kiểm tra xem có chứa số tiền đã chuyển không
    if let Ok(message) = &result {
        assert!(message.contains("Transferred amount: "));
    }
    // Nếu là một lỗi, kiểm tra xem có chứa số tiền chưa chuyển không
    if let Err(message) = &result {
        assert!(message.contains("Failed to transfer amount: "));
    }
}

// Bài kiểm tra đơn vị cho phương thức check_balance
#[test]
fn test_check_balance() {
    // Tạo một công việc kiểm tra số dư của khách hàng
    let job = Job {
        work: Work::PayDebt,
        data: "123456".to_string(),
    };
    // Gọi phương thức check_balance và kiểm tra kết quả
    let result = Worker::check_balance(job);
    // Kiểm tra xem kết quả có phải là một thông báo thành công với số dư của khách hàng không
    assert!(result.is_ok());
    assert!(result.unwrap().starts_with("Account 123456 has balance: "));
}

// Bài kiểm tra đơn vị cho phương thức deduct_amount
#[test]
fn test_deduct_amount() {
    // Tạo một công việc trừ số tiền nợ từ số dư của khách hàng
    let job = Job {
        work: Work::PayDebt,
        data: "100.0".to_string(),
    };
    // Gọi phương thức deduct_amount và kiểm tra kết quả
    let result = Worker::deduct_amount(job);
    // Kiểm tra xem kết quả có phải là một thông báo thành công hoặc một lỗi không
    assert!(result.is_ok() || result.is_err());
    // Nếu là một thông báo thành công, kiểm tra xem có chứa số tiền đã trừ và số dư mới không
    if let Ok(message) = &result {
        assert!(message.contains("Deducted amount: "));
        assert!(message.contains("new balance: "));
    }
    // Nếu là một lỗi, kiểm tra xem có chứa số tiền nợ và số dư không đủ không
    if let Err(message) = &result {
        assert!(message.contains("Insufficient balance: "));
        assert!(message.contains("debt amount: "));
    }
}

// Bài kiểm tra đơn vị cho phương thức check_items
#[test]
fn test_check_items() {
    // Tạo một công việc kiểm tra các mặt hàng trong giỏ hàng
    let job = Job {
        work: Work::PayCart,
        data: "apple:2:0.5,banana:3:0.4,orange:1:0.6".to_string(),
    };
    // Gọi phương thức check_items và kiểm tra kết quả
    let result = Worker::check_items(job);
    // Kiểm tra xem kết quả có phải là một thông báo thành công với tổng số lượng và giá trị của các mặt hàng không
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        "Total items: 6, total value: 3.20".to_string()
    );
}

// Bài kiểm tra đơn vị cho phương thức charge_amount
#[test]
fn test_charge_amount() {
    // Tạo một công việc thanh toán số tiền cho các mặt hàng
    let job = Job {
        work: Work::PayCart,
        data: "3.20".to_string(),
    };
    // Gọi phương thức charge_amount và kiểm tra kết quả
    let result = Worker::charge_amount(job);
    // Kiểm tra xem kết quả có phải là một thông báo thành công hoặc một lỗi không
    assert!(result.is_ok() || result.is_err());
    // Nếu là một thông báo thành công, kiểm tra xem có chứa số tiền đã thanh toán không
    if let Ok(message) = &result {
        assert!(message.contains("Charged amount: "));
    }
    // Nếu là một lỗi, kiểm tra xem có chứa số tiền chưa thanh toán không
    if let Err(message) = &result {
        assert!(message.contains("Failed to charge amount: "));
    }
}

// Bài kiểm tra đơn vị cho phương thức confirm_order
#[test]
fn test_confirm_order() {
    // Tạo một công việc xác nhận đơn hàng
    let job = Job {
        work: Work::PlaceOrder,
        data: "ABC123".to_string(),
    };
    // Gọi phương thức confirm_order và kiểm tra kết quả
    let result = Worker::confirm_order(job);
    // Kiểm tra xem kết quả có phải là một thông báo thành công với mã đơn hàng không
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Order ABC123 confirmed".to_string());
}

// Bài kiểm tra đơn vị cho phương thức send_order
#[test]
fn test_send_order() {
    // Tạo một công việc gửi đơn hàng cho nhà cung cấp
    let job = Job {
        work: Work::PlaceOrder,
        data: "ABC123".to_string(),
    };
    // Gọi phương thức send_order và kiểm tra kết quả
    let result = Worker::send_order(job);
    // Kiểm tra xem kết quả có phải là một thông báo thành công hoặc một lỗi không
    assert!(result.is_ok() || result.is_err());
    // Nếu là một thông báo thành công, kiểm tra xem có chứa mã đơn hàng không
    if let Ok(message) = &result {
        assert!(message.contains("Order ABC123 sent to supplier"));
    }
    // Nếu là một lỗi, kiểm tra xem có chứa mã đơn hàng không
    if let Err(message) = &result {
        assert!(message.contains("Failed to send order ABC123"));
    }
}

// Bài kiểm tra tích hợp cho hệ thống thanh toán P2P song song
#[test]
fn test_system() {
    // Tạo một hệ thống mới với 4 phòng và 8 người làm việc
    let system = System::new(4, 8);
    // Tạo một danh sách các công việc mẫu
    let jobs = vec![
        Job {
            work: Work::Authentication,
            data: "Alice".to_string(),
        },
        Job {
            work: Work::Support,
            data: "Hello, I need help".to_string(),
        },
        Job {
            work: Work::Compensation,
            data: "100.0".to_string(),
        },
        Job {
            work: Work::PayDebt,
            data: "100.0".to_string(),
        },
        Job {
            work: Work::PayCart,
            data: "apple:2:0.5,banana:3:0.4,orange:1:0.6".to_string(),
        },
        Job {
            work: Work::PlaceOrder,
            data: "ABC123".to_string(),
        },
    ];
    // Thêm các công việc vào hệ thống
    for job in jobs {
        system.add_job(job);
    }
    // Đợi một khoảng thời gian để các người làm việc xử lý các công việc
    thread::sleep(Duration::from_secs(10));
    // Tắt hệ thống
    system.shutdown();
    // Kiểm tra xem các phòng có trạng thái như mong đợi không
    for room in system.rooms.iter() {
        // Lấy trạng thái của phòng
        let state = room.state.read().unwrap();
        // Kiểm tra xem công việc có được xử lý không
        assert!(state.job.is_none());
        // Kiểm tra xem kết quả có được cập nhật không
        assert!(state.result.is_some());
        // Kiểm tra xem lỗi có được xóa không
        assert!(state.error.is_none());
    }
}
