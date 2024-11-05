// use tauri::Manager;
#![deny(warnings)]
use std::convert::Infallible;
use std::net::SocketAddr;
// use futures::FutureExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
type HttpClient = Client<hyper::client::HttpConnector>;

#[tauri::command]
pub fn close(close: String) -> String {
    println!("close window {}" , close);
    // println!("I was invoked from JavaScript! ---------------{}" , close);

    // 调用系统命令,执行睡眠
    #[cfg(target_os = "windows")]
    let output = std::process::Command::new("rundll32.exe")
        .arg("powrprof.dll,SetSuspendState")
        .output()
        .expect("failed to execute process");

    #[cfg(target_os = "linux")]
    let output = std::process::Command::new("systemctl")
        .arg("suspend")
        .output()
        .expect("failed to execute process");

    println!("Output: {}", std::str::from_utf8(&output.stdout).unwrap());

    format!("close window {}", close)
}


#[tauri::command]
pub fn open_devtools(window: tauri::Window) {
    // window.open_devtools();
    println!("open devtools {}" , window.label());

}



#[tauri::command]
pub async fn start_proxy() -> Result<String, String> {
    // 监听地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 5500));
    // 构建客户端
    let client = Client::builder()
        .http1_title_case_headers(true)
        .http1_preserve_header_case(true)
        .build_http();
    // 构建服务
    let make_service = make_service_fn(move |_| {
        // 构建客户端
        let client = client.clone();
        // 构建服务函数
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req))) }
    });
    // 服务端绑定
    let server = Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(make_service);
    // 打印监听地址
    println!("监听地址 http://{}", addr);
    // 运行服务
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    // 返回结果
    Ok("Proxy started".to_string())
}


async fn proxy(_client: HttpClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // let headers = req.headers();

    let req_url = req.uri().to_string();
    println!("请求的url-1: {}", req_url.clone());
    // 去除请求头中的host
    //请求头打印
    // println!("请求进来的请求头: {:?}", headers);
    //请求路径
    let path = req.uri().path().to_string();
    //请求方法
    if path.starts_with("") {
        print!("请求路径-2: {}", path.clone());
        // 代理出去的路径
        let target_url = "http://218.1.142.40:2324".to_owned();
        // 获取请求的响应
        let resp = get_response(_client, req, &target_url, &path).await?;
        // 打印响应
        println!("响应: {:?}", resp);
        // 返回响应给客户端
        return Ok(resp);
    }
    // 未找到路径
    let resp = Response::new(Body::from("sorry! no route found"));
    // 未找到打印
    println!("未找到路径-0: {}", path);
    Ok(resp)
}

/**
 * 获取请求的响应
 * @param client hyper::Client
 * @param req hyper::Request<hyper::Body>
 * @param target_url 请求的目标地址
 * @param path 请求的路径
 * @return hyper::Response<hyper::Body>
 */
async fn get_response(client: HttpClient, req: Request<Body>, target_url: &str, path: &str) -> Result<Response<Body>, hyper::Error> {
    let target_url = format!("{}{}", target_url, path);
    println!("请求的目标地址-3: {}", target_url);
    let headers = req.headers().clone();
    let mut request_builder = Request::builder()
        .method(req.method())
        .uri(target_url)
        .body(req.into_body())
        .unwrap();

    // 合并请求头
    *request_builder.headers_mut() = headers;
    // 发起请求
    let response =  client.request(request_builder).await?;
    // 打印响应
    println!("响应-4: {:?}", response);
    // 获取响应体
    let body = hyper::body::to_bytes(response.into_body()).await?;
    // 转换为字符串
    let body = String::from_utf8(body.to_vec()).unwrap();
    // 构造响应
    let mut resp = Response::new(Body::from(body));
    // 设置响应头
    *resp.status_mut() = http::StatusCode::OK;
    println!("响应头状态码-5: {:?}", resp.status().clone());
    Ok(resp)
}