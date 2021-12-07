use std::collections::HashMap;

use crate::Listener;

pub struct Events {
  // 监听器
  listeners: HashMap<String, Vec<Listener>>,
  // 最大的监听器个数
  max_listeners: i32,
}

impl Events {
  pub fn new() -> Self {
    Self {
      listeners: HashMap::new(),
      max_listeners: 0,
    }
  }

  pub fn get_event_names(&self) {}

  pub fn dispatch_max_listeners() {}

  // 添加监听器
  pub fn on(&mut self, event_name: String, listener: Listener) {
    let listeners = self.listeners.get(&event_name);

    match listeners {
      Some(v) => {
        let mut next_listeners = v.clone();
        next_listeners.push(listener);
        self.listeners.insert(event_name, next_listeners);
      }
      None => {
        let next_listeners = vec![listener];
        self.listeners.insert(event_name, next_listeners);
      }
    }
  }

  // 触发监听器, 暂时不支持可变参数
  pub fn emit(&self, event_name: String) {
    let listeners = self.listeners.get(&event_name);

    match listeners {
      Some(v) => {
        for val in v.iter() {
          val()
        }
      }
      None => {}
    }
  }

  // 监听后只触发一次
  pub fn once() {}

  // 移除监听器
  pub fn off(&mut self, event_name: String, listener: Listener) {
    let listeners = self.listeners.get(&event_name);

    match listeners {
      Some(v) => {
        let mut next_listeners = v.clone();
        let mut i = 0;

        while i < next_listeners.len() {
          if listener == v[i] {
            next_listeners.remove(i);
          } else {
            i += 1;
          }
        }
        self.listeners.insert(event_name, next_listeners);
      }
      None => {}
    }
  }

  // on 的别名
  pub fn add_listener(&mut self, event_name: String, listener: Listener) {
    self.on(event_name, listener);
  }

  // 获取最大监听器的个数
  pub fn get_max_listeners(&self) -> i32 {
    self.max_listeners
  }

  // 设置最大监听器的个数
  pub fn set_max_listeners(&mut self, max_listeners: i32) {
    self.max_listeners = max_listeners
  }

  // off 的别名
  pub fn remove_listener(&mut self, event_name: String, listener: Listener) {
    self.off(event_name, listener);
  }

  pub fn remove_all_listeners() {}
}
