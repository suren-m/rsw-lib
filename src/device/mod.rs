#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Platform {
    Android,
    IOS,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DeviceKind<T> {
    SmartPhone(T),
    Tablet(T),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DeviceInfo {
    kind: DeviceKind<Platform>,
    model_year: u16,
}

pub trait Device<DeviceKind> {
    fn info() -> DeviceInfo;
}
