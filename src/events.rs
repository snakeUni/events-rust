use crate::Listener;

pub struct Events<T> {
  // 监听器
  pub listeners: Vec<Listener<T>>,
  // 最大的监听器个数
  pub max_listeners: i32,
}

impl<T> Events<T> {
  pub fn new() -> Self {
    Self {
      listeners: Vec::new(),
      max_listeners: 0,
    }
  }

  pub fn get_event_names(&self) -> Vec<&str> {
    let mut names = Vec::new();
    for listener in self.listeners.iter() {
      names.push(listener.name.as_str());
    }

    names
  }

  // 添加监听器
  pub fn on(&mut self, event_name: &str, listener: fn(&T)) {
    self.listeners.push(Listener {
      name: String::from(event_name),
      once: false,
      listener,
    })
  }

  // 触发监听器, 暂时不支持可变参数
  pub fn emit(&mut self, event_name: &str, value: &T) {
    let mut remove_listeners_index = Vec::new();

    for (index, listener) in self.listeners.iter().enumerate() {
      if (listener.name.as_str() == event_name) {
        (listener.listener)(value);

        if listener.once {
          remove_listeners_index.push(index);
        }
      }
    }

    for index in remove_listeners_index.iter() {
      self.listeners.remove(*index);
    }
  }

  // 监听后只触发一次
  pub fn once(&mut self, event_name: &str, listener: fn(&T)) {
    self.listeners.push(Listener {
      name: String::from(event_name),
      once: true,
      listener,
    })
  }

  // 移除监听器
  pub fn off(&mut self, event_name: &str) {
    let mut i = 0;

    while i < self.listeners.len() {
      if self.listeners[i].name == event_name {
        self.listeners.remove(i);
      } else {
        i += 1;
      }
    }
  }

  // on 的别名
  pub fn add_listener(&mut self, event_name: &str, listener: fn(&T)) {
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
  pub fn remove_listener(&mut self, event_name: &str) {
    self.off(event_name);
  }

  pub fn remove_all_listeners(&mut self) {
    self.listeners.clear();
  }
}
