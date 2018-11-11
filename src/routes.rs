use router::Router;
use controllers::*;

//ルーティングの生成
pub fn create_router()->Router{
    router!{
        index:   get  "/"          => GetHello::new(),
        hello:   get  "/hello"     => get_hello,
        hello:   post "/hello"     => post_hello,
        no_page: any  "/*"         => no_page
    }
}