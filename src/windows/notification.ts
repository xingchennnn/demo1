import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

class Notification {
  constructor() {
    // 构造函数
  }

  public static async send(title: string, body: string): Promise<void> {
    // 你有发送通知的权限吗？
    let permissionGranted = await isPermissionGranted();

    console.log("通知权限", permissionGranted);

    // 如果没有，我们需要请求它
    if (!permissionGranted) {
      console.log("请求通知权限");
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }

    // 一旦获得许可，我们就可以发送通知
    if (permissionGranted) {
      sendNotification({ title, body });
      // this.sendNotification(body, title);
    } else {
      console.log("没有获得通知权限");
      this.sendNotification(body, title);
    }
  }

  //此方法用于发送通知
  private static sendNotification(msg: string, uid: string): void {
    const NotificationInstance = window.Notification;
    const n = new NotificationInstance("消息通知提醒", {
      body: msg, //显示通知的内容
      requireInteraction: true, //默认是false，到时间会自动关闭；如果为true，用户不点击，则会一直显示。
      // renotify: true, //默认值为 false；设为 true 将会使通知重新通知用户。
      tag: uid, //代表通知的一个识别标签，相同tag时只会打开同一个通知窗口。
      icon: "https://ts1.cn.mm.bing.net/th?id=OIP-C.zl32ej_HY_zgPWkih0k1TwAAAA&w=250&h=250&c=8&rs=1&qlt=90&o=6&pid=3.1&rm=2", //通知的图标
      data: {
        //似乎可以存放多个值
        url: "https://baidu.com",
      },
    });

    // 一下都是 通知的显示、点击、关闭的事件
    n.onshow = function () {
      console.log("通知显示了！");
    };
    n.onclick = function (e) {
      console.log("你点击了通知！", e);
      //可以直接通过实例的方式获取data内自定义的数据
      //也可以通过访问回调参数e来获取data的数据
      window.open(n.data.url, "_blank"); // 如果用户点击了通知，则直接跳转到自己的网站
      n.close();
    };
    n.onclose = function () {
      console.log("你关闭了我！！！");
    };
    n.onerror = function (err) {
      console.log("出错了，小伙子在检查一下吧");
      throw err;
    };
  }
}

export default Notification;
