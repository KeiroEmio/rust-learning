// 1. 递归数据结构：最适合使用 Box<T>
struct ListNode {
    value: i32,
    // 这里必须使用 Box，因为递归类型需要在编译时知道大小
    next: Option<Box<ListNode>>
}

// 2. 普通引用：适合临时借用数据
struct DataView<'a> {
    // 使用引用，因为只是临时查看数据
    data: &'a [i32],
}

// 3. 共享所有权：使用 Rc<T>
use std::rc::Rc;

struct SharedConfig {
    // 多个组件需要共享这个配置
    settings: Rc<String>,
}

// 4. 线程间共享：使用 Arc<T>
use std::sync::Arc;
use std::sync::Mutex;

struct ThreadSafeCache {
    // 线程间共享且需要可修改
    data: Arc<Mutex<Vec<i32>>>,
}

fn main() {
    // 1. Box 示例：适合需要在堆上分配的大型数据
    let list = Box::new(ListNode {
        value: 1,
        next: Some(Box::new(ListNode {
            value: 2,
            next: None
        }))
    });

    // 2. 引用示例：适合临时借用
    let numbers = vec![1, 2, 3];
    let view = DataView { data: &numbers };

    // 3. Rc 示例：适合多所有权场景
    let config = Rc::new(String::from("配置数据"));
    let shared1 = SharedConfig { settings: Rc::clone(&config) };
    let shared2 = SharedConfig { settings: Rc::clone(&config) };

    // 4. Arc 示例：适合线程间共享
    let cache = Arc::new(Mutex::new(vec![1, 2, 3]));
    let cache_clone = Arc::clone(&cache);

    std::thread::spawn(move || {
        let mut data = cache_clone.lock().unwrap();
        data.push(4);
    });
}