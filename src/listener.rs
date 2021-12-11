// 写法和 js 差距很大，所以才定义这样的结构体，比如 js 中的 once 可以这样实现
/**
 * function once(eventName, listener) {
 *  const nextListener = () => {
 *      listener()
 *      this.off(eventName, nextListener)
 *   }
 *   this.on(eventName, nextListener)
 * }
 */
pub struct Listener<T> {
  // 事件名称
  pub name: String,
  // 是否只监听一次
  pub once: bool,
  // 监听器
  pub listener: fn(&T),
}
