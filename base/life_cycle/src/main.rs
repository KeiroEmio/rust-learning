// 一个缓存结构的例子
#[allow(dead_code)]
#[derive(Debug)]
struct Cache<'a> {
    data: &'a str,
    processed: Vec<&'a str>,
}

// 错误示例 - 生命周期不匹配
/*
fn process_data() -> Cache<'static> {
    let data = String::from("临时数据");  // 临时数据
    let cache = Cache {
        data: &data,           // 错误：data 的生命周期太短
        processed: Vec::new(),
    };
    cache  // 返回后 data 已被释放，导致悬垂引用
}
*/

// 错误示例 - 引用逃逸
/*
fn create_cache() -> &'static str {
    let data = String::from("临时数据");
    &data  // 错误：返回局部变量的引用
}
*/

// 正确示例 - 生命周期参数的正确使用
impl<'a> Cache<'a> {
    // 创建新的缓存
    fn new(input: &'a str) -> Cache<'a> {
        Cache {
            data: input,
            processed: Vec::new(),
        }
    }

    // 处理数据并存储结果
    fn process_chunk(&mut self, chunk: &'a str) {
        self.processed.push(chunk);
    }

    // 获取所有处理过的数据
    fn get_processed(&self) -> &Vec<&'a str> {
        &self.processed
    }
}

fn main() {
    let source_data = String::from("源数据");
    
    // 创建一个作用域来演示生命周期
    {
        let mut cache = Cache::new(&source_data);
        
        // 处理一些数据
        cache.process_chunk(&source_data);
        
        // 使用处理后的数据
        let processed = cache.get_processed();
        println!("处理的数据量: {}", processed.len());
    } // cache 在这里被销毁，但 source_data 仍然有效

    // source_data 仍然可以使用
    println!("原始数据: {}", source_data);
}

// 正确示例 - 多个生命周期参数
struct MultiCache<'a, 'b> {
    primary_data: &'a str,
    secondary_data: &'b str,
}

impl<'a, 'b> MultiCache<'a, 'b> {
    fn new(primary: &'a str, secondary: &'b str) -> MultiCache<'a, 'b> {
        MultiCache {
            primary_data: primary,
            secondary_data: secondary,
        }
    }

    fn compare_data(&self) -> bool {
        self.primary_data.len() > self.secondary_data.len()
    }
}