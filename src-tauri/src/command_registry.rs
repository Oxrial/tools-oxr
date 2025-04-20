// command_registry.rs
use linkme::distributed_slice;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;
use tauri::Invoke;

// 定义命令注册项结构体
pub struct Command {
    pub name: &'static str,
    pub handler: Box<dyn Fn(Invoke) + Send + Sync>,
}

// 分布式切片收集所有命令
#[distributed_slice]
pub static COMMANDS: [Command] = [..];

lazy_static! {
    static ref COMMAND_MAP: Mutex<HashMap<String, Box<dyn Fn(Invoke) + Send + Sync>>> = Mutex::new(HashMap::new());
}

// 单行注册入口
pub fn register_command<F>(handler: F)
where
    F: Fn(Invoke) + Send + Sync + 'static,
{
    let name = std::any::type_name_of_val(&handler);
    let name = name.split("::").last().unwrap().trim_end_matches('>');
    
    COMMANDS.push(Command {
        name,
        handler: Box::new(handler),
    });
}

// 收集命令生成处理器
pub fn collect_commands() -> impl Fn(Invoke) {
    // 填充命令映射
    for cmd in COMMANDS {
        COMMAND_MAP.lock().unwrap().insert(cmd.name.to_string(), cmd.handler);
    }

    move |invoke| {
        let cmd = invoke.message.command();
        if let Some(handler) = COMMAND_MAP.lock().unwrap().get(cmd) {
            handler(invoke)
        } else {
            panic!("未注册的命令: {}", cmd);
        }
    }
}