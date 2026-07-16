pub fn enums(){
    let ipusuario :IpAddrKind = IpAddrKind::V4;
    println!("{:?}", ipusuario);
}
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}