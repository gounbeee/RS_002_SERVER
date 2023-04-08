/// RUST USING AXUM
/// https://github.com/tokio-rs/axum/tree/main/examples/readme
/// 


/// AXUM
/// HTTPリクエストの処理、サーバLISTENINGを行うためのライブラリ
/// 
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

/// SERDE
/// JSONでの情報のやり取りを行うため、SERIALIZEとDE-SERIALIZEを行う必要があるが、そのための機能
/// 
use serde::{Deserialize, Serialize};

/// HTTP上でのIPアドレス表現のための機能
/// 
use std::net::SocketAddr;



/// #[tokip::main] マクロ 
/// このマクロは、基本的な設定を簡単に行うために存在する。
///
/// #[tokio::main]
/// async fn main() {
///     println!("Hello world");                 -----------------
/// }                                                            |
///                                                              |
/// ------------------------------------------                   |
/// #[tokio::main] を使用しない場合、下のようになる。                　|
///                                                              |
///                                                              |
/// fn main() {                                                  |
///     tokio::runtime::Builder::new_multi_thread()              |
///         .enable_all()                                        |
///         .build()                                             |
///         .unwrap()                                            |
///         .block_on(async {                                    |
///             println!("Hello world");                 <<<------
///         })
/// }
/// 
/// つまり、マクロは、もちろん設定次第ではあろうが、追加機能の実行を行うものである。
///                                         ~~~~~~~~~~~~~~~~~~~~
/// 
/// 
#[tokio::main]
async fn main() {

    // tracing を初期化
    // tracing はDEBUG用文字列を表示させるために使う
    tracing_subscriber::fmt::init();

    // ROUTERの設定
    let app = Router::new()

        // GET リクエストで、パスが　/の場合、こちらを。 
        .route("/", get(root))

        // POST リクエストで、パスが/users　なら、create_user関数を実行。
        .route("/users", post(create_user));


    // IPアドレスを設定
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // tracing でDEBUG文字列を表示
    tracing::debug!("listening on {}", addr);
    
    // サーバの起動
    // AXUMパッケージのServerモジュールは、Hyperの機能を流用しているようだ。
    // 
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();



}




// -----------------------------------------------
// CONTROLLER 部分
//
// 本サンプルでは、ただの文字をレスポンスしているだけなので、
// 特にVIEWに当たる部分はない。


// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}




async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {


    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };



    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))



}


// -----------------------------------------------
// MODEL 部分


// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

